#![allow(dead_code)]

#[macro_use]
extern crate itertools;

mod bingo;

/*
 * -- Advent of Code: Day 4 --
 * https://adventofcode.com/2021/day/4
 */

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread;

use itertools::Itertools;
const PUZZLE_INPUT_PATH: &str = "./puzzle_input.txt";
const BOARD_SIZE: usize = 5;

fn main() {
    // Read input file
    let puzzle_input_file = File::open(PUZZLE_INPUT_PATH).expect("Can't find input file");
    let puzzle_input_lines: Vec<String> = BufReader::new(puzzle_input_file)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    // Split input into bingo numbers and bingo boards
    let bingo_numbers: Vec<usize> = puzzle_input_lines
        .first()
        .unwrap()
        .split(',')
        .map(|num| num.parse::<usize>().expect("bingo numbers are numbers"))
        .collect();
    let bingo_boards: Vec<bingo::Board> = puzzle_input_lines[2..]
        .chunks(BOARD_SIZE + 1)
        .map(|c| c.into())
        .collect();

    // Determine the order the boards win in seperately on each thread
    let nums = Arc::new(bingo_numbers);
    let rx = {
        let (tx, rx) = mpsc::channel::<(usize, bingo::Board)>();
        let mut children = Vec::new();
        for mut board in bingo_boards {
            let tx = tx.clone();
            let nums = nums.clone();
            children.push(thread::spawn(move || {
                for (i, &num) in nums.iter().enumerate() {
                    board.mark(num);
                    if board.has_won() {
                        tx.send((i, board)).expect("send on channel");
                        return;
                    }
                }
                // dbg!(board);
            }));
        }

        // Join on children then close channel
        for child in children {
            child.join().expect("Join on child");
        }

        rx
    };

    // Collect results then determine score of winning board
    let results: Vec<_> = rx.iter().sorted_by_key(|(i, _board)| *i).collect();
    if let Some((i, board)) = results.first() {
        let last_num = nums[*i];
        let score = board.calculate_score(last_num);
        println!("[PT1] First winning board has score {}", score);
    }

    // Determine the score of the last board
    if let Some((i, board)) = results.last() {
        let last_num = nums[*i];
        let score = board.calculate_score(last_num);
        println!("[PT2] Last winning board has score {}", score);
    }
}
