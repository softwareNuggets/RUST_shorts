use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

fn main() {
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Failed to get system time")
        .as_nanos() as u64;

    let mut rng = StdRng::seed_from_u64(seed);

    let dice1 = rng.gen_range(1..=6);
    let dice2 = rng.gen_range(1..=6);

    println!("Dice 1: {}", dice1);
    println!("Dice 2: {}", dice2);
}
