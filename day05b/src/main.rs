use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn solution(input: &str) -> u32 {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    // Map from page to pages that must come after it
    let mut ordering: HashMap<u32, HashSet<u32>> = HashMap::new();

    for line in rules.lines() {
        let (first, second) = line
            .split("|")
            .map(|t| t.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();

        ordering.entry(first).or_default().insert(second);
    }

    let mut updates: Vec<Vec<u32>> = updates
        .lines()
        .map(|l| l.split(",").map(|t| t.parse::<u32>().unwrap()).collect())
        .collect();

    // `ordering` tells us which pages must come after a given page.
    // To check for violations, search through each update backwards.
    // If we encounter a page that should come after the current page,
    // then this update is invalid.

    updates.retain(|update| {
        // Iterate over the update backwards
        for (i, page) in update.iter().rev().enumerate() {
            let Some(comes_after) = ordering.get(page) else {
                // No constraints on this page
                continue;
            };

            // Iterate again over everything that comes before this
            for earlier_page in update.iter().rev().skip(i + 1) {
                if comes_after.contains(earlier_page) {
                    return false;
                }
            }
        }

        true
    });

    // Sum up the middle elements
    updates.iter().map(|update| update[update.len() / 2]).sum()
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

        assert_eq!(res, 143);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 4905);
    }
}
