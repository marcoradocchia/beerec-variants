use darling::ast::NestedMeta;

/// Extension trait providing convenience methods for [`NestedMeta`] slices.
pub(crate) trait NestedMetaSliceExt {
    /// Extracts the [`NestedMeta`] from a slice if its length is exactly one.
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// 
    /// - the slice is empty (_too few items_);
    /// - the slice has more than one element (_too many items_).
    fn get_one_exactly(&self) -> darling::Result<&NestedMeta>;
}

impl NestedMetaSliceExt for &[NestedMeta] {
    fn get_one_exactly(&self) -> darling::Result<&NestedMeta> {
        match self {
            [nested_meta] => Ok(nested_meta),
            [] => Err(darling::Error::too_few_items(1)),
            _ => Err(darling::Error::too_many_items(1)),
        }
    }
}
