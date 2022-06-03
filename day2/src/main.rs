/*
 * -- Advent of Code: Day 2 --
 * https://adventofcode.com/2021/day/2
 */

mod submarine;

use std::fs::File;
use std::io::{BufReader, BufRead};

const PUZZLE_INPUT_PATH: &str = "./puzzle_input.txt";

fn main() {
    // Read commands from input
    let input_file = File::open(PUZZLE_INPUT_PATH).expect("Can't find input file");
    let commands: Vec<submarine::Command> = BufReader::new(input_file)
        .lines().map(|l| l.unwrap().parse().unwrap()).collect();
    
    part_1(&commands);
    part_2(&commands);
}

fn part_1(commands: &[submarine::Command]) {
    // Apply commands to determine position
    let submarine = submarine::Submarine::new().perform_commands(&commands, false);
    println!("[PT1] Position Product: {}", submarine.position_product())
}

fn part_2(commands: &[submarine::Command]) {
    // Apply commands to determine position (using aim)
    let submarine = submarine::Submarine::new().perform_commands(&commands, true);
    println!("[PT2] Position Product: {}", submarine.position_product())
}
