use std::fmt::Display;

use crate::dice::Rollable;
pub struct Arcane(String);

impl Arcane {
    pub fn new<T: ?Sized>(dice: &mut T) -> Arcane
    where
        T: Rollable,
    {
        match dice.roll() {
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

#[cfg(test)]
mod tests {
    use crate::dice::*;

    use super::Arcane;
    // use crate::dice::Rollable;

    #[test]
    fn it_works() {
        let mut dice = MockDice::new(vec![1]);
        let arcane = Arcane::new(&mut dice);
        assert_eq!(arcane.0, String::from("residue"))
    }
}
