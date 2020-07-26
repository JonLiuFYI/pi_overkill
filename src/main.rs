use num::integer::gcd;
/**
 * pi_overkill finds the value of pi the long way.
 * © Copyright 2020 JonLiuFYI
 * main.rs is licensed under GPL v3+.
 */
use rand::distributions::{Distribution, Uniform};
/**
 * TODO:
 * implement multithreading
 * command line args
 */

fn main() {
    // parameters
    let iterations: u32 = 10_000_000;
    let threads = 1; // TODO

    let num_coprime = count_coprimes(iterations);

    // the probability of two random positive ints being coprime is 6 / pi^2
    // P = 6 / π²
    // π = sqrt(6 / P)
    //      since P = coprimes / total trials
    // π = sqrt((6 * total trials) / coprimes)
    let pi: f64 = ((6 * iterations * threads) as f64 / num_coprime as f64).sqrt();
    println!("π = {}", pi);
}

/**
 * Generate two random u32s for each iteration and return how many are coprime.
 * iters: u32 how many pairs of u32s to generate and check
 */
fn count_coprimes(iters: u32) -> u32 {
    let dist = Uniform::from(1..u32::MAX);
    let mut rng = rand::thread_rng();
    let mut num_coprime = 0;

    for _ in 0..iters {
        let x: u32 = dist.sample(&mut rng);
        let y: u32 = dist.sample(&mut rng);

        // x and y are coprime
        if gcd(x, y) == 1 {
            num_coprime += 1;
        }
    }

    num_coprime
}
