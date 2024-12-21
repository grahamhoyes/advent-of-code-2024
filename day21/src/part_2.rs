use crate::part_1::{find_global_shortest_sequence, Key, Paths};
use std::collections::HashMap;

pub fn solution(input: &str) -> usize {
    let (numpad_paths, dpad_paths) = crate::part_1::get_keypad_paths();

    let mut cache = HashMap::new();

    let mut paths: Vec<&Paths> = vec![&numpad_paths];
    // 25 layers of dpads
    for _ in 0..25 {
        paths.push(&dpad_paths);
    }

    input
        .lines()
        .map(|sequence| {
            let numpad_sequence: Vec<Key> = sequence.chars().map(|c| c.into()).collect();
            let numeric_value = sequence[0..3].parse::<usize>().unwrap();

            let len = find_global_shortest_sequence(&paths, &numpad_sequence, 0, &mut cache);

            len * numeric_value
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

        assert_eq!(res, 154115708116294);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 159684145150108);
    }
}
