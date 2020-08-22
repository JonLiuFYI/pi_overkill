/**
 * pi_overkill finds the value of pi the long way.
 * © Copyright 2020 JonLiuFYI
 * main.rs is licensed under GPL v3+.
 *
 * TODO:
 * progress update
 * verbose mode (show updates on timeline?)
 * just-output-result mode for "bash scripting"
 *
 * BIG TODO:
 * do this on GPU (OpenCL?)
 */
use num::integer::gcd;
use rand::distributions::{Distribution, Uniform};
use std::f64::consts::PI;
use std::sync::mpsc;
use std::thread;
use clap::{App, Arg};

fn main() {
    let args = App::new("pi_overkill")
        .version("0.3.0")
        .author("JonLiuFYI")
        .about("Iteratively determine the value of pi using Matt Parker's Pi Day 2017 algorithm")
        .arg(Arg::with_name("iters")
            .help("Number of iterations per thread. Must be positive.")
            .short("i")
            .long("iters")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("threads")
            .help("Number of threads to create for calculating pi.")
            .short("t")
            .long("threads")
            .takes_value(true)
            .required(true))
        .get_matches();

    // parameters
    // ==========
    let num_iterations: u32 = match args.value_of("iters").unwrap().parse() {
        Err(_) => panic!("`--iter` ERROR: Number of iterations must be a positive integer!"),
        Ok(0) => panic!("`--iter` ERROR: Number of iterations can't be zero! It must be a positive integer."),
        Ok(i) => i,
    };
    let num_threads: u32 = match args.value_of("threads").unwrap().parse() {
        Err(_) => panic!("`--threads` ERROR: Number of threads must be a positive integer!"),
        Ok(0) => panic!("`--threads` ERROR: Number of threads can't be zero! It must be a positive integer."),
        Ok(t) => t,
    };

    let (tx, rx) = mpsc::channel();

    // start threads
    // =============
    let mut threads = vec![];
    for n in 0..num_threads - 1 {
        let sender = tx.clone();
        threads.push(thread::spawn(move || {
            count_coprimes(&num_iterations, n, sender);
        }));
    }
    threads.push(thread::spawn(move || {
        count_coprimes(&num_iterations, num_threads - 1, tx);
    }));

    // get results from threads
    // ========================
    let mut num_coprime: u32 = 0;
    for _ in threads {
        let addme = rx.recv().expect("No more senders left");
        num_coprime += addme
    }

    // present the value of pi
    // =======================
    /* the probability of two random positive ints being coprime is 6 / pi^2
     * P = 6 / π²
     * π = sqrt(6 / P)
     *      since P = coprimes / total trials
     * π = sqrt((6 * total trials) / coprimes)
     */
    let pi: f64 = ((6 * num_iterations * num_threads) as f64 / num_coprime as f64).sqrt();
    println!();
    println!("π = {}", pi);
    println!(
        "Calculated in {} iterations across {} threads",
        (num_threads * num_iterations),
        num_threads
    );
    println!("Percentage error: {}%", 100_f64 * f64::abs(1_f64 - pi / PI));
}

/**
 * Generate two random u32s for each iteration and determine how many are coprime.
 * Spawn a thread that calls this. Send the result to a channel that the main thread listens to.
 * # Arguments
 * iters: how many pairs of u32s to generate and check
 * thread_num: the number assigned to this thread
 * tx: send coprime count to origin thread via this Sender
 */
fn count_coprimes(iters: &u32, thread_num: u32, tx: mpsc::Sender<u32>) {
    println!("Thread {} is starting", thread_num);

    let dist = Uniform::from(1..u32::MAX);
    let mut rng = rand::thread_rng();
    let mut num_coprime = 0;

    for _ in 0..*iters {
        let x: u32 = dist.sample(&mut rng);
        let y: u32 = dist.sample(&mut rng);
        // x and y are coprime
        if gcd(x, y) == 1 {
            num_coprime += 1;
        }
    }
    // TODO: handle this Result more elegantly
    tx.send(num_coprime)
        .expect("Failed to send coprime count to origin thread!");
}
