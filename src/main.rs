mod polynomial;
mod gene;
mod animal;
mod world;

use world::World;

fn main() {
    /*
    let a = Polynomial::new();
    for i in 0..100 {
        println!("{}, {}", i, a.eval((i as f64) / 100.0));,
    }
    */
    World::new(100).next_month().next_generation();
}
