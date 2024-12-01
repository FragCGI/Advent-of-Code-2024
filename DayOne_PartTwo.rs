use std::collections::HashMap;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read input file
    let data = fs::read_to_string(env::args().nth(1).ok_or("No file argument")?)?;

    // Parse input into two columns (left and right)
    let (left, right): (Vec<i32>, Vec<i32>) = data
        .lines()
        .map(|line| {
            let mut nums = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();

    // Count occurrences of each number in the right column
    let mut right_counts = HashMap::new();
    for &num in &right {
        *right_counts.entry(num).or_insert(0) += 1;
    }

    // Calculate the similarity score
    let similarity_score: i32 = left
        .iter()
        .map(|&num| {
            let count = *right_counts.get(&num).unwrap_or(&0); // Get the count of the number in the right column
            num * count
        })
        .sum();

    // Output the similarity score
    println!("{}", similarity_score);

    Ok(())
}

