use std::fs;

fn intcode_computer(intcode: &mut Vec<i32>) {
    let mut index = 0;
    while intcode[index] != 99 {
        let first_pos: usize = intcode[index + 1] as usize;
        let second_pos: usize = intcode[index + 2] as usize;
        let third_pos: usize = intcode[index + 3] as usize;

        if intcode[index] == 1 {
            intcode[third_pos] = intcode[first_pos] + intcode[second_pos];
        } else if intcode[index] == 2 {
            intcode[third_pos] = intcode[first_pos] * intcode[second_pos];
        } else {
            panic!("Invalid opcode");
        }

        index = index + 4;
    }
}

fn find_noun_verb(intcode: &Vec<i32>, output: i32) -> (i32, i32) {
    for i in 0..99 {
        for j in 0..99 {
            let mut ic = intcode.clone();
            ic[1] = i;
            ic[2] = j;

            intcode_computer(&mut ic);
            if ic[0] == output {
                return (ic[1], ic[2]);
            }
        }
    }

    return (-1, -1);
}

fn main() {
    // load input file
    let input: String =
        fs::read_to_string("input/02.txt").expect("Unable to read from file: input/02.txt");
    let intcode: Vec<i32> = input
        .split(',')
        .map(|x| x.trim().parse::<i32>().expect("unable to convert to int"))
        .collect();

    let mut intcode_p1 = intcode.clone();

    intcode_p1[1] = 12;
    intcode_p1[2] = 2;

    intcode_computer(&mut intcode_p1);

    println!("{}", intcode_p1[0]);

    let (noun, verb) = find_noun_verb(&intcode.clone(), 19690720);
    println!("{}", 100 * noun + verb);
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
