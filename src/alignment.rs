use std::fmt::Display;

use crate::dice::Rollable;

#[derive(Debug, PartialEq)]
pub enum Alignment {
    Chaotic,
    Evil,
    Neutral,
    Good,
    Lawful,
}

impl Alignment {
    pub fn new<T: ?Sized>(dice: &mut T) -> Alignment
    where
        T: Rollable,
    {
        match dice.roll() {
            1..=2 => Alignment::Chaotic,
            3..=4 => Alignment::Evil,
            5..=8 => Alignment::Neutral,
            9..=10 => Alignment::Good,
            11..=12 => Alignment::Lawful,
            _ => unreachable!(),
        }
    }
}
impl Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Alignment::Chaotic => write!(f, "Chaotic"),
            Alignment::Evil => write!(f, "Evil"),
            Alignment::Neutral => write!(f, "Neutral"),
            Alignment::Good => write!(f, "Good"),
            Alignment::Lawful => write!(f, "Lawful"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dice::*;

    use super::Alignment;

    #[test]
    fn it_works() {
        let mut dice = MockDice::new(vec![9]);
        let alignment = Alignment::new(&mut dice);
        assert_eq!(alignment.to_string(), String::from("Good"))
    }
}
