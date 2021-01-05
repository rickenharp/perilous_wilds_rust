use std::fmt::Display;

mod arcane;
mod divine;
mod planar;
use crate::dice::Rollable;
pub use crate::unnatural_feature::arcane::Arcane;
pub use crate::unnatural_feature::divine::Divine;
pub use crate::unnatural_feature::planar::Planar;

#[derive(Debug, PartialEq)]
pub enum UnnaturalFeature {
    Arcane(Arcane),
    Planar(Planar),
    Divine(Divine),
}

impl UnnaturalFeature {
    pub fn new<T: ?Sized>(dice: &mut T) -> UnnaturalFeature
    where
        T: Rollable,
    {
        match dice.roll() {
            1..=9 => UnnaturalFeature::Arcane(Arcane::new(dice)),
            10..=11 => UnnaturalFeature::Planar(Planar::new(dice)),
            12..=12 => UnnaturalFeature::Divine(Divine::new(dice)),
            _ => unreachable!(),
        }
    }
}
impl Display for UnnaturalFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnnaturalFeature::Arcane(a) => write!(f, "Arcane: {}", a),
            UnnaturalFeature::Planar(p) => write!(f, "Planar: {}", p),
            UnnaturalFeature::Divine(d) => write!(f, "Divine {}", d),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dice::*;

    use super::UnnaturalFeature;

    #[test]
    fn test_arcane() {
        let mut dice = MockDice::new(vec![1]);
        let unnatural_feature = UnnaturalFeature::new(&mut dice);
        assert_matches!(unnatural_feature, UnnaturalFeature::Arcane(_))
    }

    #[test]
    fn test_planar() {
        let mut dice = MockDice::new(vec![10]);
        let unnatural_feature = UnnaturalFeature::new(&mut dice);
        assert_matches!(unnatural_feature, UnnaturalFeature::Planar(_))
    }

    #[test]
    fn test_divine() {
        let mut dice = MockDice::new(vec![12, 1, 1, 1, 1]);
        let unnatural_feature = UnnaturalFeature::new(&mut dice);
        assert_matches!(unnatural_feature, UnnaturalFeature::Divine(_))
    }

    #[test]
    fn test_display() {
        let mut dice = MockDice::new(vec![3, 1]);
        let unnatural_feature = UnnaturalFeature::new(&mut dice);
        assert_eq!(
            unnatural_feature.to_string(),
            String::from("Arcane: blight\nAlignment: Chaotic")
        );
    }
}
