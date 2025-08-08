use beerec_variants::Variants;

#[derive(Variants)]
pub enum Weekday {
    Monday,
    #[variants(rename = "DayAfterMonday")]
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
