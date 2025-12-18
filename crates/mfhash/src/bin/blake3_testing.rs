use std::{collections::{HashMap, HashSet}, time::Instant};
use mfhash::{Blake3Hasher, deterministic::DeterministicHash};

pub fn main() {
    const ITERATIONS: usize = 10usize.pow(9);
    const ONE_PERCENT_ITERATIONS: usize = ITERATIONS / 100;
    let mut hashes = HashSet::<u64>::with_capacity(ITERATIONS);
    let mut collision_counts = HashMap::<u64, usize>::new();
    let mut total_collisions: usize = 0;
    let mut track_collision = {
        let hashes = &mut hashes;
        let collision_counts = &mut collision_counts;
        let total_collisions = &mut total_collisions;
        move |hash: u64| {
            if !hashes.insert(hash) {
                *total_collisions += 1;
                *collision_counts.entry(hash).or_insert(0) += 1;
            }
        }
    };
    let start_time = Instant::now();
    for i in 0..ITERATIONS {
        let mut hasher = Blake3Hasher::new();
        (1, 2, 3, i).deterministic_hash(&mut hasher);
        let hash = hasher.finish_u64();
        track_collision(hash);
        if (i % ONE_PERCENT_ITERATIONS) == 0 {
            let percent = i / ONE_PERCENT_ITERATIONS;
            println!("{percent}% Finished");
        }
    }
    println!("100% Finished");
    let elapsed_time = start_time.elapsed();
    println!("Elapsed Time: {elapsed_time:.3?}");
    let collision_ratio = total_collisions as f64 / ITERATIONS as f64;
    println!("Collision Count: {total_collisions} / {ITERATIONS} (r: {collision_ratio})");
    let mut max_collisions = 0;
    let mut max_collided = 0;
    for (&hash, &count) in collision_counts.iter() {
        if count > max_collisions {
            max_collisions = count;
            max_collided = hash;
        }
    }
    println!("Max Collisions: {max_collisions} ({max_collided})");
}