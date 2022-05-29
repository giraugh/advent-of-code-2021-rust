/*
 * -- Advent of Code: Day 1 --
 * https://adventofcode.com/2021/day/1
 */

use std::fs::File;
use std::io::{BufReader, BufRead};

const SAMPLE_DEPTHS: [isize; 10] = [ 99, 200, 208, 210, 200, 207, 240, 269, 260, 263 ];
const SAMPLE_INCREASES_PT1: usize = 7;
const SAMPLE_INCREASES_PT2: usize = 5;
const PUZZLE_INPUT_PATH: &str = "./puzzle_input.txt";
const PUZZLE_WINDOW_SIZE: usize = 3;

fn main() -> Result<(), std::io::Error> {
    // Read input file
    let puzzle_input_file = File::open(PUZZLE_INPUT_PATH).expect("Can't find input file");
    let puzzle_depths: Vec<isize> = BufReader::new(puzzle_input_file)
        .lines().map(|l| l.unwrap().parse().unwrap()).collect();

    part_1(&puzzle_depths)?;
    part_2(&puzzle_depths)?;

    Ok(())
}

fn part_1(puzzle_depths: &[isize]) -> Result<(), std::io::Error> {
    // Test using sample data
    let sample_increases = count_increases(&SAMPLE_DEPTHS);
    println!("[PT1] Sample increases: {}", sample_increases);
    assert_eq!(sample_increases, SAMPLE_INCREASES_PT1);

    // Run on puzzle input
    let puzzle_increases = count_increases(&puzzle_depths);
    println!("[PT1] Puzzle increases: {}", puzzle_increases);

    Ok(())
}

fn part_2(puzzle_depths: &[isize]) -> Result<(), std::io::Error> {
    // Test using sample data
    let sample_windows = sum_windows(&SAMPLE_DEPTHS, PUZZLE_WINDOW_SIZE);
    let sample_increases = count_increases(&sample_windows);
    println!("[PT2] Sample increases: {}", sample_increases);
    assert_eq!(sample_windows.len(), 8);
    assert_eq!(sample_increases, SAMPLE_INCREASES_PT2);

    // Run on puzzle input
    let puzzle_windows = sum_windows(puzzle_depths, PUZZLE_WINDOW_SIZE);
    let puzzle_increases = count_increases(&puzzle_windows);
    println!("[PT2] Puzzle increases: {}", puzzle_increases);

    Ok(())
}

// Count number of elements greater than the previous element
fn count_increases(depths: &[isize]) -> usize {
    depths.iter()
        .zip(depths.iter().skip(1))
        .map(|(depth, next_depth)| (depth < next_depth) as usize)
        .sum()
}

// Sum elements in sliding windows
fn sum_windows(depths: &[isize], window_size: usize) -> Vec<isize> {
   (0..=depths.len() - window_size)
        .map(|i| depths[i..i+window_size].iter().sum())
        .collect()
}
