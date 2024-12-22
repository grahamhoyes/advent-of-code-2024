use crate::part_1::iterate_secret;
use std::collections::{HashMap, HashSet};

/// Get the possible sequences of changes and their corresponding prices
pub fn get_sequence_prices(mut secret: u64) -> HashMap<[i8; 4], i8> {
    let mut prices: Vec<i8> = Vec::with_capacity(2000);
    let mut changes: Vec<i8> = Vec::with_capacity(2000);

    let mut previous_price = (secret % 10) as i8;

    for _ in 0..2000 {
        secret = iterate_secret(secret);
        let price = (secret % 10) as i8;
        prices.push(price);

        let change = price - previous_price;
        changes.push(change);

        previous_price = price;
    }

    let mut sequence_values: HashMap<[i8; 4], i8> = HashMap::new();

    for i in 3..prices.len() {
        let sequence: [i8; 4] = changes[i - 3..=i].try_into().unwrap();
        let price = prices[i];

        // Only track the first occurrence
        sequence_values.entry(sequence).or_insert(price);
    }

    sequence_values
}

pub fn solution(input: &str) -> u64 {
    let sequence_prices: Vec<HashMap<[i8; 4], i8>> = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .map(get_sequence_prices)
        .collect();

    // Union together all the possible sequences
    let all_sequences: HashSet<[i8; 4]> = sequence_prices
        .iter()
        .flat_map(|prices| prices.keys().copied())
        .collect();

    // Find the sequence that results in the highest total price
    all_sequences
        .into_iter()
        .map(|sequence| {
            sequence_prices
                .iter()
                .map(move |price_map| *price_map.get(&sequence).unwrap_or(&0) as u64)
                .sum()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 23);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 1854);
    }
}
