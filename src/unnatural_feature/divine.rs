use std::fmt::Display;

use crate::{dice::Rollable, Alignment, Aspect};

pub struct Divine(String, Alignment, Aspect);

impl Divine {
    pub fn new<T: ?Sized>(dice: &mut T) -> Divine
    where
        T: Rollable,
    {
        let alignment = Alignment::new(dice);
        let aspect = Aspect::new(dice);
        match dice.roll() {
            1..=3 => Divine(String::from("mark/sign"), alignment, aspect),
            4..=6 => Divine(String::from("cursed place"), alignment, aspect),
            7..=9 => Divine(String::from("hallowed place"), alignment, aspect),
            10..=11 => Divine(String::from("watched place"), alignment, aspect),
            12 => Divine(String::from("presence"), alignment, aspect),
            _ => unreachable!(),
        }
    }
}

impl Display for Divine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\nAlignment: {}\nAspect: {}", self.0, self.1, self.2)
    }
}

#[cfg(test)]
mod tests {
    use crate::dice::*;
    use crate::alignment::*;

    use super::Divine;

    #[test]
    fn it_works() {
        let mut dice = MockDice::new(vec![1]);
        let divine = Divine::new(&mut dice);
        assert_eq!(divine.0, String::from("mark/sign"));
        assert_eq!(divine.1, Alignment(String::from("Chaotic")))
    }

    #[test]
    fn test_display() {
        let mut dice = MockDice::new(vec![1]);
        let arcane = Divine::new(&mut dice);
        assert_eq!(
            arcane.to_string(),
            String::from("mark/sign\nAlignment: Chaotic\nAspect: power/strength")
        );
    }
}
