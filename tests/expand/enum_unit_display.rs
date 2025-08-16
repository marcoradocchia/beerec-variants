use beerec_variants::Variants;

#[derive(Variants)]
#[variants(display)]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
