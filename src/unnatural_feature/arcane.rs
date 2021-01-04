use std::fmt::Display;

use crate::{dice::Rollable, Alignment};

#[derive(Debug, PartialEq)]
pub enum Arcane {
    Residue(Alignment),
    Blight(Alignment),
    Alteration(Alignment),
    Enchantment(Alignment),
    Source(Alignment),
}

impl Arcane {
    pub fn new<T: ?Sized>(dice: &mut T) -> Arcane
    where
        T: Rollable,
    {
        let alignment = Alignment::new(dice);
        match dice.roll() {
            1..=2 => Arcane::Residue(alignment),
            3..=5 => Arcane::Blight(alignment),
            6..=7 => Arcane::Alteration(alignment),
            8..=10 => Arcane::Enchantment(alignment),
            11..=12 => Arcane::Source(alignment),
            _ => unreachable!(),
        }
    }
}

impl Display for Arcane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arcane::Residue(alignment) => write!(f, "residue\nAlignment: {}", alignment),
            Arcane::Blight(alignment) => write!(f, "blight\nAlignment: {}", alignment),
            Arcane::Alteration(alignment) => {
                write!(f, "alteration/mutation\nAlignment: {}", alignment)
            }
            Arcane::Enchantment(alignment) => write!(f, "enchantment\nAlignment: {}", alignment),
            Arcane::Source(alignment) => write!(f, "source/repository\nAlignment: {}", alignment),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::alignment::*;
    use crate::dice::*;

    use super::Arcane;

    #[test]
    fn it_works() {
        let mut dice = MockDice::new(vec![1]);
        let arcane = Arcane::new(&mut dice);
        assert_matches!(arcane, Arcane::Residue(alignment) if alignment == Alignment::Chaotic)
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
