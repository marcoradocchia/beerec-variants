<div align="center">
  <h1 align="center">beerec-variants</h1>

  [![GitHub source size](https://img.shields.io/github/languages/code-size/marcoradocchia/beerec-variants?color=ea6962&logo=github&style=flat-square)](https://github.com/marcoradocchia/beerec-variants)
  [![GitHub open issues](https://img.shields.io/github/issues-raw/marcoradocchia/beerec-variants?color=d8a657&logo=github&style=flat-square)](https://github.com/marcoradocchia/beerec-variants/issues)
  [![GitHub open pull requests](https://img.shields.io/github/issues-pr-raw/marcoradocchia/beerec-variants?color=89b482&logo=github&style=flat-square)](https://github.com/marcoradocchia/beerec-variants/pulls)
  [![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/marcoradocchia/beerec-variants/rust.yml?color=7daea3&logo=github&style=flat-square)](https://github.com/marcoradocchia/beerec-variants/actions)
  [![GitHub sponsors](https://img.shields.io/github/sponsors/marcoradocchia?color=d3869b&logo=github&style=flat-square)](https://github.com/sponsors/marcoradocchia)
  [![Crates.io Total Downloads](https://img.shields.io/crates/d/beerec-variants?color=89b482&logo=rust&style=flat-square&label=crates.io%20downloads)](https://crates.io/crates/beerec-variants)
  [![Crates.io Version](https://img.shields.io/crates/v/beerec-variants?color=d3869b&logo=rust&style=flat-square&label=crates.io%20version)](https://crates.io/crates/beerec-variants)
  [![Docs.rs](https://img.shields.io/docsrs/beerec-variants?color=7c6f64&style=flat-square&logo=rust)](https://docs.rs/beerec-variants)
  [![GitHub license](https://img.shields.io/github/license/marcoradocchia/beerec-variants?color=e78a4e&style=flat-square)](https://github.com/marcoradocchia/beerec-variants/blob/main/LICENSE)
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
  representations of the `enum` variants (`&'static str` values);
- `variants_list_str` - returns a list of quoted (double-quotes) and comma
  separated string representations of the `enum` variants;
- `variants_list_str_abbr` - returns a list of of quoted (double-quotes) and
  comma separated abbreviated string representation of the `enum` variants.

# Enum level attributes

The macro exposes the following `enum` outer attributes (i.e. attributes to
be applied to the `enum` type the macro is being derived on):

- `rename` - customizes the string representation of each variant;
- `rename_abbr` - customizes the abbreviated string representation of each
  variant;
- `display` - generates a [`Display`] trait implementation based on the
  string representation provided by the generated `as_str` method;
- `from_str` - generates a [`FromStr`] trait implementation based on the
  string or abbreviated string representation provided by the generated
  `as_str` and `as_str_abbr` methods respectively.

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

```rust
#[derive(Variants)]
#[variants(from_str)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

assert_eq!(Ok(Priority::Low), FromStr::<Priority>::from_str("Low"));
assert_eq!(Ok(Priority::Medium), FromStr::<Priority>::from_str("Medium"));
assert_eq!(Ok(Priority::High), FromStr::<Priority>::from_str("High"));
assert_eq!(Ok(Priority::Critical), FromStr::<Priority>::from_str("Critical"));

assert_eq!(Ok(Priority::Low), FromStr::<Priority>::from_str("Low"));
assert_eq!(Ok(Priority::Medium), FromStr::<Priority>::from_str("Med"));
assert_eq!(Ok(Priority::High), FromStr::<Priority>::from_str("Hig"));
assert_eq!(Ok(Priority::Critical), FromStr::<Priority>::from_str("Cri"));

assert_eq!(Err(ParsePriorityError), FromStr::<Priority>::from_str("invalid"));
```

## Feature-gated attributes

### Serde

The following `enum` outer attributes are exposed when the `serde` feature is
enabled:

- `deserialize` - generates a [`Deserialize`] trait implementation based on
  the string or abbreviated string representation provided by the generated
  `as_str` and `as_str_abbr` respectively;
- `serialize` - generates a [`Serialize`] trait implementation based on the
  string representation provided by the generated `as_str` method.

#### Examples

```rust
#[derive(Debug, Variants, PartialEq, Eq)]
#[variants(deserialize)]
enum Theme {
    Auto,
    Dark,
    Light,
}

#[derive(Debug, PartialEq, Eq)]
#[derive(serde::Deserialize)]
struct Config {
    theme: Theme,
}

// Deserialize from variant string representation.
assert_eq!(
    Ok(Config { theme: Theme::Auto }),
    toml::from_str::<'_, Config>("theme = \"Auto\""),
);

assert_eq!(
    Ok(Config { theme: Theme::Dark }),
    toml::from_str::<'_, Config>("theme = \"Dark\""),
);

assert_eq!(
    Ok(Config { theme: Theme::Light }),
    toml::from_str::<'_, Config>("theme = \"Light\""),
);

// Deserialize from variant abbreviated string representation.
assert_eq!(
    Ok(Config { theme: Theme::Auto }),
    toml::from_str::<'_, Config>("theme = \"Aut\""),
);

assert_eq!(
    Ok(Config { theme: Theme::Dark }),
    toml::from_str::<'_, Config>("theme = \"Dar\""),
);

assert_eq!(
    Ok(Config { theme: Theme::Light }),
    toml::from_str::<'_, Config>("theme = \"Lig\""),
);
```

```rust
#[derive(Debug, Variants, PartialEq, Eq)]
#[variants(serialize)]
enum Codec {
    H264,
    H265,
    AV1,
}

#[derive(Debug, PartialEq, Eq)]
#[derive(serde::Serialize)]
struct Config {
    codec: Codec,
}

assert_eq!(
    Ok(String::from("codec = \"H264\"\n")),
    toml::to_string(&Config { codec: Codec::H264 }),
);

assert_eq!(
    Ok(String::from("codec = \"H265\"\n")),
    toml::to_string(&Config { codec: Codec::H265 }),
);

assert_eq!(
    Ok(String::from("codec = \"AV1\"\n")),
    toml::to_string(&Config { codec: Codec::AV1 }),
);
```

# Variant level attributes

The macro exposes the following variant attributes:

- `skip` - excludes the marked variant from iteration and listing;
- `rename` - customizes the string representation of the marked variant;
- `rename_abbr` - customizes the abbreviated string representation of the
  marked variant.

Valid `rename` and `rename_abbr` customization strategies are:

- `"..."` (string literal) - overrides the string representation with a
  custom string;
- `uppercase` - makes the (abbreviated) string representation uppercase;
- `lowercase` - makes the (abbreviated) string representation lowercase.

For custom string overrides:

- `#[variants(rename = "...")]` is equivalent to
  `#[variants(rename("..."))]`;
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

To produce _string representations_ of enum variants, renaming can be
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

To produce _abbreviated string representations_ of the enum variants,
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
1. **No renaming** (_default_) - abbreviates the full length string
   representation of the variant as is, without applying any renaming
   strategy.

Likewise, the renaming follows a priority-based fallback approach to
determine the full length string representation before applying the
abbreviation:

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
#[cfg_attr(feature = "serde", variants(deserialize, serialize))]
#[variants(display, from_str)]
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

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
struct Schedule {
    weekday: Weekday,
}

// Monday has been marked as `skip`, iterator will yield 6 values.
assert_eq!(6, Weekday::iter_variants().count());

assert_eq!("Monday", Weekday::Monday.as_str());
assert_eq!("DayAfterMonday", Weekday::Tuesday.as_str());
assert_eq!("Wednesday", Weekday::Wednesday.as_str());
assert_eq!("Giovedì", Weekday::Thursday.as_str());
assert_eq!("Friday", Weekday::Friday.as_str());
assert_eq!("Saturday", Weekday::Saturday.as_str());
assert_eq!("Sunday", Weekday::Sunday.as_str());

assert_eq!("Mon", Weekday::Monday.as_str());
assert_eq!("tue", Weekday::Tuesday.as_str());
assert_eq!("wed", Weekday::Wednesday.as_str());
assert_eq!("gio", Weekday::Thursday.as_str());
assert_eq!("Fri", Weekday::Friday.as_str());
assert_eq!("Sat", Weekday::Saturday.as_str());
assert_eq!("Sun", Weekday::Sunday.as_str());

// The enum has been marked as `display`, so `std::fmt::Display` implementation is available.
assert_eq!(String::from("Monday"), Weekday::Monday.to_string());
assert_eq!(String::from("DayAfterMonday"), Weekday::Tuesday.to_string());
assert_eq!(String::from("Wednesday"), Weekday::Wednesday.to_string());
assert_eq!(String::from("Giovedì"), Weekday::Thursday.to_string());
assert_eq!(String::from("Friday"), Weekday::Friday.to_string());
assert_eq!(String::from("Saturday"), Weekday::Saturday.to_string());
assert_eq!(String::from("Sunday"), Weekday::Sunday.to_string());

assert_eq!(String::from("Monday"), format!("{}", Weekday::Monday));
assert_eq!(String::from("DayAfterMonday"), format!("{}", Weekday::Tuesday));
assert_eq!(String::from("Wednesday"), format!("{}", Weekday::Wednesday));
assert_eq!(String::from("Giovedì"), format!("{}", Weekday::Thursday));
assert_eq!(String::from("Friday"), format!("{}", Weekday::Friday));
assert_eq!(String::from("Saturday"), format!("{}", Weekday::Saturday));
assert_eq!(String::from("Sunday"), format!("{}", Weekday::Sunday));

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

assert_eq!(
    "\"DayAfterMonday\", \"Wednesday\", \"Giovedì\", \"Friday\", \"Saturday\", \"Sunday\"",
    Weekday::variants_list_str(),
);

assert_eq!(
    "\"tue\", \"wed\", \"gio\", \"Fri\", \"Sat\", \"Sun\"",
    Weekday::variants_list_str_abbr(),
);

// The enum has been marked as `from_str`, so `std::str::FromStr` implementation is available.
assert_eq!(Ok(Weekday::Monday), FromStr::<Weekday>::from_str("Monday"));
assert_eq!(Ok(Weekday::Tuesday), FromStr::<Weekday>::from_str("DayAfterMonday"));
assert_eq!(Ok(Weekday::Wednesday), FromStr::<Weekday>::from_str("Wednesday"));
assert_eq!(Ok(Weekday::Thursday), FromStr::<Weekday>::from_str("Giovedì"));
assert_eq!(Ok(Weekday::Friday), FromStr::<Weekday>::from_str("Friday"));
assert_eq!(Ok(Weekday::Saturday), FromStr::<Weekday>::from_str("Saturday"));
assert_eq!(Ok(Weekday::Sunday), FromStr::<Weekday>::from_str("Sunday"));

assert_eq!(Ok(Weekday::Monday), FromStr::<Weekday>::from_str("Mon"));
assert_eq!(Ok(Weekday::Tuesday), FromStr::<Weekday>::from_str("tue"));
assert_eq!(Ok(Weekday::Wednesday), FromStr::<Weekday>::from_str("wed"));
assert_eq!(Ok(Weekday::Thursday), FromStr::<Weekday>::from_str("gio"));
assert_eq!(Ok(Weekday::Friday), FromStr::<Weekday>::from_str("Fri"));
assert_eq!(Ok(Weekday::Saturday), FromStr::<Weekday>::from_str("Sat"));
assert_eq!(Ok(Weekday::Sunday), FromStr::<Weekday>::from_str("Sun"));

assert_eq!(Err(ParseWeekdayError), FromStr::<Weekday>::from_str("invalid"));

// The enum has been marked as `deserialize`, so `serde::Deserialize` implementation is available.
#[cfg(feature = "serde")]
{
    // Deserialize from variant string representation.
    assert_eq!(
        Ok(Schedule { weekday: Weekday::Monday }),
        toml::from_str::<'_, Schedule>("weekday = \"Monday\"\n"),
    );

    assert_eq!(
        Ok(Schedule { weekday: Weekday::Tuesday }),
        toml::from_str::<'_, Schedule>("weekday = \"DayAfterMonday\"\n"),
    );

    assert_eq!(
        Ok(Schedule { weekday: Weekday::Wednesday }),
        toml::from_str::<'_, Schedule>("weekday = \"Wednesday\"\n"),
    );

    assert_eq!(
        Ok(Schedule { weekday: Weekday::Thursday }),
        toml::from_str::<'_, Schedule>("weekday = \"Giovedì\"\n"),
    );

    assert_eq!(
        Ok(Schedule { weekday: Weekday::Friday }),
        toml::from_str::<'_, Schedule>("weekday = \"Friday\"\n"),
    );

    assert_eq!(
        Ok(Schedule { weekday: Weekday::Saturday }),
        toml::from_str::<'_, Schedule>("weekday = \"Saturday\"\n"),
    );

    assert_eq!(
        Ok(Schedule { weekday: Weekday::Sunday }),
        toml::from_str::<'_, Schedule>("weekday = \"Sunday\"\n"),
    );

    // Deserialize from variant abbreviated string representation.
    assert_eq!(
        Ok(Schedule { weekday: Weekday::Monday }),
        toml::from_str::<'_, Schedule>("weekday = \"Mon\"\n"),
    );

    assert_eq!(
        Ok(Schedule { weekday: Weekday::Tuesday }),
        toml::from_str::<'_, Schedule>("weekday = \"tue\"\n"),
    );

    assert_eq!(
        Ok(Schedule { weekday: Weekday::Wednesday }),
        toml::from_str::<'_, Schedule>("weekday = \"wed\"\n"),
    );

    assert_eq!(
        Ok(Schedule { weekday: Weekday::Thursday }),
        toml::from_str::<'_, Schedule>("weekday = \"gio\"\n"),
    );

    assert_eq!(
        Ok(Schedule { weekday: Weekday::Friday }),
        toml::from_str::<'_, Schedule>("weekday = \"Fri\"\n"),
    );

    assert_eq!(
        Ok(Schedule { weekday: Weekday::Saturday }),
        toml::from_str::<'_, Schedule>("weekday = \"Sat\"\n"),
    );

    assert_eq!(
        Ok(Schedule { weekday: Weekday::Sunday }),
        toml::from_str::<'_, Schedule>("weekday = \"Sun\"\n"),
    );
}

// The enum has been marked as `serialize`, so `serde::Serialize` implementation is available.
#[cfg(feature = "serde")]  
{
    assert_eq!(
        Ok(String::from("weekday = \"Monday\"\n")),
        toml::to_string(&Schedule { weekday: Weekday::Monday }),
    );

    assert_eq!(
        Ok(String::from("weekday = \"DayAfterMonday\"\n")),
        toml::to_string(&Schedule { weekday: Weekday::Tuesday }),
    );

    assert_eq!(
        Ok(String::from("weekday = \"Wednesday\"\n")),
        toml::to_string(&Schedule { weekday: Weekday::Wednesday }),
    );

    assert_eq!(
        Ok(String::from("weekday = \"Giovedì\"\n")),
        toml::to_string(&Schedule { weekday: Weekday::Thursday }),
    );

    assert_eq!(
        Ok(String::from("weekday = \"Friday\"\n")),
        toml::to_string(&Schedule { weekday: Weekday::Friday }),
    );

    assert_eq!(
        Ok(String::from("weekday = \"Saturday\"\n")),
        toml::to_string(&Schedule { weekday: Weekday::Saturday }),
    );

    assert_eq!(
        Ok(String::from("weekday = \"Sunday\"\n")),
        toml::to_string(&Schedule { weekday: Weekday::Sunday }),
    );
}
```

[`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
[`Copy`]: https://doc.rust-lang.org/std/marker/trait.Copy.html
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[`FromStr`]: https://doc.rust-lang.org/std/str/trait.FromStr.html
[`Deserialize`]: https://docs.rs/serde/latest/serde/trait.Deserialize.html
[`Serialize`]: https://docs.rs/serde/latest/serde/trait.Serialize.html
