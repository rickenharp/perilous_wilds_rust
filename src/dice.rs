use rand::prelude::*;
pub struct Dice(pub u8);

pub trait Rollable {
    fn roll(&mut self) -> u8;
}
impl Rollable for Dice {
    fn roll(&mut self) -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=self.0)
    }
}

pub struct MockDice {
    results: Vec<u8>,
    index: usize,
}

#[cfg(test)]
impl MockDice {
    pub fn new(results: Vec<u8>) -> MockDice {
        MockDice { results, index: 0 }
    }
}

impl Rollable for MockDice {
    fn roll(&mut self) -> u8 {
        if self.index >= self.results.len() {
            self.index = 0;
        }
        let result = self.results[self.index];
        self.index = self.index + 1;
        result
    }
}
#[cfg(test)]
mod tests {
    use crate::dice::*;
    // use crate::dice::Rollable;

    fn foo(dice: &mut impl Rollable) -> u8 {
        dice.roll()
    }
    #[test]
    fn it_works() {
        for _ in 1..=1000 {
            let mut dice = Dice(12);
            let result = foo(&mut dice);
            assert!(result >= 1, "Was not >=1");
            assert!(result <= 12, "Was not >=1");
        }
    }

    #[test]
    fn it_works_mocked() {
        for _ in 1..=1000 {
            let mut dice = MockDice::new(vec![1, 2, 3]);
            let result = foo(&mut dice);
            assert!(result == 1, "Was not 1");
            let result = foo(&mut dice);
            assert!(result == 2, "Was not 2");
            let result = foo(&mut dice);
            assert!(result == 3, "Was not 3");
            let result = foo(&mut dice);
            assert!(result == 1, "Was not 1");
        }
    }
    #[test]
    fn it_works_mocked_simple() {
        for _ in 1..=1000 {
            let mut dice = MockDice::new(vec![1]);
            let result = foo(&mut dice);
            assert!(result == 1, "Was not 1");
            let result = foo(&mut dice);
            assert!(result == 1, "Was not 1");
            let result = foo(&mut dice);
            assert!(result == 1, "Was not 1");
            let result = foo(&mut dice);
            assert!(result == 1, "Was not 1");
        }
    }
}
