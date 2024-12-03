use regex::{Regex, RegexBuilder};

fn solution(input: &str) -> i32 {
    let instructions_re = RegexBuilder::new(r"mul\(\d{1,3},\d{1,3}\)")
        .multi_line(true)
        .build()
        .unwrap();

    let operands_re = Regex::new(r"(?<l>\d{1,3}),(?<r>\d{1,3})").unwrap();

    instructions_re
        .find_iter(input)
        .map(|instr| {
            let ops = operands_re.captures(instr.as_str()).unwrap();
            ops["l"].parse::<i32>().unwrap() * ops["r"].parse::<i32>().unwrap()
        })
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    let res = solution(input);

    println!("Result: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 161);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 166905464);
    }
}
