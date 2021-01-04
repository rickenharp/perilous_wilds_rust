use std::fmt::Display;

use crate::{dice::Rollable, Alignment};

pub struct Planar(String, Alignment);

impl Planar {
    pub fn new<T: ?Sized>(dice: &mut T) -> Planar
    where
        T: Rollable,
    {
        let alignment = Alignment::new(dice);
        match dice.roll() {
            1..=4 => Planar(String::from("distortion/warp"), alignment),
            5..=8 => Planar(String::from("portal/gate"), alignment),
            9..=10 => Planar(String::from("rift/tear"), alignment),
            11..=12 => Planar(String::from("outpost"), alignment),
            _ => unreachable!(),
        }
    }
}

impl Display for Planar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\nAlignment: {}", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::dice::*;

    use super::Planar;

    #[test]
    fn it_works() {
        let mut dice = MockDice::new(vec![3, 1]);
        let planar = Planar::new(&mut dice);
        assert_eq!(planar.0, String::from("distortion/warp"));
        assert_eq!(planar.1.to_string(), String::from("Evil"))
    }

    #[test]
    fn test_display() {
        let mut dice = MockDice::new(vec![3, 1]);
        let arcane = Planar::new(&mut dice);
        assert_eq!(
            arcane.to_string(),
            String::from("distortion/warp\nAlignment: Evil")
        );
    }
}
