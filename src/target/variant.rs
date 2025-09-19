use std::borrow::Cow;

use darling::FromVariant;
use proc_macro2::TokenStream as TokenStream2;
use syn::Ident;

use crate::ident::IdentExt;
use crate::rename::inner::InnerRenameStrategy;
use crate::rename::outer::OuterRenameStrategy;
use crate::string::StringExt;

/// The type representing a [`TargetEnum`] variant.
///
/// This type is constructed while [`TargetEnum`] variants are being parsed,
/// and it's populated with information about the variant identifier and its
/// inner attributes.
#[derive(Debug, Clone, FromVariant)]
#[darling(attributes(variants))]
pub(crate) struct TargetVariant {
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

impl TargetVariant {
    /// Checks whether the variant is _iterable_, i.e. non-skipped.
    ///
    /// This method returns `true` if the variant is _iterable_,
    /// `false` if the variant has been marked as `skip`.
    #[inline]
    pub(crate) fn is_iterable(&self) -> bool {
        !self.skip
    }

    /// Returns the variant identifier, if it's not been marked as `skip`.
    ///
    /// This method provides conditional access to the identifier of the
    /// variant: returns `Some` if the variant should not be skipped,
    /// `None` otherwise.
    #[inline]
    pub(crate) fn ident(&self) -> Option<&Ident> {
        self.is_iterable().then_some(&self.ident)
    }
}

/// Enum variant's string representation implementation.
impl TargetVariant {
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
    //
    /// This method applies rename strategies following a priority-based
    /// fallback approach:
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
    fn as_str(&self, outer_rename: Option<OuterRenameStrategy>) -> Cow<'_, str> {
        self.inner_rename().unwrap_or_else(|| {
            let outer_rename = self.outer_rename(outer_rename);
            Cow::Owned(outer_rename)
        })
    }

    /// Retuns a "_match branch_", associating the variant to the final string
    /// representation, to be used in the generation of the `as_str` method.
    pub(crate) fn as_str_match_branch(
        &self,
        outer_rename: Option<OuterRenameStrategy>,
    ) -> TokenStream2 {
        let Self { ident, .. } = self;
        let name = self.as_str(outer_rename);

        quote::quote! { Self::#ident => #name }
    }

    /// Returns a quoted (double-quotes) version of the final string
    /// representation of the variant.
    ///
    /// For further details about the final string representation (i.e. rename
    /// strategies, etc.) see [`TargetVariant::as_str`].
    pub(crate) fn as_quoted_string(&self, outer_rename: Option<OuterRenameStrategy>) -> String {
        format!("\"{}\"", self.as_str(outer_rename))
    }
}

/// Enum variant's abbreviated string representation implementation.
impl TargetVariant {
    /// Returns an abbreviated string representation by applying the
    /// [`InnerRenameStrategy::Uppercase`] renaming strategy.
    ///
    /// The renaming follows a priority-based fallback approach to determine the
    /// full length string representation before applying the abbreviation:
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
    /// full length string representation before applying the abbreviation:
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
    /// `#[variants(rename_abbr(...))]` inner attribute strategy, if one has
    /// been specified for the variant.
    ///
    /// This method provides conditional access to the custom abbreviated string
    /// representation of the variant: returns `Some` if the inner attribute has
    /// been specified for the variant, `None` otherwise.
    ///
    /// For the cases where the `#[variants(rename_abbr(...))]` inner attribute
    /// strategy is either [`InnerRenameStrategy::Uppercase`] or
    /// [`InnerRenameStrategy::Lowercase`], renaming follows a priority-based
    /// fallback approach to determine the full length string representation
    /// before applying the abbreviation:
    ///
    /// 1. [`InnerRenameStrategy`] (_highest priority_) - uses the string
    ///    produced by the rename strategy from the `#[variants(rename(...))]`
    ///    inner attribute, if one has been specified for the type;
    /// 1. **No renaming** (_fallback_) - converts the variant identifier to a
    ///    string if the inner rename attribute hasn't been specified.
    fn inner_rename_abbr(&self) -> Option<Cow<'_, str>> {
        self.rename_abbr
            .as_ref()
            .map(|rename_abbr| match rename_abbr {
                InnerRenameStrategy::Literal(literal) => Cow::Borrowed(literal.as_str()),
                InnerRenameStrategy::Uppercase => Cow::Owned(self.inner_rename_abbr_uppercase()),
                InnerRenameStrategy::Lowercase => Cow::Owned(self.inner_rename_abbr_lowercase()),
            })
    }

    /// Returns an abbreviated string representation by applying the
    /// [`OuterRenameStrategy::Uppercase`] renaming strategy.
    ///
    /// The renaming follows a priority-based fallback approach to determine the
    /// full length string representation before applying the abbreviation:
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
    /// full length string representation before applying the abbreviation:
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
    /// variant as is (see [`TargetVariant::as_str`] documentation for further
    /// details).
    ///
    /// The renaming follows a priority-based fallback approach to determine the
    /// full length string representation before applying the abbreviation:
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
    /// This method applies rename strategies for the abbreviated string
    /// representation of the variant, following a priority-based fallback
    /// approach:
    ///
    /// 1. [`InnerRenameStrategy`] (_highest priority_) - uses the abbreviated
    ///    string produced by the rename strategy from the
    ///    `#[variants(rename_abbr(...))]` inner attribute, if one has been
    ///    specified for the variant;
    /// 1. [`OuterRenameStrategy`] (_fallback_) - uses the abbreviated string
    ///    produced by the rename strategy from the
    ///    `#[variants(rename_abbr(...))]` outer attribute, if one has been
    ///    specified for the type;
    /// 1. **No renaming** (_default_) - abbreviates the full length string
    ///    representation of the variant as is, without applyaing any renaming
    ///    strategy (see [`TargetVariant::as_str`]).
    ///
    /// Likewise, the renaming follows a priority-based fallback approach to
    /// determine the full length string representation before applying the
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
    pub(crate) fn as_str_abbr_match_branch(
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
    pub(crate) fn as_quoted_string_abbr(
        &self,
        outer_rename: Option<OuterRenameStrategy>,
        outer_rename_abbr: Option<OuterRenameStrategy>,
    ) -> String {
        format!("\"{}\"", self.as_str_abbr(outer_rename, outer_rename_abbr))
    }
}

/// Enum variant's [`FromStr`] related implementation.
///
/// [`FromStr`]: ::std::str::FromStr
impl TargetVariant {
    /// Returns a "_match branch_", associating the final string and abbreviated
    /// string representations to the variant, to be used in the generation of
    /// the `FromStr` trait implementation.
    #[allow(clippy::wrong_self_convention)]
    pub(crate) fn from_str_match_branch(
        &self,
        outer_rename: Option<OuterRenameStrategy>,
        outer_rename_abbr: Option<OuterRenameStrategy>,
    ) -> TokenStream2 {
        let Self { ident, .. } = self;
        let name = self.as_str(outer_rename);
        let name_abbr = self.as_str_abbr(outer_rename, outer_rename_abbr);

        quote::quote! { #name | #name_abbr => Ok(Self::#ident) }
    }
}
