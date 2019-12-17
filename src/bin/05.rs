use adventofcode2019::*;
use std::fs;

fn main() {
    // load input file
    let input: String =
        fs::read_to_string("input/05.txt").expect("Unable to read from file: input/05.txt");
    let intcode: Vec<i64> = input
        .split(',')
        .map(|x| x.trim().parse::<i64>().expect("unable to convert to int"))
        .collect();

    let mut computer = IntcodeComputer::new();

    computer.load_program(&intcode);
    let output = computer.execute(&[1]);
    println!("Part 1: {}", output[output.len() - 1]);

    computer.load_program(&intcode);
    let output = computer.execute(&[5]);
    println!("Part 2: {}", output[output.len() - 1]);
}
