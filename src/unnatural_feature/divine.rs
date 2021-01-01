use std::fmt::Display;
use rand::prelude::*;

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