use std::borrow::Cow;

use darling::FromDeriveInput;
use darling::ast::Data;
use itertools::Itertools;
use proc_macro2::TokenStream;
use syn::Ident;

use crate::rename::outer::OuterRenameStrategy;
use crate::target::variant::TargetVariant;

/// The type representing the `enum` type the macro is being derived on.
///
/// This type is constructed while the input [`TokenStream`] is being parsed,
/// and is populated with information about the `enum` identifier and its
/// variants's and outer attributes.
///
/// [`TokenStream`]: ::proc_macro2::TokenStream
#[derive(Debug, Clone, FromDeriveInput)]
#[darling(supports(enum_unit), attributes(variants))]
pub(crate) struct TargetEnum {
    /// The identifier of the `enum` type the macro is being derived on.
    ident: Ident,
    /// The body of the `enum` type the macro is being derived on.
    ///
    /// This field represents the `enum`'s variants and allows iteration over
    /// them and their (abbreviated) string representations.
    data: Data<TargetVariant, ()>,
    /// The rename strategy for the string representation of `enum` variants
    /// that the macro is being derived on.
    ///
    /// This field represents the `#[variants(rename(...))]` outer attribute.
    #[darling(default)]
    rename: Option<OuterRenameStrategy>,
    /// The rename strategy for the abbreviated string representation of `enum`
    /// variants that the macro is being derived on.
    ///
    /// This field represents the `#[variants(rename_abbr(...))]` outer
    /// attribute.
    #[darling(default)]
    rename_abbr: Option<OuterRenameStrategy>,
    /// Whether to generate a [`Display`] trait implementation for the `enum`
    /// type the macro is being derived on, based on the final string
    /// representation.
    ///
    /// This field represents the `#[variants(display)]` outer attribute.
    ///
    /// [`Display`]: ::std::fmt::Display
    #[darling(default)]
    display: bool,
    /// Wether to generate a [`FromStr`] trait implementation for the `enum`
    /// type the macro is being derived on, based on the final string or
    /// abbreviated string representations.
    ///
    /// This field represents the `#[variants(from_str)]`outer attribute.
    ///
    /// [`FromStr`]: ::std::str::FromStr
    #[darling(default)]
    from_str: bool,
}

impl TargetEnum {
    /// Returns the identifier of the `enum` type the macro is being derived on.
    #[inline]
    pub(crate) fn ident(&self) -> &Ident {
        &self.ident
    }

    /// Returns variant data of the `enum` type the macro is being derived on.
    #[inline]
    pub(crate) fn variants(&self) -> &[TargetVariant] {
        match self.data {
            Data::Enum(ref variants) => variants,
            Data::Struct(_) => unreachable!(),
        }
    }

    /// Whether to generate a [`Display`] trait implementation for the `enum`
    /// type the macro is being derived on, based on the final string
    /// representation.
    ///
    /// [`Display`]: ::std::fmt::Display
    #[inline]
    pub(crate) fn implement_display(&self) -> bool {
        self.display
    }

    /// Whether to generate a [`FromStr`] trait implementation for the `enum`
    /// type the macro is being derived on, based on the final string or
    /// abbreviated string representations.
    ///
    /// [`FromStr`]: ::std::str::FromStr
    #[inline]
    pub(crate) fn implement_from_str(&self) -> bool {
        self.from_str
    }

    /// Returns an iterator over each and every variant of the `enum` type the
    /// macro is being derived on.
    #[inline]
    pub(crate) fn iter_variants(&self) -> impl Iterator<Item = &TargetVariant> {
        self.variants().iter()
    }

    /// Returns an iterator over _iterable_ (i.e. non-skipped) variants of the
    /// `enum` type the macro is being derived on.
    #[inline]
    #[rustfmt::skip]
    pub(crate) fn iter_iterable_variants(&self) -> impl Iterator<Item = &TargetVariant> {
        self.iter_variants().filter(|variant| variant.is_iterable())
    }

    /// Returns the count of _iterable_ (i.e. non-skipped) variants of the
    /// `enum` type the macro is being derived on.
    pub(crate) fn variants_count(&self) -> usize {
        self.iter_iterable_variants().count()
    }

    /// Returns an iterator over identifiers of _iterable_ (i.e. non-skipped)
    /// variants of the `enum` type the macro is being derived on.
    #[rustfmt::skip]
    pub(crate) fn iter_variant_idents(&self) -> impl Iterator<Item = &Ident> {
        self.iter_iterable_variants().filter_map(TargetVariant::ident)
    }

    /// Returns an iterator over "_match branches_", associating the variant of the
    /// `enum` type the macro is being derived on to its final string
    /// representation, to be used in the generation of the `as_str` method.
    #[rustfmt::skip]
    pub(crate) fn iter_variant_as_str_match_branches(&self) -> impl Iterator<Item = TokenStream> {
        self.iter_variants().map(|variant| variant.as_str_match_branch(self.rename))
    }

    /// Returns an iterator over "_match branches_", associating the variant of the
    /// `enum` type the macro is being derived on to its final abbreviated string
    /// representation, to be used in the generation of the `as_str_abbr` method.
    #[rustfmt::skip]
    pub(crate) fn iter_variant_as_str_abbr_match_branches(&self) -> impl Iterator<Item = TokenStream> {
        self.iter_variants().map(|variant| variant.as_str_abbr_match_branch(self.rename, self.rename_abbr))
    }

    /// Returns a list of quoted (double-quotes) and comma separated string
    /// representations of _iterable_ (i.e. non-skipped) variants of the `enum`
    /// type the macro is being derived on.
    pub(crate) fn variants_list_string(&self) -> String {
        Itertools::intersperse(
            self.iter_iterable_variants()
                .map(|variant| variant.as_quoted_string(self.rename))
                .map(Cow::Owned),
            Cow::Borrowed(", "),
        )
        .collect()
    }

    /// Returns a list of quoted (double-quotes) and comma separated abbreviated
    /// string representations of _iterable_ (i.e. non-skipped) variants of
    /// the `enum` type the macro is being derived on.
    pub(crate) fn variants_list_string_abbr(&self) -> String {
        Itertools::intersperse(
            self.iter_iterable_variants()
                .map(|variant| variant.as_quoted_string_abbr(self.rename, self.rename_abbr))
                .map(Cow::Owned),
            Cow::Borrowed(", "),
        )
        .collect()
    }

    /// Returns an iterator over "_match branches_", associating the final string
    /// and abbreviated string representations to the respective variant of the
    /// `enum` type the macro is being derived on, to be used on the generation of
    /// the `FromStr` trait implementation.
    ///
    /// [`FromStr`]: ::std::str::From
    #[rustfmt::skip]
    pub(crate) fn variants_from_str_match_branches(&self) -> impl Iterator<Item = TokenStream> {
        self.iter_iterable_variants().map(|variant| variant.from_str_match_branch(self.rename, self.rename_abbr))
    }
}
