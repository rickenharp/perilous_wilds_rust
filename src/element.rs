use std::fmt::Display;

use crate::dice::Rollable;

#[derive(Debug, PartialEq)]
pub enum Element{
    Air,
    Earth,
    Fire,
    Water,
    Life,
    Death
}

impl Element {
    pub fn new<T: ?Sized>(dice: &mut T) -> Element
    where
        T: Rollable,
    {
        match dice.roll() {
            1..=2 => Element::Air,
            3..=4 => Element::Earth,
            5..=6 => Element::Fire,
            7..=8 => Element::Water,
            9..=10 => Element::Life,
            11..=12 => Element::Death,
            _ => unreachable!(),
        }
    }
}
impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Air => { write!(f, "{}", "air") }
            Element::Earth => { write!(f, "{}", "earth") }
            Element::Fire => { write!(f, "{}", "fire") }
            Element::Water => { write!(f, "{}", "water") }
            Element::Life => { write!(f, "{}", "life") }
            Element::Death => { write!(f, "{}", "death") }
        }

    }
}

#[cfg(test)]
mod tests {
    use crate::dice::*;

    use super::Element;
    // use crate::dice::Rollable;

    #[test]
    fn test_air() {
        let mut dice = MockDice::new(vec![1]);
        let element = Element::new(&mut dice);
        assert_eq!(element.to_string(), String::from("air"))
    }

    #[test]
    fn test_earth() {
        let mut dice = MockDice::new(vec![3]);
        let element = Element::new(&mut dice);
        assert_eq!(element.to_string(), String::from("earth"))
    }

    #[test]
    fn test_fire() {
        let mut dice = MockDice::new(vec![5]);
        let element = Element::new(&mut dice);
        assert_eq!(element.to_string(), String::from("fire"))
    }

    #[test]
    fn test_water() {
        let mut dice = MockDice::new(vec![7]);
        let element = Element::new(&mut dice);
        assert_eq!(element.to_string(), String::from("water"))
    }

    #[test]
    fn test_life() {
        let mut dice = MockDice::new(vec![9]);
        let element = Element::new(&mut dice);
        assert_eq!(element.to_string(), String::from("life"))
    }

    #[test]
    fn test_death() {
        let mut dice = MockDice::new(vec![11]);
        let element = Element::new(&mut dice);
        assert_eq!(element.to_string(), String::from("death"))
    }
}
