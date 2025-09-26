use beerec_variants::Variants;

#[derive(Variants)]
#[variants(serialize)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
