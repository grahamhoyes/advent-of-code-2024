use crate::part_1;
use crate::part_1::Op;
use std::time::Instant;

/// Decompile a program to assembly and print it
fn decompile(program: &[u8]) {
    let combo_args = ['0', '1', '2', '3', 'a', 'b', 'c'];

    for i in (0..program.len()).step_by(2) {
        let op: Op = program[i].into();
        let arg = match op {
            Op::Adv | Op::Bst | Op::Out | Op::Bdv | Op::Cdv => combo_args[program[i + 1] as usize],
            _ => (program[i + 1] + b'0') as char,
        };

        let code = match op {
            Op::Adv => format!("a = a >> {arg}"),
            Op::Bxl => format!("b = b ^ {arg}"),
            Op::Bst => format!("b = {arg} & 7"),
            Op::Jnz => format!("if a > 0 goto {arg}"),
            Op::Bxc => "b = b ^ c".to_string(),
            Op::Out => format!("output({arg} & 7)"),
            Op::Bdv => format!("b = a >> {arg}"),
            Op::Cdv => format!("c = a >> {arg}"),
        };

        println!("{op} {arg}  |  {code}");
    }
}

#[allow(dead_code)]
const LOWER_BOUND: u64 = 1 << 45;

#[allow(dead_code)]
const UPPER_BOUND: u64 = 1 << 48;

/// Exhaustive single-threaded solution over the valid range.
///
/// Runs at around 5_579_985 iterations / second on M4 pro.
/// Would complete in at most 510 days.
#[allow(dead_code)]
pub fn solution_exhaustive_sequential(program: &[u8]) -> u64 {
    let start = Instant::now();
    let mut last_progress = Instant::now();

    for a_reg in LOWER_BOUND..UPPER_BOUND {
        let output = part_1::run_program(program, a_reg, 0, 0);

        if output == program {
            return a_reg;
        }

        if a_reg % 1_000_000 == 0 {
            let now = Instant::now();
            println!(
                "Iteration {}. This batch: {:.2} it/sec. Average: {:.2} it/sec.",
                a_reg,
                1_000_000f64 / now.duration_since(last_progress).as_secs_f64(),
                (a_reg - LOWER_BOUND) as f64 / now.duration_since(start).as_secs_f64(),
            );
            last_progress = now;
        }
    }

    0
}

/// Exhaustive parallel solution over the valid range using Rayon
///
/// Runs at around 43_338_348 iterations / second on M4 pro (12 cores).
/// Would complete in at most 66 days.
#[allow(dead_code)]
pub fn solution_exhaustive_parallel(program: &[u8]) -> u64 {
    use rayon::prelude::*;

    // Limiting iterations just to get a benchmark. For an actual solution,
    // the range below should be (LOWER_BOUND..UPPER_BOUND)
    const ITERATIONS: u64 = 2_000_000_000;

    let start = Instant::now();
    let a_reg = (LOWER_BOUND..LOWER_BOUND + ITERATIONS)
        .into_par_iter()
        .find_map_first(|a_reg| {
            let output = part_1::run_program(program, a_reg, 0, 0);
            if output == program {
                Some(a_reg)
            } else {
                None
            }
        })
        .unwrap_or(0);
    let end = Instant::now();

    let it_per_sec = ITERATIONS as f64 / end.duration_since(start).as_secs_f64();
    println!("{:.2} it/sec", it_per_sec);

    a_reg
}

/// Find the value of the a register that makes the input program a quine
///
/// This is not a general solution, it relies on specific observations about
/// this input that can be seen in the decompiled output:
/// - The program is a loop while a!=0. Each iteration, a is divided by 8.
/// - The loop needs to run exactly 16 times, which puts bounds on the value of a
///   (used in the exhaustive solutions above, not here)
/// - a is the only value that persists between iterations. b and c depend only on
///   the value of a at the start of an iteration
/// - By reducing all the operations, and working through the program backwards:
///   - The final output value depends only on the originally highest (now lowest) 3 bits of a
///   - The second output value depends only on the originally highest (now lowest) 6 bits
///   - Etc
/// - We can thus work backwards through the program, shifting a left by 3 and trying
///   all combinations for the new lower 3 bits.
/// - Each combination that produces the desired sequence is tracked, as there can be
///   multiple options at this point that lead to a dead end later on.
pub fn solution_smart(program: &[u8]) -> u64 {
    let mut possible_a_values = vec![0];

    // Work backwards through the program, changing only the lowest bits of a
    for &target in program.iter().rev() {
        let mut new_candidates = Vec::new();

        while let Some(candidate) = possible_a_values.pop() {
            for i in 0..8 {
                let test_a = (candidate << 3) + i;
                let output = part_1::run_program(program, test_a, 0, 0);

                // There are possibly multiple valid set of bits that lead to
                // a working solution at this point. We need to track all of them
                // in case one leads to a dead end in the search later on.
                if *output.first().unwrap() == target {
                    new_candidates.push(test_a);
                }
            }
        }

        possible_a_values.extend(new_candidates);
    }

    possible_a_values.into_iter().min().unwrap()
}

pub fn solution(input: &str) -> String {
    let (program, _, _, _) = part_1::parse_input(input);

    decompile(&program);

    // Returning a string just since the runner needs this to be
    // the same type as part_1::solution
    // solution_exhaustive_sequential(&program).to_string();
    solution_exhaustive_parallel(&program).to_string();
    solution_smart(&program).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example_2.txt");
        let res = solution(input);

        assert_eq!(res, "117440");
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, "216148338630253");
    }
}
