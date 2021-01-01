use std::fmt::Display;
use rand::prelude::*;

mod arcane;
mod planar;
mod divine;
pub use crate::unnatural_feature::arcane::Arcane;
pub use crate::unnatural_feature::planar::Planar;
pub use crate::unnatural_feature::divine::Divine;

pub enum UnnaturalFeature {
    Arcane(Arcane),
    Planar(Planar),
    Divine(Divine),
}

impl UnnaturalFeature {
    pub fn new<T: ?Sized>(rng: &mut T) -> UnnaturalFeature
    where
        T: Rng,
    {
        match rng.gen_range(1..=12) {
            1..=9 => UnnaturalFeature::Arcane(Arcane::new(rng)),
            10..=11 => UnnaturalFeature::Planar(Planar::new(rng)),
            12..=12 => UnnaturalFeature::Divine(Divine::new(rng)),
            _ => unreachable!(),
        }
    }
}
impl Display for UnnaturalFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnnaturalFeature::Arcane(a) => write!(f, "Arcane: {}", a),
            UnnaturalFeature::Planar(p) => write!(f, "Planar: {}", p),
            UnnaturalFeature::Divine(d) => write!(f, "Divine {}", d),
        }
    }
}