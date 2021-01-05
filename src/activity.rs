use crate::dice::Rollable;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Activity {
    LayingTrap,
    Fighting,
    Prowling,
    Hunting,
    Eating,
    Crafting,
    Traveling,
    Exploring,
    ReturningHome,
    Building,
    Sleeping,
    Dying
}

impl Activity {
    pub fn new<T: ?Sized>(dice: &mut T) -> Activity
        where
            T: Rollable,
    {
        match dice.roll() {
            1 => Activity::LayingTrap,
            2 => Activity::Fighting,
            3 => Activity::Prowling,
            4 => Activity::Hunting,
            5 => Activity::Eating,
            6 => Activity::Crafting,
            7 => Activity::Traveling,
            8 => Activity::Exploring,
            9 => Activity::ReturningHome,
            10 => Activity::Building,
            11 => Activity::Sleeping,
            12 => Activity::Dying,
            _ => unreachable!(),
        }
    }
}

impl Display for Activity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Activity::LayingTrap => write!(f, "laying trap/ambush"),
            Activity::Fighting => write!(f, "fighting/at war"),
            Activity::Prowling => write!(f, "prowling/on patrol"),
            Activity::Hunting => write!(f, "hunting/foraging"),
            Activity::Eating => write!(f, "eating/resting"),
            Activity::Crafting => write!(f, "crafting/praying"),
            Activity::Traveling => write!(f, "traveling/relocating"),
            Activity::Exploring => write!(f, "exploring/lost"),
            Activity::ReturningHome => write!(f, "returning home"),
            Activity::Building => write!(f, "building/excavating"),
            Activity::Sleeping => write!(f, "sleeping"),
            Activity::Dying => write!(f, "dying"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dice::*;

    use super::Activity;

    #[test]
    fn test_laying_trap() {
        let mut dice = MockDice::new(vec![1]);
        let activity = Activity::new(&mut dice);
        assert_matches!(activity, Activity::LayingTrap)
    }

    #[test]
    fn test_display() {
        let mut dice = MockDice::new(vec![3]);
        let activity = Activity::new(&mut dice);
        assert_eq!(
            activity.to_string(),
            String::from("prowling/on patrol")
        );
    }
}