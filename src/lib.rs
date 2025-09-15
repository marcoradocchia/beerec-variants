mod ident;
mod nested_meta;
mod string;

use std::borrow::Cow;

use darling::ast::{Data, NestedMeta};
use darling::{FromDeriveInput, FromMeta, FromVariant};
use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{DeriveInput, Ident, Lit, Meta};

use self::ident::IdentExt;
use self::nested_meta::NestedMetaSliceExt;
use self::string::StringExt;

/// Rename strategy to be used as an outer attribute of the [`TargetEnum`].
#[derive(Debug, Clone, Copy)]
enum OuterRenameStrategy {
    /// Converts variant string representation to uppercase.
    Uppercase,
    /// Converts variant string representation to lowercase.
    Lowercase,
}

impl OuterRenameStrategy {
    /// The list of valid [`Meta::Path`]s for the [`OuterRenameStrategy`]
    /// attribute.
    const VALID_PATHS: &'static [&'static str] = &["uppercase", "lowercase"];
}

impl FromMeta for OuterRenameStrategy {
    #[rustfmt::skip]
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let nested_meta = items.get_one_exactly()?;

        match nested_meta {
            NestedMeta::Meta(Meta::Path(path)) if path.is_ident("uppercase") => Ok(Self::Uppercase),
            NestedMeta::Meta(Meta::Path(path)) if path.is_ident("lowercase") => Ok(Self::Lowercase),
            NestedMeta::Meta(Meta::Path(path)) => Err(darling::Error::unknown_field_path_with_alts(path, Self::VALID_PATHS)),
            _ => Err(darling::Error::unsupported_format("non-path")),
        }
    }
}

/// Rename strategy to be used as an inner attribute of the [`TargetVariant`]s.
#[derive(Debug, Clone)]
enum InnerRenameStrategy {
    /// Replaces variant string representation with given string literal.
    Literal(String),
    /// Converts variant string representation to uppercase.
    Uppercase,
    /// Converts variant string representation to lowercase.
    Lowercase,
}

impl InnerRenameStrategy {
    /// The list of valid [`Meta::Path`]s for the [`InnerRenameStrategy`]
    /// attribute.
    const VALID_PATHS: &'static [&'static str] = &["uppercase", "lowercase", "..."];
}

impl FromMeta for InnerRenameStrategy {
    fn from_string(value: &str) -> darling::Result<Self> {
        Ok(Self::Literal(value.to_string()))
    }

    #[rustfmt::skip]
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let nested_meta = items.get_one_exactly()?;

        match nested_meta {
            NestedMeta::Meta(meta) => match meta {
                Meta::Path(path) if path.is_ident("uppercase") => Ok(Self::Uppercase),
                Meta::Path(path) if path.is_ident("lowercase") => Ok(Self::Lowercase),
                Meta::Path(path) => Err(darling::Error::unknown_field_path_with_alts(path, Self::VALID_PATHS)),
                _ => Err(darling::Error::unsupported_format("non-path")),
            },
            NestedMeta::Lit(literal) => match literal {
                Lit::Str(lit) => Ok(Self::Literal(lit.value())),
                lit => Err(darling::Error::unexpected_lit_type(lit)),
            },
        }
    }
}

/// The type representing a [`TargetEnum`] variant.
///
/// This type is constructed while [`TargetEnum`] variants are being parsed,
/// and it's populated with information about the variant identifier and its
/// inner attributes.
#[derive(Debug, Clone, FromVariant)]
#[darling(attributes(variants))]
struct TargetVariant {
    /// The identifier of the [`TargetEnum`] variant.
    ident: Ident,
    /// The rename strategy for the variant's string representation.
    ///
    /// This field is populated by the `#[variants(rename(...))]` inner
    /// attribute of the variant.
    #[darling(default)]
    rename: Option<InnerRenameStrategy>,
    /// The rename strategy for the variant's abbreviated string representation.
    ///
    /// This field is populated by the `#[variants(rename_abbr(...))]` inner
    /// attribute of the variant.
    #[darling(default)]
    rename_abbr: Option<InnerRenameStrategy>,
    /// Whether to skip the variant during iteration.
    ///
    /// This applies to `iter_variants`, `iter_variants_as_str` and
    /// `iter_variants_as_str_abbr` generated methods.
    #[darling(default)]
    skip: bool,
}

/// Enum variant's string representation implementation.
impl TargetVariant {
    /// Returns the variant identifier, if it's not been marked as `skip`.
    ///
    /// This method provides conditional access to the identifier of the
    /// variant: returns `Some` if the variant should not be skipped,
    /// `None` otherwise.
    fn ident(&self) -> Option<&Ident> {
        (!self.skip).then_some(&self.ident)
    }

    /// Returns a string representation based on the `#[variants(rename(...))]`
    /// inner attribute strategy, if one has been specified for the variant.
    ///
    /// This method provides conditional access to the custom string
    /// representation of the variant: returns `Some` if the inner attribute has
    /// been specified for the variant, `None` otherwise.
    fn inner_rename(&self) -> Option<Cow<'_, str>> {
        self.rename.as_ref().map(|rename| match rename {
            InnerRenameStrategy::Literal(literal) => Cow::Borrowed(literal.as_str()),
            InnerRenameStrategy::Uppercase => Cow::Owned(self.ident.to_uppercase_string()),
            InnerRenameStrategy::Lowercase => Cow::Owned(self.ident.to_lowercase_string()),
        })
    }

    /// Returns a string representation based on the `#[variants(rename(...))]`
    /// outer attribute strategy (`outer_rename`), if one has been specified for
    /// the type, falling back to the variant ident's stringification otherwise.
    fn outer_rename(&self, outer_rename: Option<OuterRenameStrategy>) -> String {
        match outer_rename {
            Some(OuterRenameStrategy::Uppercase) => self.ident.to_uppercase_string(),
            Some(OuterRenameStrategy::Lowercase) => self.ident.to_lowercase_string(),
            None => self.ident.to_string(),
        }
    }

    /// Returns the final string representation of the variant.
    ///
    /// This method applies rename strategies following a priority-based
    /// fallback approach:
    ///
    /// 1. [`InnerRenameStrategy`] (_highest priority_) - returns the string
    ///    produced by the rename strategy from the `#[variants(rename(...))]`
    ///    inner attribute, if one has been specified for the variant;
    /// 1. [`OuterRenameStrategy`] (_fallback_) - returns the string produced by
    ///    the rename strategy from the `#[variants(rename(...))]` outer
    ///    attribute, if one has been specified for the type;
    /// 1. **No renaming** (_default_) - converts the variant identifier to a
    ///    string if neither the inner nor the outer rename attribute has been
    ///    specified.
    fn as_str(&self, outer_rename: Option<OuterRenameStrategy>) -> Cow<'_, str> {
        self.inner_rename().unwrap_or_else(|| {
            let outer_rename = self.outer_rename(outer_rename);
            Cow::Owned(outer_rename)
        })
    }

    /// Retuns a "_match branch_", associating the variant to the final string
    /// representation, to be used in the generation of the `as_str` method.
    fn as_str_match_branch(&self, outer_rename: Option<OuterRenameStrategy>) -> TokenStream2 {
        let Self { ident, .. } = self;
        let name = self.as_str(outer_rename);

        quote::quote! { Self::#ident => #name }
    }

    /// Returns a quoted (double-quotes) version of the final string
    /// representation of the variant.
    ///
    /// For further details about the final string representation (i.e. rename
    /// strategies, etc.) see [`TargetVariant::as_str`].
    fn as_quoted_string(&self, outer_rename: Option<OuterRenameStrategy>) -> String {
        format!("\"{}\"", self.as_str(outer_rename))
    }
}

/// Enum variant's abbreviated string representation implementation.
impl TargetVariant {
    /// Returns an abbreviated string representation by applying the
    /// [`InnerRenameStrategy::Uppercase`] renaming strategy.
    ///
    /// The renaming follows a priority-based fallback approach to determine the
    /// base string representation before applying the abbreviation:
    ///
    /// 1. [`InnerRenameStrategy`] (_highest priority_) - uses the string
    ///    produced by the rename strategy from the `#[variants(rename(...))]`
    ///    inner attribute, if one has been specified for the variant;
    /// 1. **No renaming** (_fallback_) - converts the variant identifier to a
    ///    string if the inner rename attribute hasn't been specified.
    fn inner_rename_abbr_uppercase(&self) -> String {
        self.inner_rename()
            .map(|name| name.into_owned().to_uppercase_in_place().to_abbr_in_place())
            .unwrap_or_else(|| self.ident.to_uppercase_string_abbr())
    }

    /// Returns an abbreviated string representation by applying the
    /// [`InnerRenameStrategy::Lowercase`] renaming strategy.
    ///
    /// The renaming follows a priority-based fallback approach to determine the
    /// base string representation before applying the abbreviation:
    ///
    /// 1. [`InnerRenameStrategy`] (_highest priority_) - uses the string
    ///    produced by the rename strategy from the `#[variants(rename(...))]`
    ///    inner attribute, if one has been specified for the variant;
    /// 1. **No renaming** (_fallback_) - converts the variant identifier to a
    ///    string if the inner rename attribute hasn't been specified.
    fn inner_rename_abbr_lowercase(&self) -> String {
        self.inner_rename()
            .map(|name| name.into_owned().to_lowercase_in_place().to_abbr_in_place())
            .unwrap_or_else(|| self.ident.to_lowercase_string_abbr())
    }

    /// Returns an abbreviated string representation based on the
    /// `#[variants(rename_abbr(...))]` inner attribute strategy, if one has been
    /// specified for the variant.
    ///
    /// This method provides conditional access to the custom abbreviated string
    /// representation of the variant: returns `Some` if the inner attribute has
    /// been specified for the variant, `None` otherwise.
    ///
    /// For the cases where the `#[variants(rename_abbr(...))]` inner attribute
    /// strategy is either [`InnerRenameStrategy::Uppercase`] or
    /// [`InnerRenameStrategy::Lowercase`], renaming follows a
    /// priority-based fallback approach to determine the base string
    /// representation before applying the abbreviation:
    ///
    /// 1. [`InnerRenameStrategy`] (_highest priority_) - uses the string produced
    ///    by the rename strategy from the `#[variants(rename(...))]` inner
    ///    attribute, if one has been specified for the type;
    /// 1. **No renaming** (_fallback_) - converts the variant identifier to a
    ///    string if the inner rename attribute hasn't been specified.
    #[rustfmt::skip]
    fn inner_rename_abbr(&self) -> Option<Cow<'_, str>> {
        self.rename_abbr.as_ref().map(|rename_abbr| match rename_abbr {
            InnerRenameStrategy::Literal(literal) => Cow::Borrowed(literal.as_str()),
            InnerRenameStrategy::Uppercase => Cow::Owned(self.inner_rename_abbr_uppercase()),
            InnerRenameStrategy::Lowercase => Cow::Owned(self.inner_rename_abbr_lowercase()),
        })
    }

    /// Returns an abbreviated string representation by applying the
    /// [`OuterRenameStrategy::Uppercase`] renaming strategy.
    ///
    /// The renaming follows a priority-based fallback approach to determine the
    /// base string representation before applying the abbreviation:
    ///
    /// 1. [`InnerRenameStrategy`] (_highest priority_) - uses the string
    ///    produced by the rename strategy from the `#[variants(rename(...))]`
    ///    inner attribute, if one has been specified for the variant;
    /// 1. **No renaming** (_fallback_) - converts the variant identifier to a
    ///    string if the inner rename attribute hasn't been specified.
    fn outer_rename_abbr_uppercase(&self) -> String {
        self.inner_rename()
            .map(|name| name.into_owned().to_uppercase_in_place().to_abbr_in_place())
            .unwrap_or_else(|| self.ident.to_uppercase_string_abbr())
    }

    /// Returns an abbreviated string representation applying the
    /// [`OuterRenameStrategy::Lowercase`] renaming strategy.
    ///
    /// The renaming follows a priority-based fallback approach to determine the
    /// base string representation before applying the abbreviation:
    ///
    /// 1. [`InnerRenameStrategy`] (_highest priority_) - uses the string
    ///    produced by the rename strategy from the `#[variants(rename(...))]`
    ///    inner attribute, if one has been specified for the variant;
    /// 1. **No renaming** (_fallback_) - converts the variant identifier to a
    ///    string if the inner rename attribute hasn't been specified.
    fn outer_rename_abbr_lowercase(&self) -> String {
        self.inner_rename()
            .map(|name| name.into_owned().to_lowercase_in_place().to_abbr_in_place())
            .unwrap_or_else(|| self.ident.to_lowercase_string_abbr())
    }

    /// Returns an abbreviated string representation based on the
    /// `#[variants(rename_abbr(...))]` outer attribute strategy
    /// (`outer_rename_abbr`), if one has been specified for the type, falling
    /// back to abbreviating the full length final string representation of the
    /// variant (see [`TargetVariant::as_str`] documentation for further details).
    ///
    /// The renaming follows a priority-based fallback approach to determine the
    /// base string representation before applying the abbreviation:
    ///
    /// 1. [`InnerRenameStrategy`] (_highest priority_) - uses the string produced
    ///    by the rename strategy from the `#[variants(rename(...))]` inner
    ///    attribute, if one has been specified for the variant;
    /// 1. [`OuterRenameStrategy`] (_fallback_) - uses the string produced by the
    ///    rename strategy from the `#[variants(rename(...))]` outer attribute, if
    ///    one has been specified for the type;
    /// 1. **No renaming** (_default_) - converts the variant identifier to a string
    ///    if the outer rename attribute is not specified.
    #[rustfmt::skip]
    fn outer_rename_abbr(
        &self,
        outer_rename: Option<OuterRenameStrategy>,
        outer_rename_abbr: Option<OuterRenameStrategy>,
    ) -> String {
        match outer_rename_abbr {
            Some(OuterRenameStrategy::Uppercase) => self.outer_rename_abbr_uppercase(),
            Some(OuterRenameStrategy::Lowercase) => self.outer_rename_abbr_lowercase(),
            None => self.as_str(outer_rename).into_owned().to_abbr_in_place(),
        }
    }

    /// Returns the final abbreviated string representation of the variant.
    ///
    /// This method applies rename strategies on the string representation of
    /// the variant, following a priority-based fallback approach:
    ///
    /// 1. [`InnerRenameStrategy`] (_highest priority_) - returns the
    ///    abbreviated string produced by the rename strategy from the
    ///    `#[variants(rename_abbr(...))]` inner attribute, if one has been
    ///    specified for the variant;
    /// 1. [`OuterRenameStrategy`] (_fallback_) - returns the abbreviated string
    ///    produced by the rename strategy from the
    ///    `#[variants(rename_abbr(...))]` outer attribute, if one has been
    ///    specified for the type;
    /// 1. **No renaming** (_default_) - converts the variant identifier to an
    ///    abbreviated string if neither the inner nor the outer rename
    ///    attribute has been specified.
    ///
    /// Likewise, the renaming follows a priority-based fallback approach to
    /// determine the base string representation before applying the
    /// abbreviation:
    ///
    /// 1. [`InnerRenameStrategy`] (_highest priority_) - uses the string
    ///    produced by the rename strategy from the `#[variants(rename(...))]`
    ///    inner attribute, if one has been specified for the variant;
    /// 1. [`OuterRenameStrategy`] (_fallback_) - uses the string produced by
    ///    the rename strategy from the `#[variants(rename(...))]` outer
    ///    attribute, if one has been specified for the type;
    /// 1. **No renaming** (_default_) - converts the variant identifier to a
    ///    string if neither the inner nor the outer rename attribute has been
    ///    specified.
    fn as_str_abbr(
        &self,
        outer_rename: Option<OuterRenameStrategy>,
        outer_rename_abbr: Option<OuterRenameStrategy>,
    ) -> Cow<'_, str> {
        self.inner_rename_abbr().unwrap_or_else(|| {
            let outer_rename_abbr = self.outer_rename_abbr(outer_rename, outer_rename_abbr);
            Cow::Owned(outer_rename_abbr)
        })
    }

    /// Retuns a "_match branch_", associating the variant to the final abbreviated
    /// string representation, to be used in the generation of the `as_str_abbr`
    /// method.
    #[rustfmt::skip]
    fn as_str_abbr_match_branch(
        &self,
        outer_rename: Option<OuterRenameStrategy>,
        outer_rename_abbr: Option<OuterRenameStrategy>,
    ) -> TokenStream2 {
        let Self { ident, .. } = self;
        let name_abbr = self.as_str_abbr(outer_rename, outer_rename_abbr);

        quote::quote! { Self::#ident => #name_abbr }
    }

    /// Returns a quoted (double-quotes) version of the final abbreviated string
    /// representation of the variant.
    ///
    /// For further details about the final abbreviated string representation
    /// (i.e. rename strategies, etc.) see [`TargetVariant::as_str_abbr`].
    fn as_quoted_string_abbr(
        &self,
        outer_rename: Option<OuterRenameStrategy>,
        outer_rename_abbr: Option<OuterRenameStrategy>,
    ) -> String {
        format!("\"{}\"", self.as_str_abbr(outer_rename, outer_rename_abbr))
    }
}

/// The type representing the `enum` type the macro is being derived on.
///
/// This type is constructed while the input [`TokenStream`] is being parsed,
/// and is populated with information about the `enum` identifier and its
/// variants's and outer attributes.
#[derive(Debug, Clone, FromDeriveInput)]
#[darling(supports(enum_unit), attributes(variants))]
struct TargetEnum {
    /// The identifier of the `enum` type the macro is being derived on.
    ident: Ident,
    /// The body of the `enum` type the macro is being derived on.
    ///
    /// This field represents the `enum`'s variants and allows iteration over
    /// them and their (abbreviated) string representations.
    data: Data<TargetVariant, ()>,
    /// The base rename strategy for the `enum` variants' string representation.
    ///
    /// This field represents the `#[variants(rename(...))]` outer attribute.
    #[darling(default)]
    rename: Option<OuterRenameStrategy>,
    /// The base rename strategy for the `enum` variants' abbreviated string
    /// representation.
    ///
    /// This field represents the `#[variants(rename_abbr(...))]` outer
    /// attribute.
    #[darling(default)]
    rename_abbr: Option<OuterRenameStrategy>,
    /// Whether to implement the [`Display`] trait for the `enum` type.
    ///
    /// This field represents the `#[variants(display)]` outer attribute.
    #[darling(default)]
    display: bool,
}

/// The actual derive macro implementation.
///
/// This function is introduced in conjunction with `derive_enum_variants`
/// because the `proc_macro_derive` attribute requires its target function
/// signature to be `fn(proc_macro::TokenStream) -> proc_macro::TokenStream`.
///
/// This `impl` function allows to return a `syn::Result`, enabling the `?`
/// operator for fallible function calls. This way, the macro's function
/// can map the error value to a compilation error, providing more precise
/// error messages.
#[rustfmt::skip]
#[allow(clippy::too_many_lines)]
fn derive_enum_variants_impl(input: &DeriveInput) -> syn::Result<TokenStream2> {
    let target_enum = TargetEnum::from_derive_input(input)?;

    let variants = match target_enum.data {
        Data::Enum(ref variants) => variants,
        Data::Struct(_) => unreachable!(),
    };

    let variant_count = variants.iter().filter(|variant| !variant.skip).count();
    let variant_idents = variants.iter().filter_map(TargetVariant::ident);

    let variant_as_str_match_branches = variants.iter().map(|variant| {
        variant.as_str_match_branch(target_enum.rename)
    });

    let variant_as_str_abbr_match_branches = variants.iter().map(|variant| {
        variant.as_str_abbr_match_branch(target_enum.rename, target_enum.rename_abbr)
    });

    let variants_list_str_iter = variants.iter().filter(|variant| !variant.skip).map(|variant| {
        Cow::Owned(variant.as_quoted_string(target_enum.rename))
    });

    let variants_list_str = Itertools::intersperse(
        variants_list_str_iter,
        Cow::Borrowed(", "),
    )
    .collect::<String>();
    
    let variants_list_str_abbr_iter = variants.iter().filter(|variant| !variant.skip).map(|variant| {
        Cow::Owned(variant.as_quoted_string_abbr(target_enum.rename, target_enum.rename_abbr))
    });

    let variants_list_str_abbr = Itertools::intersperse(
        variants_list_str_abbr_iter,
        Cow::Borrowed(", "),
    )
    .collect::<String>();

    let ident = &target_enum.ident;

    let iterable_variants_doc = format!("The array of iterable (i.e. non-skipped) [`{ident}`] variants.");
    let iterable_variants_count_doc = format!("The number of iterable (i.e. non-skipped) [`{ident}`] variants.");

    let as_str_doc = format!(
        r"Returns a string representation of the [`{ident}`] variant.

This method applies rename strategies following a priority-based fallback approach:

1. [`InnerRenameStrategy`] (_highest priority_) - returns the string
   produced by the rename strategy from the `#[variants(rename(...))]`
   attribute, if one has been specified for the variant;
1. [`OuterRenameStrategy`] (_fallback_) - returns the string produced by the
   rename strategy from the `#[variants(rename(...))]` attribute, if one has
   been specified for the type;
1. **No renaming** (_default_) - converts the variant identifier to a string
   if neither the type-level nor the variant-level rename attribute has been
   specified."
    );

    let as_str_abbr_doc = format!(
        r"Returns an abbreviated string representation of the [`{ident}`] variant.

This method applies rename strategies on the string representation of the
variant, following a priority-based fallback approach:

1. [`InnerRenameStrategy`] (_highest priority_) - uses the abbreviated
   string produced by the rename strategy from the
   `#[variants(rename_abbr(...))]` attribute, if one has been specified for
   the variant;
1. [`OuterRenameStrategy`] (_fallback_) - uses the string produced by the
   rename strategy from the `#[variants(rename(...))]` attribute, if one has
   been specified for the type;
1. **No renaming** (_default_) - converts the variant identifier to an
   abbreviated string if neither the type-level nor the variant-level rename
   attribute has been specified.

Likewise, the renaming follows a priority-based fallback approach to
determine the base string representation before applying the abbreviation:

1. **Variant-level attribute** (_highest priority_) - uses the string
   produced by the rename strategy from the `#[variants(rename(...))]`
   attribute, if one has been specified for the type;
1. **Type-level attribute** (_fallback_) - uses the string produced by the
   rename strategy from the `#[variants(rename(...))]` attribute, if one has
   been specified for the type;
1. **No renaming** (_default_) - converts the variant identifier to a string
   if neither the type-level nor the variant-level rename attribute has been
   specified."
    );

    let iter_variants_doc = format!(
        r"Iterates over [`{ident}`] variants.

Enum variants marked with the `#[variants(skip)]` attribute are ignored."
    );

    let iter_variants_as_str_doc = format!(
        r"Iterates over string representation of [`{ident}`] variants.

Enum variants marked with the `#[variants(skip)]` attribute are excluded from iteration.

See [`{ident}::as_str`] for further details about yielded values."
    );

    let iter_variants_as_str_abbr_doc = format!(
        r"Iterates over abbreviated string representation of [`{ident}`] variants.

Enum variants marked with the `#[variants(skip)]` attribute are excluded from iteration.

See [`{ident}::as_str_abbr`] for further details about yielded values."
    );

    let variants_list_str_doc = format!(
        r"Returns a list of quoted (double-quotes) and comma separated string
representations of the [`{ident}`] variants.

See [`{ident}::as_str`] for further details about the string representation."
    );

    let variants_list_str_abbr_doc = format!(
        r"Returns a list of quoted (double-quotes) and comma separated abbreviated string
representations of the [`{ident}`] variants.

See [`{ident}::as_str_abbr`] for further details about the abbreviated string representation."
    );

    let mut generated = quote::quote! {
        impl ::std::marker::Copy for #ident {}

        impl ::std::clone::Clone for #ident {
            fn clone(&self) -> Self {
                *self
            }
        }

        #[automatically_derived]
        impl #ident {
            #[doc = #iterable_variants_doc]
            const ITERABLE_VARIANTS: [Self; #variant_count] = [
                #(Self::#variant_idents,)*
            ];

            #[doc = #iterable_variants_count_doc]
            const ITERABLE_VARIANTS_COUNT: usize = #variant_count;

            #[inline]
            #[must_use]
            #[doc = #as_str_doc]
            pub const fn as_str(self) -> &'static str {
                match self {
                    #(#variant_as_str_match_branches,)*
                }
            }

            #[inline]
            #[must_use]
            #[doc = #as_str_abbr_doc]
            pub const fn as_str_abbr(self) -> &'static str {
                match self {
                    #(#variant_as_str_abbr_match_branches,)*
                }
            }

            #[doc = #iter_variants_doc]
            pub fn iter_variants() -> impl ::std::iter::Iterator<Item = Self> {
                Self::ITERABLE_VARIANTS.into_iter()
            }

            #[doc = #iter_variants_as_str_doc]
            pub fn iter_variants_as_str() -> impl ::std::iter::Iterator<Item = &'static str> {
                Self::iter_variants().map(Self::as_str)
            }

            #[doc = #iter_variants_as_str_abbr_doc]
            pub fn iter_variants_as_str_abbr() -> impl ::std::iter::Iterator<Item = &'static str> {
                Self::iter_variants().map(Self::as_str_abbr)
            }

            #[doc = #variants_list_str_doc]
            pub const fn variants_list_str() -> &'static str {
                #variants_list_str
            }

            #[doc = #variants_list_str_abbr_doc]
            pub const fn variants_list_str_abbr() -> &'static str {
                #variants_list_str_abbr
            }
        }
    };

    if target_enum.display {
        let generated_display_impl = quote::quote! {
            impl ::std::fmt::Display for #ident {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    f.write_str(self.as_str())
                }
            }
        };

        generated.extend(generated_display_impl);
    }

    Ok(generated)
}

/// Derive macro to generate boilerplate on unit variants `enum` types.
///
/// The macro generates the following methods:
///
/// - `as_str` - returns a string representation of the target `enum` variant;
/// - `as_str_abbr` - returns an abbreviated string representation of the target
///   `enum` variant;
/// - `iter_variants` - returns an iterator over target `enum` variants (owned
///   values);
/// - `iter_variants_as_str` - returns an iterator over string representations
///   of the target `enum` variants (`&'static str` values);
/// - `iter_variants_as_str_abbr` - returns an iterator over abbreviated string
///   representations of the `enum` variants (`&'static str` values);
/// - `variants_list_str` - returns a list of quoted (double-quotes) and comma
///   separated string representations of the `enum` variants;
/// - `variants_list_str_abbr` - returns a list of of quoted (double-quotes) and
///   comma separated abbreviated string representation of the `enum` variants.
///
/// # Enum level attributes
///
/// The macro exposes the following `enum` outer attributes (i.e. attributes to
/// be applied to the `enum` type the macro is being derived on):
///
/// - `rename` - customizes the string representation of each variant;
/// - `rename_abbr` - customizes the abbreviated string representation of each
///   variant;
/// - `display` - automatically implements the [`Display`] trait for the target
///   enum using the string representation provided by the generated `as_str`
///   method.
///
/// Valid `rename` and `rename_abbr` customization strategies are:
///
/// - `uppercase` - makes the (abbreviated) string representation uppercase;
/// - `lowercase` - makes the (abbreviated) string representation lowercase.
///
/// ## Examples
///
/// ```rust
/// # use beerec_variants::Variants;
/// #
/// #[derive(Variants)]
/// #[variants(rename(uppercase))]
/// enum CardinalDirection {
///     North,
///     East,
///     South,
///     West,
/// }
///
/// # fn main() {
/// assert_eq!("NORTH", CardinalDirection::North.as_str());
/// assert_eq!("EAST", CardinalDirection::East.as_str());
/// assert_eq!("SOUTH", CardinalDirection::South.as_str());
/// assert_eq!("WEST", CardinalDirection::West.as_str());
///
/// assert_eq!("NOR", CardinalDirection::North.as_str_abbr());
/// assert_eq!("EAS", CardinalDirection::East.as_str_abbr());
/// assert_eq!("SOU", CardinalDirection::South.as_str_abbr());
/// assert_eq!("WES", CardinalDirection::West.as_str_abbr());
/// # }
/// ```
///
/// ```rust
/// # use beerec_variants::Variants;
/// #
/// #[derive(Variants)]
/// #[variants(rename(lowercase), rename_abbr(uppercase))]
/// enum State {
///     Active,
///     Inactive,
///     Disabled,
/// }
///
/// # fn main() {
/// assert_eq!("active", State::Active.as_str());
/// assert_eq!("inactive", State::Inactive.as_str());
/// assert_eq!("disabled", State::Disabled.as_str());
///
/// assert_eq!("ACT", State::Active.as_str_abbr());
/// assert_eq!("INA", State::Inactive.as_str_abbr());
/// assert_eq!("DIS", State::Disabled.as_str_abbr());
/// # }
/// ```
///
/// ```rust
/// # use beerec_variants::Variants;
/// #
/// #[derive(Variants)]
/// #[variants(display)]
/// enum Season {
///     Spring,
///     Summer,
///     Autumn,
///     Winter,
/// }
///
/// # fn main() {
/// assert_eq!(String::from("Spring"), Season::Spring.to_string());
/// assert_eq!(String::from("Summer"), Season::Summer.to_string());
/// assert_eq!(String::from("Autumn"), Season::Autumn.to_string());
/// assert_eq!(String::from("Winter"), Season::Winter.to_string());
///
/// assert_eq!(String::from("Spring"), format!("{}", Season::Spring));
/// assert_eq!(String::from("Summer"), format!("{}", Season::Summer));
/// assert_eq!(String::from("Autumn"), format!("{}", Season::Autumn));
/// assert_eq!(String::from("Winter"), format!("{}", Season::Winter));
/// # }
/// ```
///
/// # Variant level attributes
///
/// The macro exposes the following variant attributes:
///
/// - `skip` - excludes the marked variant from iteration and listing;
/// - `rename` - customizes the string representation of the marked variant;
/// - `rename_abbr` - customizes the abbreviated string representation of the
///   marked variant.
///
/// Valid `rename` and `rename_abbr` customization strategies are:
///
/// - `"..."` (string literal) - overrides the string representation with a
///   custom string;
/// - `uppercase` - makes the (abbreviated) string representation uppercase;
/// - `lowercase` - makes the (abbreviated) string representation lowercase.
///
/// For custom string overrides:
///
/// - `#[variants(rename = "...")]` is equivalent to
///   `#[variants(rename("..."))]`;
/// - `#[variants(rename_abbr = "...")]` is equivalent to
///   `#[variants(rename_abbr("..."))]`;
///
/// both are valid, supported formats.
///
/// ## Examples
///
/// ```rust
/// # use beerec_variants::Variants;
/// #
/// #[derive(Variants)]
/// enum Format {
///     Xml,
///     Csv,
///     #[variants(rename("plain-text"), rename_abbr = "txt")]
///     PlainText,
/// }
///
/// # fn main() {
/// assert_eq!("Xml", Format::Xml.as_str());
/// assert_eq!("Csv", Format::Csv.as_str());
/// assert_eq!("plain-text", Format::PlainText.as_str());
///
/// assert_eq!("Xml", Format::Xml.as_str_abbr());
/// assert_eq!("Csv", Format::Csv.as_str_abbr());
/// assert_eq!("txt", Format::PlainText.as_str_abbr());
/// # }
/// ```
///
/// # String representation renaming priority
///
/// When using _string representations_ of enum variants, renaming can be
/// applied at both the type level and variant level. The string representation
/// of each variant is obtained by applying rename strategies following a
/// priority-based fallback approach:
///
/// 1. **Variant-level attribute** (_highest priority_) - usese the string
///    produced by the rename strategy from the `#[variants(rename(...))]`
///    attribute, if one has been specified for the variant;
/// 1. **Type-level attribute** (_fallback_) - uses the string produced by the
///    rename strategy from the `#[variants(rename(...))]` attribute, if one has
///    been specified for the type;
/// 1. **No renaming** (_default_) - converts the variant identifier to a string
///    if neither the type-level nor the variant-level rename attribute has been
///    specified.
///
/// # Abbreviated string representation renaming priority
///
/// When using _abbreviated string representation_ of the enum variants,
/// renaming can be applied at both the type level and the variant level. The
/// abbreviated string representation of each variant is obtained by applying
/// rename strategies following a priority-based fallback approach:
///
/// 1. **Variant-level attribute** (_highest priority_) - uses the abbreviated
///    string produced by the rename strategy from the
///    `#[variants(rename_abbr(...))]` attribute, if one has been specified for
///    the variant;
/// 1. **Type-level attribute** (_fallback_) - uses the string produced by the
///    rename strategy from the `#[variants(rename(...))]` attribute, if one has
///    been specified for the type;
/// 1. **No renaming** (_default_) - converts the variant identifier to an
///    abbreviated string if neither the type-level nor the variant-level rename
///    attribute has been specified.
///
/// Likewise, the renaming follows a priority-based fallback approach to
/// determine the base string representation before applying the abbreviation:
///
/// 1. **Variant-level attribute** (_highest priority_) - uses the string
///    produced by the rename strategy from the `#[variants(rename(...))]`
///    attribute, if one has been specified for the type;
/// 1. **Type-level attribute** (_fallback_) - uses the string produced by the
///    rename strategy from the `#[variants(rename(...))]` attribute, if one has
///    been specified for the type;
/// 1. **No renaming** (_default_) - converts the variant identifier to a string
///    if neither the type-level nor the variant-level rename attribute has been
///    specified.
///
/// # Errors
///
/// The macro will produce a compile error if:
///
/// - derived on `struct` types;
/// - derived on `union` types;
/// - derived on `enum` types with any named field variants;
/// - derived on `enum` types with any unnamed field (i.e. tuple) variants;
/// - derived on `enum` types with any newtype variants;
/// - the `rename` variant-level attribute is passed any other value than a
///   string literal, `uppercase` or `lowercase`;
/// - the `rename_abbr` variant-level attribute is passed any other value than a
///   string literal, `uppercase` or `lowercase`;
/// - the `rename` type-level attribute is passed any other value than
///   `uppercase` or `lowercase`;
/// - the `rename_abbr` type-level attribute is passed any other value than
///   `uppercase` or `lowercase`.
///
/// # Notes
///
/// Deriving [`Variants`] on type automatically implements [`Clone`] and
/// [`Copy`] for such type. This means that deriving either trait on a type that
/// also derives [`Variants`] will result in a "conflicting implementations"
/// compilation error.
///
/// # Examples
///
/// ```rust
/// # use beerec_variants::Variants;
/// #
/// #[derive(Variants, Debug, PartialEq, Eq)]
/// #[variants(display)]
/// enum Weekday {
///     #[variants(skip)]
///     Monday,
///     #[variants(rename = "DayAfterMonday", rename_abbr = "tue")]
///     Tuesday,
///     #[variants(rename_abbr = "wed")]
///     Wednesday,
///     #[variants(rename = "Giovedì", rename_abbr(lowercase))]
///     Thursday,
///     Friday,
///     Saturday,
///     Sunday,
/// }
///
/// # fn main() {
/// // Monday has been marked as `skip`, iterator will yield 6 values.
/// assert_eq!(6, Weekday::iter_variants().count());
///
/// assert_eq!("Monday", Weekday::Monday.as_str());
/// assert_eq!("Mon", Weekday::Monday.as_str_abbr());
///
/// // The enum has been marked as `display`, so `std::fmt::Display` implementation is available.
/// assert_eq!(String::from("Monday"), Weekday::Monday.to_string());
/// assert_eq!(String::from("Monday"), format!("{}", Weekday::Monday));
///
/// let mut weekdays = Weekday::iter_variants();
/// assert_eq!(Some(Weekday::Tuesday), weekdays.next());
/// assert_eq!(Some(Weekday::Wednesday), weekdays.next());
/// assert_eq!(Some(Weekday::Thursday), weekdays.next());
/// assert_eq!(Some(Weekday::Friday), weekdays.next());
/// assert_eq!(Some(Weekday::Saturday), weekdays.next());
/// assert_eq!(Some(Weekday::Sunday), weekdays.next());
/// assert_eq!(None, weekdays.next());
///
/// let mut weekdays_as_str = Weekday::iter_variants_as_str();
/// assert_eq!(Some("DayAfterMonday"), weekdays_as_str.next());
/// assert_eq!(Some("Wednesday"), weekdays_as_str.next());
/// assert_eq!(Some("Giovedì"), weekdays_as_str.next());
/// assert_eq!(Some("Friday"), weekdays_as_str.next());
/// assert_eq!(Some("Saturday"), weekdays_as_str.next());
/// assert_eq!(Some("Sunday"), weekdays_as_str.next());
/// assert_eq!(None, weekdays.next());
///
/// let mut weekdays_as_str_abbr = Weekday::iter_variants_as_str_abbr();
/// assert_eq!(Some("tue"), weekdays_as_str_abbr.next());
/// assert_eq!(Some("wed"), weekdays_as_str_abbr.next());
/// assert_eq!(Some("gio"), weekdays_as_str_abbr.next());
/// assert_eq!(Some("Fri"), weekdays_as_str_abbr.next());
/// assert_eq!(Some("Sat"), weekdays_as_str_abbr.next());
/// assert_eq!(Some("Sun"), weekdays_as_str_abbr.next());
/// assert_eq!(None, weekdays.next());
///
/// assert_eq!(
///     "\"DayAfterMonday\", \"Wednesday\", \"Giovedì\", \"Friday\", \"Saturday\", \"Sunday\"",
///     Weekday::variants_list_str(),
/// );
///
/// assert_eq!(
///     "\"tue\", \"wed\", \"gio\", \"Fri\", \"Sat\", \"Sun\"",
///     Weekday::variants_list_str_abbr(),
/// );
/// # }
/// ```
///
/// [`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
/// [`Copy`]: https://doc.rust-lang.org/std/marker/trait.Copy.html
/// [`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
#[proc_macro_derive(Variants, attributes(variants))]
pub fn derive_enum_variants(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    derive_enum_variants_impl(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[cfg(test)]
mod test {
    #[test]
    fn expand() {
        macrotest::expand("tests/expand/*.rs");
    }

    #[test]
    fn error() {
        let test = trybuild::TestCases::new();
        test.compile_fail("tests/fail/*.rs");
    }
}
