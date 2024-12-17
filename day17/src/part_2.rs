use crate::part_1;
use std::time::Instant;

pub fn solution(input: &str) -> String {
    let (program, _, b_reg, c_reg) = part_1::parse_input(input);

    let mut a_reg = 0;
    let start = Instant::now();
    let mut last_progress = Instant::now();

    loop {
        let output = part_1::run_program(&program, a_reg, b_reg, c_reg);

        if output == program {
            return a_reg.to_string();
        }

        a_reg += 1;

        if a_reg % 1_000_000 == 0 {
            let now = Instant::now();
            println!(
                "Iteration {}. This iteration: {:.2} values/sec. Total: {:.2} values/sec.",
                a_reg,
                1_000_000f64 / now.duration_since(last_progress).as_secs_f64(),
                a_reg as f64 / now.duration_since(start).as_secs_f64(),
            );
            last_progress = now;
        }
    }
}

// 756806.62   initially
// 1427209.56  with vec equality
// 1487361.60  arg lookup in the loop
// 1897771.91

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example_2.txt");
        let res = solution(input);

        assert_eq!(res, "");
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, "");
    }
}
