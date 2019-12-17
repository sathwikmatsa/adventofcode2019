use adventofcode2019::*;
use itertools::Itertools;
use std::fs;

fn max_output_signal(computer: &mut IntcodeComputer, program: &[i64]) -> i64 {
    let mut output = Vec::new();
    let phase_settings = (0..5).permutations(5);
    for phase_setting in phase_settings {
        // A
        computer.load_program(program);
        let in_b = computer.execute(&[phase_setting[0], 0]).pop().unwrap();
        // B
        computer.load_program(program);
        let in_c = computer.execute(&[phase_setting[1], in_b]).pop().unwrap();
        // C
        computer.load_program(program);
        let in_d = computer.execute(&[phase_setting[2], in_c]).pop().unwrap();
        // D
        computer.load_program(program);
        let in_e = computer.execute(&[phase_setting[3], in_d]).pop().unwrap();
        // E
        computer.load_program(program);
        let outsig = computer.execute(&[phase_setting[4], in_e]).pop().unwrap();
        output.push(outsig);
    }
    *(output.iter().max().unwrap())
}

fn max_output_signal_fb_loop(program: &[i64]) -> i64 {
    let mut output = Vec::new();
    let phase_settings = (5..10).permutations(5);
    for phase_setting in phase_settings {
        let mut amp_a = IntcodeComputer::new();
        let mut amp_b = IntcodeComputer::new();
        let mut amp_c = IntcodeComputer::new();
        let mut amp_d = IntcodeComputer::new();
        let mut amp_e = IntcodeComputer::new();

        amp_a.load_program(program);
        amp_b.load_program(program);
        amp_c.load_program(program);
        amp_d.load_program(program);
        amp_e.load_program(program);

        amp_a.execute(&[phase_setting[0]]);
        amp_b.execute(&[phase_setting[1]]);
        amp_c.execute(&[phase_setting[2]]);
        amp_d.execute(&[phase_setting[3]]);
        amp_e.execute(&[phase_setting[4]]);

        let mut signal = 0;
        let mut in_b;
        let mut in_c;
        let mut in_d;
        let mut in_e;

        loop {
            let output = amp_a.execute(&[signal]);
            in_b = output[output.len() - 1];
            let output = amp_b.execute(&[in_b]);
            in_c = output[output.len() - 1];
            let output = amp_c.execute(&[in_c]);
            in_d = output[output.len() - 1];
            let output = amp_d.execute(&[in_d]);
            in_e = output[output.len() - 1];
            let output = amp_e.execute(&[in_e]);
            signal = output[output.len() - 1];

            if amp_e.done {
                break;
            }
        }
        output.push(signal);
    }
    *(output.iter().max().unwrap())
}

fn main() {
    // load input file
    let input: String =
        fs::read_to_string("input/07.txt").expect("Unable to read from file: input/07.txt");
    let intcode: Vec<i64> = input
        .split(',')
        .map(|x| x.trim().parse::<i64>().expect("unable to convert to int"))
        .collect();

    let mut computer = IntcodeComputer::new();
    println!("{}", max_output_signal(&mut computer, &intcode));
    println!("{}", max_output_signal_fb_loop(&intcode));
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_max_output_signal() {
        let mut computer = IntcodeComputer::new();
        let intcode = [
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(43210, max_output_signal(&mut computer, &intcode));
        let intcode = [
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(54321, max_output_signal(&mut computer, &intcode));
        let intcode = [
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(65210, max_output_signal(&mut computer, &intcode));
    }
    #[test]
    fn test_max_output_signal_fb_loop() {
        let intcode = [
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(18216, max_output_signal_fb_loop(&intcode));
        let intcode = [
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(139_629_729, max_output_signal_fb_loop(&intcode));
    }
}
