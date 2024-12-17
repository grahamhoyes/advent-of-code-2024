use crate::part_1;
use itertools::Itertools;

pub fn solution(input: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example_2.txt");
        let res = solution(input);

        assert_eq!(res, "");
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, "");
    }
}
