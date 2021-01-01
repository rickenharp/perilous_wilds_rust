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

pub struct Planar(String);

impl Planar {
    pub fn new<T: ?Sized>(rng: &mut T) -> Planar
    where
        T: Rng,
    {
        match rng.gen_range(1..=12) {
            1..=4 => Planar(String::from("distortion/warp")),
            5..=8 => Planar(String::from("portal/gate")),
            9..=10 => Planar(String::from("rift/tear")),
            11..=12 => Planar(String::from("outpost")),
            _ => unreachable!(),
        }
    }
}

impl Display for Planar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Divine(String);

impl Divine {
    pub fn new<T: ?Sized>(rng: &mut T) -> Divine
    where
        T: Rng,
    {
        match rng.gen_range(1..=12) {
            1..=3 => Divine(String::from("mark/sign")),
            4..=6 => Divine(String::from("cursed place")),
            7..=9 => Divine(String::from("hallowed place")),
            10..=11 => Divine(String::from("watched place")),
            12 => Divine(String::from("presence")),
            _ => unreachable!(),
        }
    }
}

impl Display for Divine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub enum UnnaturalFeature {
    Arcane(Arcane),
    Planar(Planar),
    Divine(Divine),
}

impl UnnaturalFeature {
    pub fn new<T: ?Sized>(mut rng: &mut T) -> UnnaturalFeature
    where
        T: Rng,
    {
        match rng.gen_range(1..=12) {
            1..=9 => UnnaturalFeature::Arcane(Arcane::new(&mut rng)),
            10..=11 => UnnaturalFeature::Planar(Planar::new(&mut rng)),
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
pub enum Discovery {
    UnnaturalFeature(UnnaturalFeature),
    NaturalFeature,
    Evidence,
    Creature,
    Structure,
}

impl Display for Discovery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Discovery::UnnaturalFeature(uf) => write!(f, "Unnatural Feature: {}", uf),
            Discovery::NaturalFeature => write!(f, "Natural Feature"),
            Discovery::Evidence => write!(f, "Evidence"),
            Discovery::Creature => write!(f, "Creature"),
            Discovery::Structure => write!(f, "Structure"),
        }
    }
}

impl Discovery {
    pub fn new<T: ?Sized>(mut rng: &mut T) -> Discovery
    where
        T: Rng,
    {
        // let mut rng = rand::thread_rng();
        match rng.gen_range(1..=12) {
            1 => Discovery::UnnaturalFeature(UnnaturalFeature::new(&mut rng)),
            2..=4 => Discovery::NaturalFeature,
            5..=6 => Discovery::Evidence,
            7..=8 => Discovery::Creature,
            9..=12 => Discovery::Structure,
            _ => unreachable!(),
        }
    }
}
