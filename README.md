<div align="center">
  <h1 align="center">beerec-variants</h1>

  <div align="center">
    [crates.io](https://crates.io/crates/beerec-variants) | [docs.rs](https://docs.rs/beerec-variants)
  </div>

  ![GitHub source size](https://img.shields.io/github/languages/code-size/marcoradocchia/beerec-variants?color=ea6962&logo=github&style=flat-square)
  ![GitHub open issues](https://img.shields.io/github/issues-raw/marcoradocchia/beerec-variants?color=d8a657&logo=github&style=flat-square)
  ![GitHub open pull requests](https://img.shields.io/github/issues-pr-raw/marcoradocchia/beerec-variants?color=89b482&logo=github&style=flat-square)
  ![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/marcoradocchia/beerec-variants/rust.yml?color=7daea3&logo=github&style=flat-square)
  ![GitHub sponsors](https://img.shields.io/github/sponsors/marcoradocchia?color=d3869b&logo=github&style=flat-square)
  ![Crates.io Total Downloads](https://img.shields.io/crates/d/beerec-variants?color=89b482&logo=rust&style=flat-square)
  ![Crates.io Version](https://img.shields.io/crates/v/beerec-variants?color=d3869b&logo=rust&style=flat-square)
  ![GitHub license](https://img.shields.io/github/license/marcoradocchia/beerec-variants?color=e78a4e&style=flat-square)
</div>

Derive macro to generate boilerplate on unit variants `enum` types.

The macro generates the following methods:

- `as_str` - returns a string representation of the target `enum` variant;
- `as_str_abbr` - returns an abbreviated string representation of the target
  `enum` variant;
- `iter_variants` - returns an iterator over target `enum` variants (owned
  values);
- `iter_variants_as_str` - returns an iterator over string representations
  of the target `enum` variants (`&'static str` values);
- `iter_variants_as_str_abbr` - returns an iterator over abbreviated string
  representations of the `enum` variants (`&'static str` values).

# Enum level attributes

The macro exposes the following `enum` outer attributes (i.e. attributes to
be applied to the `enum` type the macro is being derived on):

- `rename` - customizes the string representation of each variant;
- `rename_abbr` - customizes the abbreviated string representation of each
  variant;
- `display` - automatically implements the [`Display`] trait for the target
  enum using the string representation provided by the generated `as_str`
  method.

Valid `rename` and `rename_abbr` customization strategies are:

- `uppercase` - makes the (abbreviated) string representation uppercase;
- `lowercase` - makes the (abbreviated) string representation lowercase.

## Examples

```rust
#[derive(Variants)]
#[variants(rename(uppercase))]
enum CardinalDirection {
    North,
    East,
    South,
    West,
}

assert_eq!("NORTH", CardinalDirection::North.as_str());
assert_eq!("EAST", CardinalDirection::East.as_str());
assert_eq!("SOUTH", CardinalDirection::South.as_str());
assert_eq!("WEST", CardinalDirection::West.as_str());

assert_eq!("NOR", CardinalDirection::North.as_str_abbr());
assert_eq!("EAS", CardinalDirection::East.as_str_abbr());
assert_eq!("SOU", CardinalDirection::South.as_str_abbr());
assert_eq!("WES", CardinalDirection::West.as_str_abbr());
```

```rust
#[derive(Variants)]
#[variants(rename(lowercase), rename_abbr(uppercase))]
enum State {
    Active,
    Inactive,
    Disabled,
}

assert_eq!("active", State::Active.as_str());
assert_eq!("inactive", State::Inactive.as_str());
assert_eq!("disabled", State::Disabled.as_str());

assert_eq!("ACT", State::Active.as_str_abbr());
assert_eq!("INA", State::Inactive.as_str_abbr());
assert_eq!("DIS", State::Disabled.as_str_abbr());
```

```rust
#[derive(Variants)]
#[variants(display)]
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

assert_eq!(String::from("Spring"), Season::Spring.to_string());
assert_eq!(String::from("Summer"), Season::Summer.to_string());
assert_eq!(String::from("Autumn"), Season::Autumn.to_string());
assert_eq!(String::from("Winter"), Season::Winter.to_string());

assert_eq!(String::from("Spring"), format!("{}", Season::Spring));
assert_eq!(String::from("Summer"), format!("{}", Season::Summer));
assert_eq!(String::from("Autumn"), format!("{}", Season::Autumn));
assert_eq!(String::from("Winter"), format!("{}", Season::Winter));
```

# Variant level attributes

The macro exposes the following variant attributes:

- `skip` - excludes the marked variant from iteration;
- `rename` - customizes the string representation of the marked variant;
- `rename_abbr` - customizes the abbreviated string representation of the
  marked variant.

Valid `rename` and `rename_abbr` customization strategies are:

- `"..."` (string literal) - overrides the string representation with a
  custom string;
- `uppercase` - makes the (abbreviated) string representation uppercase;
- `lowercase` - makes the (abbreviated) string representation lowercase.

For custom string overrides:

- `#[variants(rename = "...")]` is equivalent to `#[variants(rename("..."))]`;
- `#[variants(rename_abbr = "...")]` is equivalent to
  `#[variants(rename_abbr("..."))]`;

both are valid, supported formats.

## Examples

```rust
#[derive(Variants)]
enum Format {
    Xml,
    Csv,
    #[variants(rename("plain-text"), rename_abbr = "txt")]
    PlainText,
}

assert_eq!("Xml", Format::Xml.as_str());
assert_eq!("Csv", Format::Csv.as_str());
assert_eq!("plain-text", Format::PlainText.as_str());

assert_eq!("Xml", Format::Xml.as_str_abbr());
assert_eq!("Csv", Format::Csv.as_str_abbr());
assert_eq!("txt", Format::PlainText.as_str_abbr());
```

# String representation renaming priority

When using _string representations_ of enum variants, renaming can be
applied at both the type level and variant level. The string representation
of each variant is obtained by applying rename strategies following a
priority-based fallback approach:

1. **Variant-level attribute** (_highest priority_) - usese the string
   produced by the rename strategy from the `#[variants(rename(...))]`
   attribute, if one has been specified for the variant;
1. **Type-level attribute** (_fallback_) - uses the string produced by the
   rename strategy from the `#[variants(rename(...))]` attribute, if one has
   been specified for the type;
1. **No renaming** (_default_) - converts the variant identifier to a string
   if neither the type-level nor the variant-level rename attribute has been
   specified.

# Abbreviated string representation renaming priority

When using _abbreviated string representation_ of the enum variants,
renaming can be applied at both the type level and the variant level. The
abbreviated string representation of each variant is obtained by applying
rename strategies following a priority-based fallback approach:

1. **Variant-level attribute** (_highest priority_) - uses the abbreviated
   string produced by the rename strategy from the
   `#[variants(rename_abbr(...))]` attribute, if one has been specified for
   the variant;
1. **Type-level attribute** (_fallback_) - uses the string produced by the
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
   specified.

# Errors

The macro will produce a compile error if:

- derived on `struct` types;
- derived on `union` types;
- derived on `enum` types with any named field variants;
- derived on `enum` types with any unnamed field (i.e. tuple) variants;
- derived on `enum` types with any newtype variants;
- the `rename` variant-level attribute is passed any other value than a
  string literal, `uppercase` or `lowercase`;
- the `rename_abbr` variant-level attribute is passed any other value than a
  string literal, `uppercase` or `lowercase`;
- the `rename` type-level attribute is passed any other value than
  `uppercase` or `lowercase`;
- the `rename_abbr` type-level attribute is passed any other value than
  `uppercase` or `lowercase`.

# Notes

Deriving `Variants` on type automatically implements [`Clone`] and
[`Copy`] for such type. This means that deriving either trait on a type that
also derives `Variants` will result in a "conflicting implementations"
compilation error.

# Examples

```rust
#[derive(Variants, Debug, PartialEq, Eq)]
#[variants(display)]
enum Weekday {
    #[variants(skip)]
    Monday,
    #[variants(rename = "DayAfterMonday", rename_abbr = "tue")]
    Tuesday,
    #[variants(rename_abbr = "wed")]
    Wednesday,
    #[variants(rename = "Giovedì", rename_abbr(lowercase))]
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

// Monday has been marked as `skip`, iterator will yield 6 values.
assert_eq!(6, Weekday::iter_variants().count());

assert_eq!("Monday", Weekday::Monday.as_str());
assert_eq!("Mon", Weekday::Monday.as_str_abbr());

// The enum has been marked as `display`, so `std::fmt::Display` implementation is available.
assert_eq!(String::from("Monday"), Weekday::Monday.to_string());
assert_eq!(String::from("Monday"), format!("{}", Weekday::Monday));

let mut weekdays = Weekday::iter_variants();
assert_eq!(Some(Weekday::Tuesday), weekdays.next());
assert_eq!(Some(Weekday::Wednesday), weekdays.next());
assert_eq!(Some(Weekday::Thursday), weekdays.next());
assert_eq!(Some(Weekday::Friday), weekdays.next());
assert_eq!(Some(Weekday::Saturday), weekdays.next());
assert_eq!(Some(Weekday::Sunday), weekdays.next());
assert_eq!(None, weekdays.next());

let mut weekdays_as_str = Weekday::iter_variants_as_str();
assert_eq!(Some("DayAfterMonday"), weekdays_as_str.next());
assert_eq!(Some("Wednesday"), weekdays_as_str.next());
assert_eq!(Some("Giovedì"), weekdays_as_str.next());
assert_eq!(Some("Friday"), weekdays_as_str.next());
assert_eq!(Some("Saturday"), weekdays_as_str.next());
assert_eq!(Some("Sunday"), weekdays_as_str.next());
assert_eq!(None, weekdays.next());

let mut weekdays_as_str_abbr = Weekday::iter_variants_as_str_abbr();
assert_eq!(Some("tue"), weekdays_as_str_abbr.next());
assert_eq!(Some("wed"), weekdays_as_str_abbr.next());
assert_eq!(Some("gio"), weekdays_as_str_abbr.next());
assert_eq!(Some("Fri"), weekdays_as_str_abbr.next());
assert_eq!(Some("Sat"), weekdays_as_str_abbr.next());
assert_eq!(Some("Sun"), weekdays_as_str_abbr.next());
assert_eq!(None, weekdays.next());
```

[`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
[`Copy`]: https://doc.rust-lang.org/std/marker/trait.Copy.html
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
