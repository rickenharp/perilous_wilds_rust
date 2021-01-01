use randomizer::Discovery;
// use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let discovery = Discovery::new(&mut rng);
    println!("Discovery: {}", discovery);
}
