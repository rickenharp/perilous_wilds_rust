use std::fmt::Display;

use crate::dice::Rollable;

#[derive(Debug, PartialEq)]
pub struct Alignment(pub String);

impl Alignment {
    pub fn new<T: ?Sized>(dice: &mut T) -> Alignment
    where
        T: Rollable,
    {
        match dice.roll() {
            1..=2 => Alignment(String::from("Chaotic")),
            3..=4 => Alignment(String::from("Evil")),
            5..=8 => Alignment(String::from("Neutral")),
            9..=10 => Alignment(String::from("Good")),
            11..=12 => Alignment(String::from("Lawful")),
            _ => unreachable!(),
        }
    }
}
impl Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::dice::*;

    use super::Alignment;
    // use crate::dice::Rollable;

    #[test]
    fn it_works() {
        let mut dice = MockDice::new(vec![9]);
        let alignment = Alignment::new(&mut dice);
        assert_eq!(alignment.0, String::from("Good"))
    }
}
