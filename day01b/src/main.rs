use std::collections::HashMap;

fn solution(input: &str) -> u64 {
    let (first, second): (Vec<u64>, Vec<u64>) = input
        .lines()
        .map(|line| {
            let mut parts = line
                .trim()
                .split_whitespace()
                .map(|t| t.parse::<u64>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .unzip();

    // Convert the second list into a map of occurrence counts
    let mut occurrences: HashMap<u64, u64> = HashMap::new();

    for num in second.into_iter() {
        *occurrences.entry(num).or_insert(0) += 1;
    }

    first
        .into_iter()
        .map(|num| {
            let occ = occurrences.get(&num).unwrap_or(&0);
            num * occ
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

        assert_eq!(res, 31);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 23228917);
    }
}
