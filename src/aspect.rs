use std::fmt::Display;

use crate::aspect::Aspect::{
    Culture, Hate, Knowledge, Love, Nature, Peace, Power, Time, Trickery, Twice, War,
};
use crate::dice::Rollable;
use crate::element::Element;

#[derive(Debug, PartialEq)]
pub enum Aspect {
    Power,
    Trickery,
    Time,
    Knowledge,
    Nature,
    Culture,
    War,
    Peace,
    Hate,
    Love,
    Element(Element),
    Twice(Box<Aspect>, Box<Aspect>),
}

impl Aspect {
    pub fn new<T: ?Sized>(dice: &mut T) -> Aspect
    where
        T: Rollable,
    {
        match dice.roll() {
            1 => Aspect::Power,
            2 => Aspect::Trickery,
            3 => Aspect::Time,
            4 => Aspect::Knowledge,
            5 => Aspect::Nature,
            6 => Aspect::Culture,
            7 => Aspect::War,
            8 => Aspect::Peace,
            9 => Aspect::Hate,
            10 => Aspect::Love,
            11 => Aspect::Element(Element::new(dice)),
            12 => {
                let mut first: Aspect;
                loop {
                    first = Aspect::new(dice);
                    match first {
                        Twice(_, _) => continue,
                        _ => break,
                    }
                }
                let mut second: Aspect;
                loop {
                    second = Aspect::new(dice);
                    if second == first {
                        continue;
                    }
                    match second {
                        Twice(_, _) => continue,
                        _ => break,
                    }
                }
                Twice(Box::new(first), Box::new(second))
            }
            _ => unreachable!(),
        }
    }
}
impl Display for Aspect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Power => write!(f, "power/strength"),
            Trickery => write!(f, "trickery/dexterity"),
            Time => write!(f, "time/constitution"),
            Knowledge => write!(f, "knowledge/intelligence"),
            Nature => write!(f, "nature/wisdom"),
            Culture => write!(f, "culture/charisma"),
            War => write!(f, "war/lies/discord"),
            Peace => write!(f, "peace/truth/balance"),
            Hate => write!(f, "hate/envy"),
            Love => write!(f, "love/admiration"),
            Aspect::Element(e) => write!(f, "{}", e),
            Twice(first, second) => write!(f, "{} and {}", first, second),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dice::*;

    use super::Aspect;

    #[test]
    fn it_works() {
        let mut dice = MockDice::new(vec![9]);
        let aspect = Aspect::new(&mut dice);
        assert_eq!(aspect.to_string(), String::from("hate/envy"))
    }

    #[test]
    fn it_rolls_twice_on_12() {
        let mut dice = MockDice::new(vec![12, 1, 1, 9]);
        let aspect = Aspect::new(&mut dice);
        assert_eq!(
            aspect.to_string(),
            String::from("power/strength and hate/envy")
        )
    }

    #[test]
    fn it_rolls_element_on_11() {
        let mut dice = MockDice::new(vec![11, 7]);
        let aspect = Aspect::new(&mut dice);
        assert_eq!(aspect.to_string(), String::from("water"))
    }
}
