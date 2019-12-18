use adventofcode2019::*;
use std::fs;

fn main() {
    // load input file
    let input: String =
        fs::read_to_string("input/09.txt").expect("Unable to read from file: input/09.txt");
    let boost_program: Vec<i64> = input
        .split(',')
        .map(|x| x.trim().parse::<i64>().expect("unable to convert to int"))
        .collect();

    let mut computer = IntcodeComputer::new();

    computer.load_program(&boost_program);
    let output = computer.execute(&[1]);
    println!("Part 1: {}", output[0]);

    computer.load_program(&boost_program);
    let output = computer.execute(&[2]);
    println!("Part 2: {}", output[0]);
}
