/// Extension trait providing in-place transformation methods for the [`String`]
/// type.
///
/// This trait extends the [`String`] type provided by the standard library with
/// methods that act by taking ownership of the value, transforming it in-place
/// and returning it afterwards, allowing method chaining.
pub(crate) trait StringExt {
    /// Returns an uppercase conversion of the [`String`] by taking
    /// ownership of the value and transforming it in-place.
    fn to_uppercase_in_place(self) -> Self;

    /// Returns an lowercase conversion of the [`String`] by taking
    /// ownership of the value and transforming it in-place.
    fn to_lowercase_in_place(self) -> Self;

    /// Returns an abbreviation of the [`String`] by taking ownership
    /// of the value and transforming it in-place.
    fn to_abbr_in_place(self) -> Self;
}

impl StringExt for String {
    #[inline]
    fn to_uppercase_in_place(mut self) -> Self {
        self.make_ascii_uppercase();
        self
    }

    #[inline]
    fn to_lowercase_in_place(mut self) -> Self {
        self.make_ascii_lowercase();
        self
    }

    #[inline]
    fn to_abbr_in_place(mut self) -> Self {
        self.truncate(3);
        self
    }
}
