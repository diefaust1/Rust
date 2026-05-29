use std::time::Instant;

fn main() {
    const N: u64 = 10000;
    const ROUNDS: u64 = 500;
    const MOD: u64 = 1000;

    let start = Instant::now();

    let mut total: u64 = 0;

    for _ in 0..ROUNDS {
        for i in 1..=N {
            // Same math as in Python
            let value = (i * i + 3 * i + 7) % MOD;
            total = (total + value) % MOD;
        }
    }

    let duration = start.elapsed();

    println!("Result: {}", total);
    println!("Time: {:.6} seconds", duration.as_secs_f64());
}