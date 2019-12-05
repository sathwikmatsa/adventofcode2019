#[macro_use]
extern crate text_io;
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
            for _ in 0..3 - pm_len {
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
                let input: i32 = read!();
                intcode[first_param as usize] = input;

                input_size = 1;
            }
            4 => {
                if param_modes[0] == 0 {
                    println!("{}", intcode[first_param as usize]);
                } else {
                    println!("{}", first_param);
                }

                input_size = 1;
            }
            5 => {
                let param1 = if param_modes[0] == 0 {
                    intcode[first_param as usize]
                } else {
                    first_param
                };
                let param2 = if param_modes[1] == 0 {
                    intcode[second_param as usize]
                } else {
                    second_param
                };

                if param1 != 0 {
                    index = param2 as usize;
                    opcode = intcode[index].abs() % 100;
                    continue;
                }
                input_size = 2;
            }
            6 => {
                let param1 = if param_modes[0] == 0 {
                    intcode[first_param as usize]
                } else {
                    first_param
                };
                let param2 = if param_modes[1] == 0 {
                    intcode[second_param as usize]
                } else {
                    second_param
                };

                if param1 == 0 {
                    index = param2 as usize;
                    opcode = intcode[index].abs() % 100;
                    continue;
                }
                input_size = 2;
            }
            7 => {
                let param1 = if param_modes[0] == 0 {
                    intcode[first_param as usize]
                } else {
                    first_param
                };
                let param2 = if param_modes[1] == 0 {
                    intcode[second_param as usize]
                } else {
                    second_param
                };

                if param1 < param2 {
                    intcode[third_param as usize] = 1;
                } else {
                    intcode[third_param as usize] = 0;
                }
                input_size = 3;
            }
            8 => {
                let param1 = if param_modes[0] == 0 {
                    intcode[first_param as usize]
                } else {
                    first_param
                };
                let param2 = if param_modes[1] == 0 {
                    intcode[second_param as usize]
                } else {
                    second_param
                };

                if param1 == param2 {
                    intcode[third_param as usize] = 1;
                } else {
                    intcode[third_param as usize] = 0;
                }
                input_size = 3;
            }
            _ => {
                eprintln!("invalid opcode");
            }
        }

        index += input_size + 1;
        opcode = intcode[index].abs() % 100;
    }
}

fn main() {
    // load input file
    let input: String =
        fs::read_to_string("input/05.txt").expect("Unable to read from file: input/05.txt");
    let mut intcode: Vec<i32> = input
        .split(',')
        .map(|x| x.trim().parse::<i32>().expect("unable to convert to int"))
        .collect();

    intcode_computer(&mut intcode);
}
