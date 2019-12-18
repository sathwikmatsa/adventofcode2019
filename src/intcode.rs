use std::collections::HashMap;
use std::convert::From;

enum Opcode {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    RelativeBaseOffset,
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
            Self::RelativeBaseOffset => 1,
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
            9 => Self::RelativeBaseOffset,
            99 => Self::Halt,
            _ => Self::Invalid,
        }
    }
}

enum Mode {
    Positional,
    Immediate,
    Relative,
    Invalid,
}

impl From<usize> for Mode {
    fn from(item: usize) -> Self {
        match item {
            0 => Self::Positional,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => Self::Invalid,
        }
    }
}

#[derive(Default)]
pub struct IntcodeComputer {
    ip: usize,
    rel_base: i64,
    program_len: usize,
    pub memory: HashMap<usize, i64>,
    pub waiting_for_input: bool,
    pub done: bool,
}

impl IntcodeComputer {
    pub fn new() -> Self {
        Self {
            ip: 0,
            rel_base: 0,
            program_len: 0,
            memory: HashMap::new(),
            waiting_for_input: false,
            done: true,
        }
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
    fn fetch(&mut self, index: usize, mode: &Mode) -> i64 {
        match mode {
            Mode::Positional => {
                self.memory.entry(self.memory[&index] as usize).or_insert(0);
                self.memory[&(self.memory[&index] as usize)]
            }
            Mode::Immediate => self.memory[&index],
            Mode::Relative => {
                self.memory
                    .entry((self.rel_base + self.memory[&index]) as usize)
                    .or_insert(0);
                self.memory[&((self.rel_base + self.memory[&index]) as usize)]
            }
            _ => panic!("Invalid param mode"),
        }
    }
    fn store(&mut self, index: usize, mode: &Mode, value: i64) {
        match mode {
            Mode::Positional => {
                self.memory.insert(self.memory[&index] as usize, value);
            }
            Mode::Relative => {
                self.memory
                    .insert((self.memory[&index] + self.rel_base) as usize, value);
            }
            _ => panic!("Invalid mode in store"),
        }
    }
    pub fn load_program(&mut self, program: &[i64]) {
        self.memory.clear();
        program.iter().enumerate().for_each(|(i, x)| {
            self.memory.insert(i, *x);
        });
        self.ip = 0;
        self.program_len = program.len();
        self.done = false;
        self.rel_base = 0;
        self.waiting_for_input = false;
    }
    pub fn program_memory(&self) -> Vec<i64> {
        let mut pmem = Vec::new();
        for i in 0..self.program_len {
            pmem.push(self.memory[&i]);
        }
        pmem
    }
    pub fn execute(&mut self, stdin: &[i64]) -> Vec<i64> {
        self.waiting_for_input = self.waiting_for_input && stdin.is_empty();
        let mut stdout = Vec::new();
        let mut input_index = 0;
        loop {
            let (opcode, pmodes) = Self::decode(self.memory[&self.ip]);
            match opcode {
                Opcode::Add => {
                    let op1 = self.fetch(self.ip + 1, pmodes.get(0).unwrap_or(&Mode::Positional));
                    let op2 = self.fetch(self.ip + 2, pmodes.get(1).unwrap_or(&Mode::Positional));

                    self.store(
                        self.ip + 3,
                        pmodes.get(2).unwrap_or(&Mode::Positional),
                        op1 + op2,
                    );
                    self.ip += Opcode::Add.param_len() + 1;
                }
                Opcode::Mul => {
                    let op1 = self.fetch(self.ip + 1, pmodes.get(0).unwrap_or(&Mode::Positional));
                    let op2 = self.fetch(self.ip + 2, pmodes.get(1).unwrap_or(&Mode::Positional));
                    self.store(
                        self.ip + 3,
                        pmodes.get(2).unwrap_or(&Mode::Positional),
                        op1 * op2,
                    );
                    self.ip += Opcode::Mul.param_len() + 1;
                }
                Opcode::Input => {
                    if input_index >= stdin.len() {
                        self.waiting_for_input = true;
                        break;
                    }
                    let input: i64 = stdin[input_index];
                    input_index += 1;
                    self.store(
                        self.ip + 1,
                        pmodes.get(0).unwrap_or(&Mode::Positional),
                        input,
                    );
                    self.ip += Opcode::Input.param_len() + 1;
                }
                Opcode::Output => {
                    let op1 = self.fetch(self.ip + 1, pmodes.get(0).unwrap_or(&Mode::Positional));
                    stdout.push(op1);
                    self.ip += Opcode::Output.param_len() + 1;
                }
                Opcode::JumpIfTrue => {
                    let op1 = self.fetch(self.ip + 1, pmodes.get(0).unwrap_or(&Mode::Positional));
                    let op2 = self.fetch(self.ip + 2, pmodes.get(1).unwrap_or(&Mode::Positional));
                    if op1 != 0 {
                        self.ip = op2 as usize;
                        continue;
                    }
                    self.ip += Opcode::JumpIfTrue.param_len() + 1;
                }
                Opcode::JumpIfFalse => {
                    let op1 = self.fetch(self.ip + 1, pmodes.get(0).unwrap_or(&Mode::Positional));
                    let op2 = self.fetch(self.ip + 2, pmodes.get(1).unwrap_or(&Mode::Positional));
                    if op1 == 0 {
                        self.ip = op2 as usize;
                        continue;
                    }
                    self.ip += Opcode::JumpIfFalse.param_len() + 1;
                }
                Opcode::LessThan => {
                    let op1 = self.fetch(self.ip + 1, pmodes.get(0).unwrap_or(&Mode::Positional));
                    let op2 = self.fetch(self.ip + 2, pmodes.get(1).unwrap_or(&Mode::Positional));
                    self.store(
                        self.ip + 3,
                        pmodes.get(2).unwrap_or(&Mode::Positional),
                        if op1 < op2 { 1 } else { 0 },
                    );
                    self.ip += Opcode::LessThan.param_len() + 1;
                }
                Opcode::Equals => {
                    let op1 = self.fetch(self.ip + 1, pmodes.get(0).unwrap_or(&Mode::Positional));
                    let op2 = self.fetch(self.ip + 2, pmodes.get(1).unwrap_or(&Mode::Positional));
                    self.store(
                        self.ip + 3,
                        pmodes.get(2).unwrap_or(&Mode::Positional),
                        if op1 == op2 { 1 } else { 0 },
                    );
                    self.ip += Opcode::Equals.param_len() + 1;
                }
                Opcode::RelativeBaseOffset => {
                    let op1 = self.fetch(self.ip + 1, pmodes.get(0).unwrap_or(&Mode::Positional));
                    self.rel_base += op1;
                    self.ip += Opcode::RelativeBaseOffset.param_len() + 1;
                }
                Opcode::Halt => {
                    self.done = true;
                    break;
                }
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

        computer.load_program(&[1, 0, 0, 0, 99]);
        computer.execute(&[]);
        assert_eq!(computer.program_memory(), vec![2, 0, 0, 0, 99]);

        computer.load_program(&[2, 3, 0, 3, 99]);
        computer.execute(&[]);
        assert_eq!(computer.program_memory(), vec![2, 3, 0, 6, 99]);

        computer.load_program(&[2, 4, 4, 5, 99, 0]);
        computer.execute(&[]);
        assert_eq!(computer.program_memory(), vec![2, 4, 4, 5, 99, 9801]);

        computer.load_program(&[1, 1, 1, 4, 99, 5, 6, 0, 99]);
        computer.execute(&[]);
        assert_eq!(computer.program_memory(), vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
    #[test]
    fn test_io() {
        let mut computer = IntcodeComputer::new();

        computer.load_program(&[3, 0, 4, 0, 99]);
        let output = computer.execute(&[25]);
        assert_eq!(output, vec![25]);
    }
    #[test]
    fn negative_integers() {
        let mut computer = IntcodeComputer::new();

        computer.load_program(&[1101, 100, -1, 4, 0]);
        computer.execute(&[]);
        assert_eq!(computer.program_memory(), vec![1101, 100, -1, 4, 99]);
    }
    #[test]
    fn test_equals_and_less_than() {
        let mut computer = IntcodeComputer::new();

        computer.load_program(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        let output = computer.execute(&[4]);
        assert_eq!(output, vec![0]);

        computer.load_program(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        let output = computer.execute(&[8]);
        assert_eq!(output, vec![1]);

        computer.load_program(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        let output = computer.execute(&[4]);
        assert_eq!(output, vec![1]);

        computer.load_program(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        let output = computer.execute(&[8]);
        assert_eq!(output, vec![0]);

        computer.load_program(&[3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        let output = computer.execute(&[8]);
        assert_eq!(output, vec![1]);

        computer.load_program(&[3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        let output = computer.execute(&[0]);
        assert_eq!(output, vec![0]);

        computer.load_program(&[3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        let output = computer.execute(&[18]);
        assert_eq!(output, vec![0]);

        computer.load_program(&[3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        let output = computer.execute(&[6]);
        assert_eq!(output, vec![1]);
    }
    #[test]
    fn test_jumps() {
        let mut computer = IntcodeComputer::new();

        computer.load_program(&[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]);
        let output = computer.execute(&[4]);
        assert_eq!(output, vec![1]);

        computer.load_program(&[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]);
        let output = computer.execute(&[0]);
        assert_eq!(output, vec![0]);

        computer.load_program(&[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        let output = computer.execute(&[4]);
        assert_eq!(output, vec![1]);

        computer.load_program(&[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        let output = computer.execute(&[0]);
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
        computer.load_program(&program);
        let output = computer.execute(&[4]);
        assert_eq!(output, vec![999]);

        computer.load_program(&program);
        let output = computer.execute(&[8]);
        assert_eq!(output, vec![1000]);

        computer.load_program(&program);
        let output = computer.execute(&[14]);
        assert_eq!(output, vec![1001]);
    }
    #[test]
    fn extra_memory_large_digits() {
        let mut computer = IntcodeComputer::new();

        let program = [
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        computer.load_program(&program);
        let output = computer.execute(&[]);
        assert_eq!(output.as_slice(), program);

        let program = [1102, 34915192, 34915192, 7, 4, 7, 99, 0];
        computer.load_program(&program);
        let output = computer.execute(&[]);
        assert_eq!(output[0].to_string().len(), 16);

        let program = [104, 1125899906842624, 99];
        computer.load_program(&program);
        let output = computer.execute(&[]);
        assert_eq!(output[0], 1125899906842624);
    }
    #[test]
    fn test_relative_mode() {
        let mut computer = IntcodeComputer::new();
        computer.rel_base = 2000;

        let program = [109, 19, 204, -34, 99];
        computer.load_program(&program);
        computer.memory.insert(1985, 123456789);
        let output = computer.execute(&[]);
        assert_eq!(computer.rel_base, 2019);
        assert_eq!(output[0], 123456789);
    }
}
