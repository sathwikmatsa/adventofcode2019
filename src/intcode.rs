use std::convert::From;

const DEBUG: bool = true;
enum Opcode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
    Invalid,
}

impl Opcode {
    fn param_len(&self) -> usize {
        match self {
            Self::Add => 3,
            Self::Mul => 3,
            Self::Input => 1,
            Self::Output => 1,
            Self::JumpIfTrue => 2,
            Self::JumpIfFalse => 2,
            Self::LessThan => 3,
            Self::Equals => 3,
            Self::Halt => 0,
            Self::Invalid => 0,
        }
    }
}

impl From<i64> for Opcode {
    fn from(item: i64) -> Self {
        match item {
            1 => Self::Add,
            2 => Self::Mul,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LessThan,
            8 => Self::Equals,
            99 => Self::Halt,
            _ => Self::Invalid,
        }
    }
}

enum Mode {
    Positional,
    Immediate,
    Invalid,
}

impl From<usize> for Mode {
    fn from(item: usize) -> Self {
        match item {
            0 => Self::Positional,
            1 => Self::Immediate,
            _ => Self::Invalid,
        }
    }
}

#[derive(Default)]
pub struct IntcodeComputer {
    pub memory: Vec<i64>,
}

impl IntcodeComputer {
    pub fn new() -> Self {
        Self { memory: Vec::new() }
    }
    fn decode(ins: i64) -> (Opcode, Vec<Mode>) {
        let opcode = Opcode::from(ins % 100);
        let param_modes = ins
            .to_string()
            .chars()
            .rev()
            .skip(2)
            .map(|x| Mode::from(x.to_digit(10).unwrap() as usize))
            .collect();
        (opcode, param_modes)
    }
    fn fetch(&self, index: usize, mode: &Mode) -> i64 {
        match mode {
            Mode::Positional => self.memory[self.memory[index] as usize],
            Mode::Immediate => self.memory[index],
            _ => panic!("Invalid param mode"),
        }
    }
    pub fn run_program(&mut self, program: &[i64], stdin: &[i64]) -> Vec<i64> {
        self.memory.clear();
        self.memory.extend_from_slice(program);
        let mut stdout = Vec::new();

        let mut ip = 0;
        let mut input_index = 0;
        loop {
            let (opcode, pmodes) = Self::decode(self.memory[ip]);
            match opcode {
                Opcode::Add => {
                    let op1 = self.fetch(ip + 1, pmodes.get(0).unwrap_or(&Mode::Positional));
                    let op2 = self.fetch(ip + 2, pmodes.get(1).unwrap_or(&Mode::Positional));
                    let op3 = self.memory[ip + 3];
                    self.memory[op3 as usize] = op1 + op2;
                    ip += Opcode::Add.param_len() + 1;
                }
                Opcode::Mul => {
                    let op1 = self.fetch(ip + 1, pmodes.get(0).unwrap_or(&Mode::Positional));
                    let op2 = self.fetch(ip + 2, pmodes.get(1).unwrap_or(&Mode::Positional));
                    let op3 = self.memory[ip + 3];
                    self.memory[op3 as usize] = op1 * op2;
                    ip += Opcode::Mul.param_len() + 1;
                }
                Opcode::Input => {
                    let input: i64 = if input_index < stdin.len() {
                        input_index += 1;
                        stdin[input_index - 1]
                    } else {
                        println!("expecting input: ");
                        read!()
                    };
                    let op1 = self.memory[ip + 1];
                    self.memory[op1 as usize] = input;
                    ip += Opcode::Input.param_len() + 1;
                }
                Opcode::Output => {
                    let op1 = self.fetch(ip + 1, pmodes.get(0).unwrap_or(&Mode::Positional));
                    if DEBUG {
                        println!("output: {}", op1);
                    }
                    stdout.push(op1);
                    ip += Opcode::Output.param_len() + 1;
                }
                Opcode::JumpIfTrue => {
                    let op1 = self.fetch(ip + 1, pmodes.get(0).unwrap_or(&Mode::Positional));
                    let op2 = self.fetch(ip + 2, pmodes.get(1).unwrap_or(&Mode::Positional));
                    if op1 != 0 {
                        ip = op2 as usize;
                        continue;
                    }
                    ip += Opcode::JumpIfTrue.param_len() + 1;
                }
                Opcode::JumpIfFalse => {
                    let op1 = self.fetch(ip + 1, pmodes.get(0).unwrap_or(&Mode::Positional));
                    let op2 = self.fetch(ip + 2, pmodes.get(1).unwrap_or(&Mode::Positional));
                    if op1 == 0 {
                        ip = op2 as usize;
                        continue;
                    }
                    ip += Opcode::JumpIfFalse.param_len() + 1;
                }
                Opcode::LessThan => {
                    let op1 = self.fetch(ip + 1, pmodes.get(0).unwrap_or(&Mode::Positional));
                    let op2 = self.fetch(ip + 2, pmodes.get(1).unwrap_or(&Mode::Positional));
                    let op3 = self.memory[ip + 3];
                    self.memory[op3 as usize] = if op1 < op2 { 1 } else { 0 };
                    ip += Opcode::LessThan.param_len() + 1;
                }
                Opcode::Equals => {
                    let op1 = self.fetch(ip + 1, pmodes.get(0).unwrap_or(&Mode::Positional));
                    let op2 = self.fetch(ip + 2, pmodes.get(1).unwrap_or(&Mode::Positional));
                    let op3 = self.memory[ip + 3];
                    self.memory[op3 as usize] = if op1 == op2 { 1 } else { 0 };
                    ip += Opcode::Equals.param_len() + 1;
                }
                Opcode::Halt => break,
                Opcode::Invalid => panic!("Invalid opcode"),
            }
        }

        stdout
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn test_add_mul() {
        let mut computer = IntcodeComputer::new();
        computer.run_program(&[1, 0, 0, 0, 99], &[]);
        assert_eq!(computer.memory, vec![2, 0, 0, 0, 99]);
        computer.run_program(&[2, 3, 0, 3, 99], &[]);
        assert_eq!(computer.memory, vec![2, 3, 0, 6, 99]);
        computer.run_program(&[2, 4, 4, 5, 99, 0], &[]);
        assert_eq!(computer.memory, vec![2, 4, 4, 5, 99, 9801]);
        computer.run_program(&[1, 1, 1, 4, 99, 5, 6, 0, 99], &[]);
        assert_eq!(computer.memory, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
    #[test]
    fn test_io() {
        let mut computer = IntcodeComputer::new();
        let output = computer.run_program(&[3, 0, 4, 0, 99], &[25]);
        assert_eq!(output, vec![25]);
    }
    #[test]
    fn negative_integers() {
        let mut computer = IntcodeComputer::new();
        computer.run_program(&[1101, 100, -1, 4, 0], &[]);
        assert_eq!(computer.memory, vec![1101, 100, -1, 4, 99]);
    }
    #[test]
    fn test_equals_and_less_than() {
        let mut computer = IntcodeComputer::new();
        let output = computer.run_program(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &[4]);
        assert_eq!(output, vec![0]);
        let output = computer.run_program(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &[8]);
        assert_eq!(output, vec![1]);
        let output = computer.run_program(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &[4]);
        assert_eq!(output, vec![1]);
        let output = computer.run_program(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &[8]);
        assert_eq!(output, vec![0]);
        let output = computer.run_program(&[3, 3, 1108, -1, 8, 3, 4, 3, 99], &[8]);
        assert_eq!(output, vec![1]);
        let output = computer.run_program(&[3, 3, 1108, -1, 8, 3, 4, 3, 99], &[0]);
        assert_eq!(output, vec![0]);
        let output = computer.run_program(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], &[18]);
        assert_eq!(output, vec![0]);
        let output = computer.run_program(&[3, 3, 1107, -1, 8, 3, 4, 3, 99], &[6]);
        assert_eq!(output, vec![1]);
    }
    #[test]
    fn test_jumps() {
        let mut computer = IntcodeComputer::new();
        let output = computer.run_program(
            &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            &[4],
        );
        assert_eq!(output, vec![1]);
        let output = computer.run_program(
            &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
            &[0],
        );
        assert_eq!(output, vec![0]);
        let output = computer.run_program(&[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], &[4]);
        assert_eq!(output, vec![1]);
        let output = computer.run_program(&[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], &[0]);
        assert_eq!(output, vec![0]);
    }
    #[test]
    fn mega_test_jump_compare() {
        let mut computer = IntcodeComputer::new();
        let program = [
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let output = computer.run_program(&program, &[4]);
        assert_eq!(output, vec![999]);
        let output = computer.run_program(&program, &[8]);
        assert_eq!(output, vec![1000]);
        let output = computer.run_program(&program, &[14]);
        assert_eq!(output, vec![1001]);
    }
}
