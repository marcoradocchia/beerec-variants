use darling::FromMeta;
use darling::ast::NestedMeta;
use syn::{Lit, Meta};

use crate::nested_meta::NestedMetaSliceExt;

/// Rename strategy to be used as an inner attribute of the [`TargetVariant`]s.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum InnerRenameStrategy {
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
