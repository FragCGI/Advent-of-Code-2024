use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input file
    let data = fs::read_to_string(env::args().nth(1).ok_or("No file argument")?)?;

    // Parse, sort and compute total difference in a single chain of operations
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = data
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();

    // Sort both vectors
    left.sort_unstable();
    right.sort_unstable();

    // Compute the total difference
    let diff: i32 = left
        .iter()
        .zip(&right)
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("{}", diff);
    Ok(())
}

