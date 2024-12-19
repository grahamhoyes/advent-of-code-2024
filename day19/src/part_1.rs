use std::collections::HashMap;

pub fn is_design_possible<'a>(
    design: &'a str,
    patterns: &[&str],
    memo: &mut HashMap<&'a str, bool>,
) -> bool {
    if design.is_empty() {
        return true;
    }

    if let Some(&possible) = memo.get(design) {
        return possible;
    }

    for &pattern in patterns {
        if pattern.len() > design.len() {
            continue;
        }

        let (start, rest) = design.split_at(pattern.len());

        if start == pattern && is_design_possible(rest, patterns, memo) {
            memo.insert(design, true);
            return true;
        }
    }

    memo.insert(design, false);
    false
}

pub fn solution(input: &str) -> usize {
    let (patterns, designs) = input.trim().split_once("\n\n").unwrap();

    let patterns: Vec<&str> = patterns.split(", ").collect();
    let designs: Vec<&str> = designs.lines().collect();

    let mut memo: HashMap<&str, bool> = HashMap::new();

    designs
        .into_iter()
        .map(|d| {
            if is_design_possible(d, &patterns, &mut memo) {
                1
            } else {
                0
            }
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

        assert_eq!(res, 6);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 367);
    }
}
