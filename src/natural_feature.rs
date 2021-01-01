use std::fmt::Display;

use crate::dice::Rollable;

pub enum NaturalFeature {
    Lair(String),
    Obstacle(String),
    TerrainChange(String),
    WaterFeature(String),
    Landmark(String),
    Resource(String),
}

impl NaturalFeature {
    pub fn new<T: ?Sized>(dice: &mut T) -> NaturalFeature
    where
        T: Rollable,
    {
        match dice.roll() {
            1..=2 => NaturalFeature::Lair(String::from("Lair")),
            3..=4 => NaturalFeature::Obstacle(String::from("Obstacle")),
            5..=7 => NaturalFeature::TerrainChange(String::from("Terrain Change")),
            8..=9 => NaturalFeature::WaterFeature(String::from("Water Feature")),
            10..=11 => NaturalFeature::Landmark(String::from("Landmark")),
            12 => NaturalFeature::Resource(String::from("Resource")),
            _ => unreachable!(),
        }
    }
}
impl Display for NaturalFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NaturalFeature::Lair(_) => write!(f, "Lair"),
            NaturalFeature::Obstacle(_) => write!(f, "Obstacle"),
            NaturalFeature::TerrainChange(_) => write!(f, "Terrain Change"),
            NaturalFeature::WaterFeature(_) => write!(f, "Water Feature"),
            NaturalFeature::Landmark(_) => write!(f, "Landmark"),
            NaturalFeature::Resource(_) => write!(f, "Resource"),
        }
    }
}
