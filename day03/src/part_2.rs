use regex::{Regex, RegexBuilder};

pub fn solution(input: &str) -> i32 {
    let instructions_re = RegexBuilder::new(r"(?:mul\(\d{1,3},\d{1,3}\))|(?:don't\(\))|(?:do\(\))")
        .multi_line(true)
        .build()
        .unwrap();

    let operands_re = Regex::new(r"(?<l>\d{1,3}),(?<r>\d{1,3})").unwrap();

    let mut enabled = true;

    instructions_re
        .find_iter(input)
        .map(|instr| {
            let instr = instr.as_str();
            if instr.starts_with("mul") {
                if !enabled {
                    return 0;
                }

                let ops = operands_re.captures(instr).unwrap();
                return ops["l"].parse::<i32>().unwrap() * ops["r"].parse::<i32>().unwrap();
            } else {
                enabled = !instr.starts_with("don't");
            }

            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 48);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 72948684);
    }
}
