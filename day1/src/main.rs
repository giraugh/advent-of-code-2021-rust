/*
 * -- Advent of Code: Day 1 --
 * https://adventofcode.com/2021/day/1
 */

use std::fs::File;
use std::io::{BufReader, BufRead};

const SAMPLE_DEPTHS: [i32; 10] = [ 99, 200, 208, 210, 200, 207, 240, 269, 260, 263 ];
const SAMPLE_INCREASES: i32 = 7;
const PUZZLE_INPUT_PATH: &str = "./puzzle_input.txt";

fn main() {
    // Test using sample data
    let sample_increases = count_increases(From::from(SAMPLE_DEPTHS));
    println!("Sample increases: {}", sample_increases);
    assert_eq!(sample_increases, SAMPLE_INCREASES);

    // Read input file
    let puzzle_input_file = File::open(PUZZLE_INPUT_PATH)
        .unwrap_or_else(|_| panic!("Expected to find puzzle input file at {}", PUZZLE_INPUT_PATH));
    let puzzle_depths = BufReader::new(puzzle_input_file)
        .lines().map(|l| l.unwrap().parse().unwrap()).collect();

    // Test using puzzle input
    let puzzle_increases = count_increases(puzzle_depths);
    println!("Puzzle increases: {}", puzzle_increases);
}

// Count number of elements greater than the previous element
fn count_increases(depths: Vec<i32>) -> i32 {
    depths.iter()
        .zip(depths.iter().skip(1))
        .map(|(depth, next_depth)| (depth < next_depth) as i32)
        .sum()
}
