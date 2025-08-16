use beerec_variants::Variants;

#[derive(Variants)]
#[variants(rename(uppercase), rename_abbr(lowercase))]
pub enum Weekday {
    Monday,
    #[variants(rename = "DayAfterMonday", rename_abbr("Tue"))]
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
