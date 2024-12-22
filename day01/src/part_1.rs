pub fn solution(input: &str) -> i64 {
    let (mut first, mut second): (Vec<i64>, Vec<i64>) = input
        .lines()
        .map(|line| {
            let mut parts = line
                .trim()
                .split_whitespace()
                .map(|t| t.parse::<i64>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .unzip();

    first.sort();
    second.sort();

    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 11);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 1889772);
    }
}
