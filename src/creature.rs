use crate::dice::Rollable;
use std::fmt::Display;
use crate::activity::Activity;

#[derive(Debug, PartialEq)]
pub enum Creature {
    Beast(Activity),
    Human(Activity),
    Humanoid(Activity),
    Monster(Activity),
}

impl Creature {
    pub fn new<T: ?Sized>(dice: &mut T) -> Creature
    where
        T: Rollable,
    {
        let activity = Activity::new(dice);
        match dice.roll() {
            1..=4 => Creature::Beast(activity),
            5..=6 => Creature::Human(activity),
            7..=8 => Creature::Humanoid(activity),
            9..=12 => Creature::Monster(activity),
            _ => unreachable!(),
        }
    }
}

impl Display for Creature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Creature::Beast(activity) => write!(f, "Beast\nActivity: {}", activity),
            Creature::Human(activity) => write!(f, "Human\nActivity: {}", activity),
            Creature::Humanoid(activity) => write!(f, "Humanoid\nActivity: {}", activity),
            Creature::Monster(activity) => write!(f, "Monster\nActivity: {}", activity),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dice::*;

    use super::Creature;
    use crate::activity::Activity;

    #[test]
    fn test_beast() {
        let mut dice = MockDice::new(vec![1]);
        let creature = Creature::new(&mut dice);
        assert_matches!(creature, Creature::Beast(activity) if activity == Activity::LayingTrap)
    }

    #[test]
    fn test_human() {
        let mut dice = MockDice::new(vec![5]);
        let creature = Creature::new(&mut dice);
        assert_matches!(creature, Creature::Human(activity) if activity == Activity::Eating)
    }

    #[test]
    fn test_humanoid() {
        let mut dice = MockDice::new(vec![7]);
        let creature = Creature::new(&mut dice);
        assert_matches!(creature, Creature::Humanoid(activity) if activity == Activity::Traveling)
    }

    #[test]
    fn test_monster() {
        let mut dice = MockDice::new(vec![9]);
        let creature = Creature::new(&mut dice);
        assert_matches!(creature, Creature::Monster(activity) if activity == Activity::ReturningHome)
    }

    #[test]
    fn test_display() {
        let mut dice = MockDice::new(vec![3]);
        let creature = Creature::new(&mut dice);
        assert_eq!(creature.to_string(), String::from("Beast\nActivity: prowling/on patrol"));
    }
}
