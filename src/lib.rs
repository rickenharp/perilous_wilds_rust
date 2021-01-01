use std::fmt::Display;

use rand::prelude::*;

mod unnatural_feature;
mod natural_feature;
pub use unnatural_feature::UnnaturalFeature;
pub use natural_feature::NaturalFeature;

pub enum Discovery {
    UnnaturalFeature(UnnaturalFeature),
    NaturalFeature(NaturalFeature),
    Evidence,
    Creature,
    Structure,
}

impl Display for Discovery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Discovery::UnnaturalFeature(uf) => write!(f, "Unnatural Feature: {}", uf),
            Discovery::NaturalFeature(nf) => write!(f, "Natural Feature: {}", nf),
            Discovery::Evidence => write!(f, "Evidence"),
            Discovery::Creature => write!(f, "Creature"),
            Discovery::Structure => write!(f, "Structure"),
        }
    }
}

impl Discovery {
    pub fn new<T: ?Sized>(rng: &mut T) -> Discovery
    where
        T: Rng,
    {
        match rng.gen_range(1..=12) {
            1 => Discovery::UnnaturalFeature(UnnaturalFeature::new(rng)),
            2..=4 => Discovery::NaturalFeature(NaturalFeature::new(rng)),
            5..=6 => Discovery::Evidence,
            7..=8 => Discovery::Creature,
            9..=12 => Discovery::Structure,
            _ => unreachable!(),
        }
    }
}
