use darling::FromMeta;
use darling::ast::NestedMeta;
use syn::Meta;

use crate::nested_meta::NestedMetaSliceExt;

/// Rename strategy to be used as an outer attribute of the [`TargetEnum`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OuterRenameStrategy {
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
