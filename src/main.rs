use perilous_wilds::Dice;
use perilous_wilds::Discovery;

fn main() {
    let mut dice = Dice(12);
    let discovery = Discovery::new(&mut dice);
    println!("Discovery: {}", discovery);
}
