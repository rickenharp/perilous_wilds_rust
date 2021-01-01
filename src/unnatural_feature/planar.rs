use std::fmt::Display;

use crate::dice::Rollable;

pub struct Planar(String);

impl Planar {
    pub fn new<T: ?Sized>(dice: &mut T) -> Planar
    where
        T: Rollable,
    {
        match dice.roll() {
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
