use std::fmt::Display;

use crate::{dice::Rollable, Alignment};
pub struct Arcane(String, Alignment);

impl Arcane {
    pub fn new<T: ?Sized>(dice: &mut T) -> Arcane
    where
        T: Rollable,
    {
        let alignment = Alignment::new(dice);
        match dice.roll() {
            1..=2 => Arcane(String::from("residue"), alignment),
            3..=5 => Arcane(String::from("blight"), alignment),
            6..=7 => Arcane(String::from("alteration/mutation"), alignment),
            8..=10 => Arcane(String::from("enchantment"), alignment),
            11..=12 => Arcane(String::from("source/repository"), alignment),
            _ => unreachable!(),
        }
    }
}

impl Display for Arcane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\nAlignment: {}", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::dice::*;

    use super::Arcane;
    // use crate::dice::Rollable;

    #[test]
    fn it_works() {
        let mut dice = MockDice::new(vec![1]);
        let arcane = Arcane::new(&mut dice);
        assert_eq!(arcane.0, String::from("residue"));
        assert_eq!(arcane.1.to_string(), String::from("Chaotic"))
    }

    #[test]
    fn test_display() {
        let mut dice = MockDice::new(vec![1]);
        let arcane = Arcane::new(&mut dice);
        assert_eq!(
            arcane.to_string(),
            String::from("residue\nAlignment: Chaotic")
        );
    }
}
