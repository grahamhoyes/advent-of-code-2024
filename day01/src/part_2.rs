use std::collections::HashMap;

pub fn solution(input: &str) -> i64 {
    let (first, second): (Vec<i64>, Vec<i64>) = input
        .lines()
        .map(|line| {
            let mut parts = line
                .trim()
                .split_whitespace()
                .map(|t| t.parse::<i64>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .unzip();

    // Convert the second list into a map of occurrence counts
    let mut occurrences: HashMap<i64, i64> = HashMap::new();

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
