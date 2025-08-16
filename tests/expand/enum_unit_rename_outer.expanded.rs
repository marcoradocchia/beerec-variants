use beerec_variants::Variants;
#[variants(rename(uppercase), rename_abbr(lowercase))]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
impl ::std::clone::Clone for Weekday {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::marker::Copy for Weekday {}
#[automatically_derived]
impl Weekday {
    /// The array of iterable (i.e. non-skipped) enum variants.
    const ITERABLE_VARIANTS: [Self; 7usize] = [
        Self::Monday,
        Self::Tuesday,
        Self::Wednesday,
        Self::Thursday,
        Self::Friday,
        Self::Saturday,
        Self::Sunday,
    ];
    #[inline]
    #[must_use]
    /**Returns a string representation of the [`Weekday`] variant.

This method applies rename strategies following a priority-based fallback approach:

1. [`InnerRenameStrategy`] (_highest priority_) - returns the string
   produced by the rename strategy from the `#[variants(rename(...))]`
   attribute, if one has been specified for the variant;
1. [`OuterRenameStrategy`] (_fallback_) - returns the string produced by the
   rename strategy from the `#[variants(rename(...))]` attribute, if one has
   been specified for the type;
1. **No renaming** (_default_) - converts the variant identifier to a string
   if neither the type-level nor the variant-level rename attribute has been
   specified.*/
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Monday => "MONDAY",
            Self::Tuesday => "TUESDAY",
            Self::Wednesday => "WEDNESDAY",
            Self::Thursday => "THURSDAY",
            Self::Friday => "FRIDAY",
            Self::Saturday => "SATURDAY",
            Self::Sunday => "SUNDAY",
        }
    }
    #[inline]
    #[must_use]
    /**Returns an abbreviated string representation of the [`Weekday`] variant.

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
   specified.*/
    pub fn as_str_abbr(self) -> &'static str {
        match self {
            Self::Monday => "mon",
            Self::Tuesday => "tue",
            Self::Wednesday => "wed",
            Self::Thursday => "thu",
            Self::Friday => "fri",
            Self::Saturday => "sat",
            Self::Sunday => "sun",
        }
    }
    /**Iterates over enum variants.

Enum variants marked with the `#[variants(skip)]` attribute are ignored.*/
    pub fn iter_variants() -> impl ::std::iter::Iterator<Item = Self> {
        Self::ITERABLE_VARIANTS.into_iter()
    }
    /**Iterates over string representation of enum variants.

Enum variants marked with the `#[variants(skip)]` attribute are excluded from iteration.

See `Weekday::as_str` for further details about yielded values.*/
    pub fn iter_variants_as_str() -> impl ::std::iter::Iterator<Item = &'static str> {
        Self::iter_variants().map(Self::as_str)
    }
    /**Iterates over abbreviated string representation of enum variants.

Enum variants marked with the `#[variants(skip)]` attribute are excluded from iteration.

See `Weekday::as_str_abbr` for further details about yielded values.*/
    pub fn iter_variants_as_str_abbr() -> impl ::std::iter::Iterator<
        Item = &'static str,
    > {
        Self::iter_variants().map(Self::as_str_abbr)
    }
}
