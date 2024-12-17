use itertools::Itertools;

#[derive(Debug)]
#[repr(u8)]
#[allow(dead_code)]
enum Op {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl From<u8> for Op {
    fn from(value: u8) -> Self {
        if value <= 7 {
            unsafe { std::mem::transmute::<u8, Op>(value) }
        } else {
            panic!("Invalid value {value}")
        }
    }
}

pub fn run_program(program: &Vec<u8>, mut a_reg: u32, mut b_reg: u32, mut c_reg: u32) -> String {
    // Instruction pointer
    let mut ip: usize = 0;

    let mut output: Vec<u32> = Vec::new();

    loop {
        let Some(&op) = program.get(ip) else {
            break;
        };
        let op: Op = op.into();

        let Some(&arg) = program.get(ip + 1) else {
            break;
        };

        let combo_arg = match arg {
            0..=3 => arg as u32,
            4 => a_reg,
            5 => b_reg,
            6 => c_reg,
            7 => panic!("Invalid operand"),
            _ => unreachable!(),
        };

        match op {
            Op::Adv => {
                // a_reg = a_reg / 2^arg
                a_reg /= 1u32 << combo_arg;
            }
            Op::Bxl => {
                b_reg ^= arg as u32;
            }
            Op::Bst => {
                b_reg = combo_arg & 7;
            }
            Op::Jnz => {
                if a_reg > 0 {
                    ip = arg as usize;
                    continue;
                }
            }
            Op::Bxc => {
                b_reg ^= c_reg;
            }
            Op::Out => {
                output.push(combo_arg & 7);
            }
            Op::Bdv => {
                b_reg = a_reg / (1u32 << combo_arg);
            }
            Op::Cdv => {
                c_reg = a_reg / (1u32 << combo_arg);
            }
        }

        ip += 2;
    }

    output.into_iter().join(",")
}

pub fn parse_input(input: &str) -> (Vec<u8>, u32, u32, u32) {
    let (registers, program) = input.trim().split_once("\n\n").unwrap();

    // Registers, u32
    let (a_reg, b_reg, c_reg) = registers
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse::<u32>().unwrap())
        .collect_tuple()
        .unwrap();

    // Program
    let program: Vec<u8> = program
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|num| num.parse::<u8>().unwrap())
        .collect();

    (program, a_reg, b_reg, c_reg)
}

pub fn solution(input: &str) -> String {
    let (program, a_reg, b_reg, c_reg) = parse_input(input);

    run_program(&program, a_reg, b_reg, c_reg)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, "1,6,7,4,3,0,5,0,6");
    }
}
