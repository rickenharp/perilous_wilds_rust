use std::fmt::Display;

use rand::prelude::*;

enum UnnaturalFeature {
    Arcane,
    Planar,
    Divine
}

impl UnnaturalFeature {
    fn new() -> UnnaturalFeature {
        let mut rng = rand::thread_rng();
        match rng.gen_range(1..12) {
            1..=9 => UnnaturalFeature::Arcane,
            10..=11 => UnnaturalFeature::Planar,
            12..=12 => UnnaturalFeature::Divine,
            _ => unreachable!(),
        }
    }
}
impl Display for UnnaturalFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnnaturalFeature::Arcane => write!(f, "Unnatural Feature: Arcane"),
            UnnaturalFeature::Planar => write!(f, "Unnatural Feature: Planar"),
            UnnaturalFeature::Divine => write!(f, "Unnatural Feature: Divine")
        }
    }
}
enum Discovery {
    UnnaturalFeature(UnnaturalFeature),
    NaturalFeature,
    Evidence,
    Creature,
    Structure,
}

impl Display for Discovery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Discovery::UnnaturalFeature(uf) => write!(f, "{}", uf),
            Discovery::NaturalFeature => write!(f, "Natural Feature"),
            Discovery::Evidence => write!(f, "Evidence"),
            Discovery::Creature => write!(f, "Creature"),
            Discovery::Structure => write!(f, "Structure"),
        }
    }
}

impl Discovery {
    fn new() -> Discovery {
        let mut rng = rand::thread_rng();
        match rng.gen_range(1..12) {
            1 => Discovery::UnnaturalFeature(UnnaturalFeature::new()),
            2..=4 => Discovery::NaturalFeature,
            5..=6 => Discovery::Evidence,
            7..=8 => Discovery::Creature,
            9..=12 => Discovery::Structure,
            _ => unreachable!(),
        }
    }
}
fn main() {
    let discovery = Discovery::new();
    println!("{}", discovery);
}
