use beerec_variants::Variants;

#[derive(Variants)]
pub enum Enum {
    Variant(usize, usize),
}

fn main() {}
