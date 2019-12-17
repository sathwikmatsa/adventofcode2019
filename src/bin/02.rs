use adventofcode2019::*;
use std::fs;

fn find_noun_verb(
    computer: &mut IntcodeComputer,
    program: &mut Vec<i64>,
    output: i64,
) -> (i64, i64) {
    for i in 0..99 {
        for j in 0..99 {
            program[1] = i;
            program[2] = j;

            computer.run_program(program, &[]);
            if computer.memory[0] == output {
                return (i, j);
            }
        }
    }

    (-1, -1)
}

fn main() {
    // load input file
    let input: String =
        fs::read_to_string("input/02.txt").expect("Unable to read from file: input/02.txt");
    let mut program: Vec<i64> = input
        .split(',')
        .map(|x| x.trim().parse::<i64>().expect("unable to convert to int"))
        .collect();

    let mut computer = IntcodeComputer::new();

    program[1] = 12;
    program[2] = 2;
    computer.run_program(&program, &[]);
    let part1 = computer.memory[0];

    let (noun, verb) = find_noun_verb(&mut computer, &mut program, 19_690_720);
    let part2 = 100 * noun + verb;

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
