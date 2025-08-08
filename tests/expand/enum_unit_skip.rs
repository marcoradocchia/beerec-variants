use beerec_variants::Variants;

#[derive(Variants)]
pub enum Weekday {
    Monday,
    #[variants(skip)]
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
