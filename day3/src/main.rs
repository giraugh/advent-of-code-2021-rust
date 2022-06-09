/*
 * -- Advent of Code: Day 3 --
 * https://adventofcode.com/2021/day/3
 */

use std::cmp::Ordering::Less;
use std::fs::File;
use std::io::{BufRead, BufReader};

const PUZZLE_INPUT_PATH: &str = "./puzzle_input.txt";

fn main() {
    // Read input file
    let puzzle_input_file = File::open(PUZZLE_INPUT_PATH).expect("Can't find input file");
    let puzzle_input_lines: Vec<String> = BufReader::new(puzzle_input_file)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    part_1(&puzzle_input_lines);
    part_2(&puzzle_input_lines);
}

fn part_1(puzzle_input_lines: &[String]) {
    // Determine gamma and epsilon bits
    let gamma_bits = gamma_bits_from_lines(&puzzle_input_lines);
    let epsilon_bits: Vec<u8> = gamma_bits.iter().map(|x| 1 - x).collect();

    // Determine factors
    let gamma_factor = convert_bits(&gamma_bits);
    let epsilon_factor = convert_bits(&epsilon_bits);

    // Output result
    let result = gamma_factor * epsilon_factor;
    println!("[PT1] Result: {:?}", result);
}

fn part_2(puzzle_input_lines: &[String]) {
    // Convert each line to a vector of bits
    let bit_rows: Vec<Vec<u8>> = puzzle_input_lines
        .iter()
        .map(|line| line.chars().map(|c| if c == '1' { 1 } else { 0 }).collect())
        .collect();

    // Determine oxygen rate
    let oxygen = apply_bit_criteria(&bit_rows, |bits| {
        match (bits.iter().sum::<usize>()*2).cmp(&bits.len()) {
            Less => 0,
            _ => 1,
        }
    });

    // Determine co2 rate
    let co2 = apply_bit_criteria(&bit_rows, |bits| {
        match (bits.iter().sum::<usize>()*2).cmp(&bits.len()) {
            Less => 1,
            _ => 0,
        }
    });

    // Output result
    let result = oxygen * co2;
    println!("[PT2] Result: {:?}", result);
}

fn apply_bit_criteria<F>(bit_rows: &Vec<Vec<u8>>, criteria: F) -> usize
where
    F: Fn(&Vec<usize>) -> u8,
{
    // Make a mutable copy of the rows
    let mut rows = bit_rows.clone();

    // Filter until 1 remains
    while rows.len() > 1 {
        for i in 0..rows[0].len() {
            let desired_bit = criteria(&rows.iter().map(|row| row[i] as usize).collect());
            // println!("Position={} Desired={}", i, desired_bit);
            rows = filter_while_many(&rows, |row| row[i] == desired_bit);
        }
    }

    // Collect remaining row into integer
    convert_bits(&rows[0])
}

// Determine gamma bits from input lines
fn gamma_bits_from_lines(lines: &[String]) -> Vec<u8> {
    // Count 1s in each column
    let mut col_counts = vec![0; lines[0].len()];
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            col_counts[i] += (c == '1') as usize
        }
    }

    // Convert col counts to bits
    col_counts
        .iter()
        .map(|&c| if c > lines.len() / 2 { 1 } else { 0 })
        .collect()
}

// Convert list of binary digits to integer
fn convert_bits(bits: &[u8]) -> usize {
    bits.iter()
        .fold(0 as usize, |acc, &x| (acc << 1) ^ (x as usize))
}

// Apply a predicate to filter a vector but quit early if only one element remains
fn filter_while_many<T, P>(items: &Vec<T>, predicate: P) -> Vec<T>
where
    P: Fn(&T) -> bool,
    T: Clone,
{
    let mut dropped = 0;
    items
        .iter()
        .filter(|x| {
            if predicate(x) || dropped >= items.len() - 1 {
                true
            } else {
                dropped += 1;
                false
            }
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_while_many() {
        let items = vec![0, 1, 2, 3, 4];
        let names = vec!["ewan", "max"];
        assert_eq!(filter_while_many(&items, |&x| x % 2 == 0), vec!(0, 2, 4));
        assert_eq!(filter_while_many(&items, |&x| x > 6), vec!(4));
        assert_eq!(filter_while_many(&names, |&x| x.len() > 5), vec!("max"));
        assert_eq!(
            filter_while_many(&names, |&x| x.len() > 2),
            vec!("ewan", "max")
        );
    }

    #[test]
    fn test_convert_bits() {
        assert_eq!(convert_bits(&vec!(1, 0, 1)), 5);
    }
}
