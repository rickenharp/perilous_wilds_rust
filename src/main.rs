use perilous_wilds::Dice;
use perilous_wilds::Discovery;
// use perilous_wilds::Dice;
// use rand::prelude::*;

fn main() {
    let mut dice = Dice(12);
    let discovery = Discovery::new(&mut dice);
    println!("Discovery: {}", discovery);
}
