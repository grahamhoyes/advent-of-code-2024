fn solution(input: &str) -> usize {
    let mut stones: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for _ in 0..25 {
        let mut next_iteration: Vec<u64> = Vec::with_capacity(stones.len());

        for stone in stones {
            let stone_len = stone.checked_ilog10().unwrap_or(0) + 1;

            if stone == 0 {
                next_iteration.push(1);
            } else if stone_len & 1 == 0 {
                // Split at the midpoint, using math
                let split_point = 10u64.pow(stone_len / 2);
                let left = stone / split_point;
                let right = stone % split_point;

                next_iteration.push(left);
                next_iteration.push(right);
            } else {
                next_iteration.push(stone * 2024);
            }
        }

        stones = next_iteration;
    }

    stones.len()
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

        assert_eq!(res, 55312);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 172484);
    }
}
