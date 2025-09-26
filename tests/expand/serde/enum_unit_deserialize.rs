use beerec_variants::Variants;

#[derive(Variants)]
#[variants(deserialize)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
