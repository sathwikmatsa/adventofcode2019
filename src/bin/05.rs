#[macro_use] extern crate text_io;
use std::fs;

fn intcode_computer(intcode: &mut Vec<i32>) {
    let mut index = 0;
    let mut opcode = intcode[index].abs() % 100;
    while opcode != 99 {
        let mut param_modes: Vec<u32> = intcode[index]
            .to_string()
            .chars()
            .rev()
            .skip(2)
            .map(|x| x.to_digit(10).unwrap())
            .collect();

        let pm_len = param_modes.len();
        if pm_len < 3 {
            for _ in 0..3-pm_len {
                param_modes.push(0);
            }
        }

        let first_param = intcode[index + 1];
        let second_param = intcode[index + 2];
        let third_param = intcode[index + 3];

        let mut input_size = 0;

        match opcode {
            1 => {
                intcode[third_param as usize] = if param_modes[0] == 0 {
                    intcode[first_param as usize]
                } else {
                    first_param
                } + if param_modes[1] == 0 {
                    intcode[second_param as usize]
                } else {
                    second_param
                };

                input_size = 3;
            }
            2 => {
                intcode[third_param as usize] = if param_modes[0] == 0 {
                    intcode[first_param as usize]
                } else {
                    first_param
                } * if param_modes[1] == 0 {
                    intcode[second_param as usize]
                } else {
                    second_param
                };

                input_size = 3;
            }
            3 => {
                let input : i32 = read!();                
                intcode[first_param as usize] = input;
            }
            4 => {
                if param_modes[0] == 0 {
                    println!("{}", intcode[first_param as usize]);
                } else {
                    println!("{}", first_param);
                }
            }
            _ => { eprintln!("invalid opcode"); }
        }

        index += input_size + 1;
        opcode = intcode[index].abs() % 100;
    }
}

fn main() {
    // load input file
    let input: String =
        fs::read_to_string("input/05.txt").expect("Unable to read from file: input/05.txt");
    let intcode: Vec<i32> = input
        .split(',')
        .map(|x| x.trim().parse::<i32>().expect("unable to convert to int"))
        .collect();

    let mut intcode_p1 = intcode.clone();

    intcode_computer(&mut intcode_p1);
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_intcode_computer() {
        let mut ic1: Vec<i32> = vec![1, 0, 0, 0, 99];
        let mut ic2: Vec<i32> = vec![2, 3, 0, 3, 99];
        let mut ic3: Vec<i32> = vec![2, 4, 4, 5, 99, 0];
        let mut ic4: Vec<i32> = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];

        intcode_computer(&mut ic1);
        intcode_computer(&mut ic2);
        intcode_computer(&mut ic3);
        intcode_computer(&mut ic4);

        assert_eq!(ic1, vec![2, 0, 0, 0, 99]);
        assert_eq!(ic2, vec![2, 3, 0, 6, 99]);
        assert_eq!(ic3, vec![2, 4, 4, 5, 99, 9801]);
        assert_eq!(ic4, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
