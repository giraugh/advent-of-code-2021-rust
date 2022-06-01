use std::fs::File;
use std::io::{BufReader, BufRead};

const PUZZLE_INPUT_PATH: &str = "./puzzle_input.txt";

fn main() {
    // Read input file
    let puzzle_input_file = File::open(PUZZLE_INPUT_PATH).expect("Can't find input file");
    let puzzle_input_lines: Vec<String> = BufReader::new(puzzle_input_file)
        .lines().map(|l| l.unwrap()).collect();
   
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

// Determine gamma bits from input lines
fn gamma_bits_from_lines(lines: &[String]) -> Vec<u8> {
    // Count 1s in each column
    let mut col_counts = vec!(0; lines[0].len());
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            col_counts[i] += (c == '1') as usize
        };
    };
    
    // Convert col counts to bits
    col_counts
        .iter()
        .map(|&c| if c > lines.len() / 2 { 1 } else { 0 } )
        .collect()
}

// Convert list of binary digits to integer
fn convert_bits(bits: &[u8]) -> usize {
    bits
        .iter()
        .fold(0 as usize, |acc, &x| (acc << 1) ^ (x as usize))
}
