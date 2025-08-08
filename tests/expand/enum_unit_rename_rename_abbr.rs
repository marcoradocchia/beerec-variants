use beerec_variants::Variants;

#[derive(Variants)]
pub enum Weekday {
    Monday,
    #[variants(rename = "DayAfterMonday", rename_abbr = "tue")]
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
