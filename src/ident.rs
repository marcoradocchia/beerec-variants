use syn::Ident;

use crate::string::StringExt;

/// Extension trait providing string conversion methods for [`syn::Ident`].
///
/// This trait extends the `syn::Ident` type with methods converting identifiers
/// to various string formats, including case transformation and abbreviations.
pub(crate) trait IdentExt {
    /// Converts the identifier to an uppercase string.
    fn to_uppercase_string(&self) -> String;

    /// Converts the identifier to a lowercase string.
    fn to_lowercase_string(&self) -> String;

    /// Converts the identifier to an abbreviated string.
    fn to_string_abbr(&self) -> String;

    /// Converts the identifier to an uppercase string abbreviation.
    fn to_uppercase_string_abbr(&self) -> String;

    /// Converts the identifier to an lowercase string abbreviation.
    fn to_lowercase_string_abbr(&self) -> String;
}

impl IdentExt for Ident {
    #[inline]
    fn to_uppercase_string(&self) -> String {
        self.to_string().to_uppercase_in_place()
    }

    #[inline]
    fn to_lowercase_string(&self) -> String {
        self.to_string().to_lowercase()
    }

    #[inline]
    fn to_string_abbr(&self) -> String {
        self.to_string().to_abbr_in_place()
    }

    #[inline]
    fn to_uppercase_string_abbr(&self) -> String {
        self.to_string_abbr().to_uppercase_in_place()
    }

    #[inline]
    fn to_lowercase_string_abbr(&self) -> String {
        self.to_string_abbr().to_lowercase_in_place()
    }
}
