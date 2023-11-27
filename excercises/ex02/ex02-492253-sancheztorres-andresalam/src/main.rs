//! Author: Andres Alam Sanchez Torres
//!
//! Finds the minimum maximum Hailstone number appearing in the Collatz Sequence
//! for every starting number in a given range of [a, b], i.e.
//!
//! $ \min_{\forall n \in [a, b]} \max_{i} f^i(n) $
//!
use std::collections::HashMap;
use std::env;

/// Finds the max Hailstone number for a Collatz sequence starting at `n`.
///
/// Uses a memoization map `max_hailstone_cache` to prevent re-computation
/// of max values, i.e. let $f(n)$ be the next hailstone number from $n$ and
/// $g(n)$ a function that returns the max hailstone number of a sequence,
/// defined as
///
/// $g(n) = max(n, g(f(n)))$
///
fn find_max_hailstone(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
    if let Some(max_value) = cache.get(&n) {
        return *max_value;
    }

    let next_hailstone = if n % 2 == 0 {
        n / 2
    } else {
        n * 3 + 1
    };

    let max_value = n.max(find_max_hailstone(next_hailstone, cache));
    cache.insert(n, max_value);

    return max_value;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let a: i32 = args
        .get(1)
        .expect("Not enough arguments")
        .parse()
        .expect("First argument is not a number");

    let b: i32 = args
        .get(2)
        .expect("Not enough arguments")
        .parse()
        .expect("Second argument is not a number");

    // Cache to avoid re-calculating a same collatz sequence
    let mut max_hailstone_cache: HashMap<i32, i32> = HashMap::new();

    // Set base cases
    max_hailstone_cache.insert(1, 1);
    max_hailstone_cache.insert(2, 2);

    // Compute max hailstone for each n in [a, b] and get minimum
    let evaluation_range = a..b + 1;
    let min_value = evaluation_range
        .map(|n| find_max_hailstone(n, &mut max_hailstone_cache))
        .min()
        .unwrap();

    print!("{}", min_value);
}
