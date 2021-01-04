use std::fmt::Display;

use crate::{dice::Rollable, Alignment, Aspect};

#[derive(Debug, PartialEq)]
pub enum Divine {
    Mark {
        alignment: Alignment,
        aspect: Aspect,
    },
    CursedPlace {
        alignment: Alignment,
        aspect: Aspect,
    },
    HallowedPlace {
        alignment: Alignment,
        aspect: Aspect,
    },
    WatchedPlace {
        alignment: Alignment,
        aspect: Aspect,
    },
    Presence {
        alignment: Alignment,
        aspect: Aspect,
    },
}

impl Divine {
    pub fn new<T: ?Sized>(dice: &mut T) -> Divine
    where
        T: Rollable,
    {
        let alignment = Alignment::new(dice);
        let aspect = Aspect::new(dice);
        match dice.roll() {
            1..=3 => Divine::Mark { alignment, aspect },
            4..=6 => Divine::CursedPlace { alignment, aspect },
            7..=9 => Divine::HallowedPlace { alignment, aspect },
            10..=11 => Divine::WatchedPlace { alignment, aspect },
            12 => Divine::Presence { alignment, aspect },
            _ => unreachable!(),
        }
    }
}

impl Display for Divine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Divine::Mark { alignment, aspect } => {
                write!(f, "mark/sign\nAlignment: {}\nAspect: {}", alignment, aspect)
            }
            Divine::CursedPlace { alignment, aspect } => write!(
                f,
                "cursed place\nAlignment: {}\nAspect: {}",
                alignment, aspect
            ),
            Divine::HallowedPlace { alignment, aspect } => write!(
                f,
                "hallowed place\nAlignment: {}\nAspect: {}",
                alignment, aspect
            ),
            Divine::WatchedPlace { alignment, aspect } => write!(
                f,
                "watched place\nAlignment: {}\nAspect: {}",
                alignment, aspect
            ),
            Divine::Presence { alignment, aspect } => {
                write!(f, "presence\nAlignment: {}\nAspect: {}", alignment, aspect)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::alignment::*;
    use crate::aspect::*;
    use crate::dice::*;

    use super::Divine;

    #[test]
    fn it_works() {
        let mut dice = MockDice::new(vec![1]);
        let divine = Divine::new(&mut dice);
        assert_matches!(divine, Divine::Mark{alignment, aspect} if alignment == Alignment::Chaotic && aspect == Aspect::Power);
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
