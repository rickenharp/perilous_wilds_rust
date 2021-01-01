use std::fmt::Display;
use rand::prelude::*;
pub struct Arcane(String);

impl Arcane {
    pub fn new<T: ?Sized>(rng: &mut T) -> Arcane
    where
        T: Rng,
    {
        match rng.gen_range(1..=12) {
            1..=2 => Arcane(String::from("residue")),
            3..=5 => Arcane(String::from("blight")),
            6..=7 => Arcane(String::from("alteration/mutation")),
            8..=10 => Arcane(String::from("enchantment")),
            11..=12 => Arcane(String::from("source/repository")),
            _ => unreachable!(),
        }
    }
}

impl Display for Arcane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
