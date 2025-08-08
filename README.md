<div align="center">
  <h1 align="center">beerec-variants</h1>

  ![GitHub source size](https://img.shields.io/github/languages/code-size/marcoradocchia/beerec-variants?color=%ea6962&logo=github&style=flat-square)
  ![GitHub open issues](https://img.shields.io/github/issues-raw/marcoradocchia/beerec-variants?color=%23d8a657&logo=github&style=flat-square)
  ![GitHub open pull requests](https://img.shields.io/github/issues-pr-raw/marcoradocchia/beerec-variants?color=%2389b482&logo=github&style=flat-square)
  ![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/marcoradocchia/beerec-variants/rust.yml?color=%7daea3&logo=github&style=flat-square)
  ![GitHub sponsors](https://img.shields.io/github/sponsors/marcoradocchia?color=%23d3869b&logo=github&style=flat-square)
  ![Crates.io Total Downloads](https://img.shields.io/crates/d/beerec-variants?color=%89b482&logo=rust&style=flat-square)
  ![Crates.io Version](https://img.shields.io/crates/v/beerec-variants?color=%d3869b&logo=rust&style=flat-square)
  ![GitHub license](https://img.shields.io/github/license/marcoradocchia/beerec-variants?color=%23e78a4e&style=flat-square)
</div>

Procedural derive macro to generate boilerplate on unit variants `enum` types.

The `Variants` macro generates the following methods:
- `as_str` returning a string representation of the `enum` variant;
- `as_abbr_str` returning an abbreviated string representation of the `enum` variant;
- `iter_variants` returning an iterator over owned `enum` variants;
- `iter_variants_as_str` returning an iterator over string representations of the `enum` variants;
- `iter_variants_as_abbr_str` returning an iterator over abbreviated string representations of the `enum` variants.

It also generates a `Display` implementation based on the value returned by the `as_str` method.

# Variant attributes

The `Variants` macro exposes the following variant attributes:
- `skip` to exclude the marked variant from iteration;
- `rename` to assign a custom string representation to the marked variant;
- `rename_abbr` to assign a custom abbreviated string representation to the marked variant.

# Errors

The macro will produce a compile error if:
- derived on `struct` types;
- derived on `union` types;
- derived on `enum` types with any number of named field variants;
- derived on `enum` types with any number of unnamed field (i.e. tuple) variants;
- derived on `enum` types with any number of newtype variants;
- the `rename` variant attribute is passed any other value type than a string literal;
- the `rename_abbr` variant attribute is passed any other value type than a string literal.

# Notes 
Deriving `Variants` on type automatically implements [`Clone`] and [`Copy`] for such type.
This means that deriving [`Clone`] or [`Copy`] on a type that also derives `Variants`
will result in a compilation error for conflicting implementations.

[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
[`Copy`]: https://doc.rust-lang.org/std/marker/trait.Copy.html

# Examples

```rust
use beerec_variants::Variants;
 
#[derive(Variants, PartialEq, Eq)]
enum Weekday {
    #[variants(skip)]
    Monday,
    #[variants(rename = "DayAfterMonday", rename_abbr = "tue")]
    Tuesday,
    #[variants(rename_abbr = "wed")]
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
 }

fn main() {
  assert_eq!(6, Weekday::iter_variants().count());

  assert_eq!("Monday", Weekday::Monday.as_str());
  assert_eq!("Mon", Weekday::Monday.as_abbr_str());
  assert_eq!(String::from("Monday"), Weekday::Monday.to_string());
  assert_eq!(String::from("Today is Monday"), format!("Today is {}", Weekday::Monday));

  let mut weekdays = Weekday::iter_variants();
  assert_eq!(Some(Weekday::Tuesday), weekdays.next());
  assert_eq!(Some(Weekday::Wednesday), weekdays.next());
  assert_eq!(Some(Weekday::Thursday), weekdays.next());
  assert_eq!(Some(Weekday::Friday), weekdays.next());
  assert_eq!(Some(Weekday::Saturday), weekdays.next());
  assert_eq!(Some(Weekday::Sunday), weekdays.next());
  assert_eq!(None, weekdays.next());

  let mut weekdays_as_str = Weekday::iter_variants_as_str();
  assert_eq!(Some("DayAfterMonday"), weekdays.next());
  assert_eq!(Some("Wednesday"), weekdays.next());
  assert_eq!(Some("Thursday"), weekdays.next());
  assert_eq!(Some("Friday"), weekdays.next());
  assert_eq!(Some("Saturday"), weekdays.next());
  assert_eq!(Some("Sunday"), weekdays.next());
  assert_eq!(None, weekdays.next());

  let mut weekdays_as_abbr_str = Weekday::iter_variants_as_abbr_str();
  assert_eq!(Some("tue"), weekdays.next());
  assert_eq!(Some("wed"), weekdays.next());
  assert_eq!(Some("Thu"), weekdays.next());
  assert_eq!(Some("Fri"), weekdays.next());
  assert_eq!(Some("Sat"), weekdays.next());
  assert_eq!(Some("Sun"), weekdays.next());
  assert_eq!(None, weekdays.next());
}
```
