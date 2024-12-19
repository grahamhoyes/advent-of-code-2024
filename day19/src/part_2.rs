use std::collections::HashMap;

pub fn count_possible_arrangements<'a>(
    design: &'a str,
    patterns: &[&str],
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(&num) = memo.get(design) {
        return num;
    }

    let num_possible: usize = patterns
        .iter()
        .map(|&pattern| {
            if pattern.len() > design.len() {
                return 0;
            }

            let (start, rest) = design.split_at(pattern.len());

            if start == pattern {
                count_possible_arrangements(rest, patterns, memo)
            } else {
                0
            }
        })
        .sum();

    memo.insert(design, num_possible);
    num_possible
}

pub fn solution(input: &str) -> usize {
    let (patterns, designs) = input.trim().split_once("\n\n").unwrap();

    let patterns: Vec<&str> = patterns.split(", ").collect();
    let designs: Vec<&str> = designs.lines().collect();

    let mut memo: HashMap<&str, usize> = HashMap::new();

    designs
        .into_iter()
        .map(|d| count_possible_arrangements(d, &patterns, &mut memo))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 16);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 724388733465031);
    }
}
