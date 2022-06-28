#![allow(dead_code)]

/*
 * -- Advent of Code: Day 4 --
 * https://adventofcode.com/2021/day/4
 */

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
    
    // Split input into bingo number and bingo boards
    let puzzle_input_blocks = puzzle_input_lines.iter().split_when(|line| line.is_empty());
    let bingo_numbers: &Vec<_> = &puzzle_input_blocks[0].first().unwrap().split(',').collect();
    let bingo_boards = &puzzle_input_blocks[1..]; 

    println!("{} bingo numbers", bingo_numbers.len());
    println!("{} bingo boards", bingo_boards.len());
    println!("{:?}", bingo_boards[0]);
}

mod bingo_board {
    #[derive(Debug, Default)]
    struct Cell {
        number: usize,
        marked: bool,
    }

    #[derive(Debug)]
    struct Board {
        rows: Vec<Vec<Cell>>
    }

    impl Cell {
        fn new(number: usize) -> Self {
            Self {
                number,
                marked: false,
            }
        }
    }

    impl Board {
        fn new(lines: Vec<String>) -> Self { !unimplemented!() }
        fn set_marked(&self, row: usize, col: usize) { !unimplemented!() }
        fn calculate_score(&self) -> usize { !unimplemented!() }
    }
}


trait SplitWhenExt: Iterator {
    fn split_when<F>(&mut self, pred: F) -> Vec<Vec<Self::Item>>
        where Self: Sized, F: Fn(&Self::Item) -> bool;
}

impl<I: Iterator> SplitWhenExt for I {
    fn split_when<F>(&mut self, pred: F) -> Vec<Vec<Self::Item>>
        where F: Fn(&Self::Item) -> bool
    {
        self
            .fold(vec!(), |mut chunks: Vec<Vec<Self::Item>>, item: Self::Item| {
                if pred(&item) {
                    chunks.push(vec!())
                } else {
                    match chunks.last_mut() {
                        Some(chunk) => chunk.push(item),
                        None => chunks.push(vec!(item)),
                    }
                }
                chunks
            })
    }
}
