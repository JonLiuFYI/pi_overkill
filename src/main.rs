/**
 * pi_overkill finds the value of pi the long way.
 * © Copyright 2020 JonLiuFYI
 * This file is licensed under GPL v3+.
 */

use rand::distributions::{Distribution, Uniform};
use num::integer::gcd;
/**
 * TODO:
 * implement multithreading
 * run until ctrl+c
 */

// configure this program by changing these consts
const ITERATIONS: u32 = 10_000_000;

fn main() {
    let dist = Uniform::from(1..u32::MAX);
    let mut rng = rand::thread_rng();
    let mut num_coprime = 0;

    for _ in 0..ITERATIONS {
        let x: u32 = dist.sample(&mut rng);
        let y: u32 = dist.sample(&mut rng);

        // x and y are coprime
        if gcd(x, y) == 1 {
            num_coprime += 1;
        }
    }

    // the odds of two random positive ints being coprime is 6 / pi^2
    let pi: f64 = ((6 * ITERATIONS) as f64 / num_coprime as f64).sqrt();
    println!("π = {}", pi);
}
