use std::fmt::Display;
#[cfg(test)]
#[macro_use]
extern crate assert_matches;

mod alignment;
mod aspect;
mod creature;
mod dice;
mod element;
mod natural_feature;
mod unnatural_feature;
mod activity;

pub use alignment::Alignment;
pub use aspect::Aspect;
pub use creature::Creature;
pub use dice::Dice;
pub use element::Element;
pub use natural_feature::NaturalFeature;
pub use unnatural_feature::UnnaturalFeature;

pub enum Discovery {
    UnnaturalFeature(UnnaturalFeature),
    NaturalFeature(NaturalFeature),
    Evidence,
    Creature(Creature),
    Structure,
}

impl Display for Discovery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Discovery::UnnaturalFeature(uf) => write!(f, "Unnatural Feature: {}", uf),
            Discovery::NaturalFeature(nf) => write!(f, "Natural Feature: {}", nf),
            Discovery::Evidence => write!(f, "Evidence"),
            Discovery::Creature(c) => write!(f, "Creature: {}", c),
            Discovery::Structure => write!(f, "Structure"),
        }
    }
}

impl Discovery {
    pub fn new<T: ?Sized>(dice: &mut T) -> Discovery
    where
        T: dice::Rollable,
    {
        match dice.roll() {
            1 => Discovery::UnnaturalFeature(UnnaturalFeature::new(dice)),
            2..=4 => Discovery::NaturalFeature(NaturalFeature::new(dice)),
            5..=6 => Discovery::Evidence,
            7..=8 => Discovery::Creature(Creature::new(dice)),
            9..=12 => Discovery::Structure,
            _ => unreachable!(),
        }
    }
}
