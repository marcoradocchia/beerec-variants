use beerec_variants::Variants;

#[derive(Variants)]
#[variants(from_str)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
