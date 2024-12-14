pub fn solution(_input: &str, _board_size: (i32, i32)) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input, (11, 7));

        assert_eq!(res, 0);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input, (101, 103));

        assert_eq!(res, 0);
    }
}
