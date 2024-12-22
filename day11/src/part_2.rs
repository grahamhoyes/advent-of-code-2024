use std::collections::HashMap;

pub fn solution(input: &str) -> usize {
    // Instead of using a vector as in part 1, just store the count of each stone
    // since the order doesn't matter (we never re-combine, I figured that would be
    // the part 2 twist).
    let mut stone_counts: HashMap<u64, usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .fold(HashMap::new(), |mut acc, val| {
            *acc.entry(val).or_insert(0) += 1;
            acc
        });

    for _ in 0..75 {
        let mut next_iteration = HashMap::new();

        for (value, count) in stone_counts {
            let stone_len = value.checked_ilog10().unwrap_or(0) + 1;

            if value == 0 {
                *next_iteration.entry(1).or_insert(0) += count;
            } else if stone_len & 1 == 0 {
                // Split at the midpoint, using math
                let split_point = 10u64.pow(stone_len / 2);
                let left = value / split_point;
                let right = value % split_point;

                *next_iteration.entry(left).or_insert(0) += count;
                *next_iteration.entry(right).or_insert(0) += count;
            } else {
                *next_iteration.entry(value * 2024).or_insert(0) += count;
            }
        }

        stone_counts = next_iteration;
    }

    stone_counts.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 65601038650482);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 205913561055242);
    }
}
