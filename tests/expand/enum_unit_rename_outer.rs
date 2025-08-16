use beerec_variants::Variants;

#[derive(Variants)]
#[variants(rename(uppercase), rename_abbr(lowercase))]
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
