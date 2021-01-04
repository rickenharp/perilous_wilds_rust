use std::fmt::Display;

use crate::{dice::Rollable, Alignment, Element};

#[derive(Debug, PartialEq)]
pub enum Planar {
    Distortion(Alignment, Element),
    Portal(Alignment, Element),
    Rift(Alignment, Element),
    Outpost(Alignment, Element),
}

impl Planar {
    pub fn new<T: ?Sized>(dice: &mut T) -> Planar
    where
        T: Rollable,
    {
        let alignment = Alignment::new(dice);
        let element = Element::new(dice);
        match dice.roll() {
            1..=4 => Planar::Distortion(alignment, element),
            5..=8 => Planar::Portal(alignment, element),
            9..=10 => Planar::Rift(alignment, element),
            11..=12 => Planar::Outpost(alignment, element),
            _ => unreachable!(),
        }
    }
}

impl Display for Planar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Planar::Distortion(alignment, element) => write!(
                f,
                "distortion/warp\nAlignment: {}\nElement: {}",
                alignment, element
            ),
            Planar::Portal(alignment, element) => write!(
                f,
                "portal/gate\nAlignment: {}\nElement: {}",
                alignment, element
            ),
            Planar::Rift(alignment, element) => write!(
                f,
                "rift/tear\nAlignment: {}\nElement: {}",
                alignment, element
            ),
            Planar::Outpost(alignment, element) => {
                write!(f, "outpost\nAlignment: {}\nElement: {}", alignment, element)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::alignment::*;
    use crate::dice::*;
    use crate::element::*;

    use super::Planar;

    #[test]
    fn it_works() {
        let mut dice = MockDice::new(vec![1, 11, 3]);
        let planar = Planar::new(&mut dice);
        assert_matches!(planar, Planar::Distortion(alignment, element) if alignment == Alignment::Chaotic && element == Element::Death)
    }

    #[test]
    fn test_display() {
        let mut dice = MockDice::new(vec![3, 1]);
        let arcane = Planar::new(&mut dice);
        assert_eq!(
            arcane.to_string(),
            String::from("distortion/warp\nAlignment: Evil\nElement: air")
        );
    }
}
