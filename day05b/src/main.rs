use itertools::Itertools;
use std::cmp::Ordering;
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

    updates
        .lines()
        .map(|l| {
            l.split(",")
                .map(|t| t.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        // `ordering` tells us which pages must come after a given page.
        // To check for violations, search through each update backwards.
        // If we encounter a page that should come after the current page,
        // then this update is invalid.
        // Difference from part 1: Only retaining the invalid elements this time
        .filter(|update| {
            // Iterate over the update backwards
            for (i, page) in update.iter().rev().enumerate() {
                let Some(comes_after) = ordering.get(page) else {
                    // No constraints on this page
                    continue;
                };

                // Iterate again over everything that comes before this
                for earlier_page in update.iter().rev().skip(i + 1) {
                    if comes_after.contains(earlier_page) {
                        // Invalid
                        return true;
                    }
                }
            }

            false
        })
        .map(|mut update| {
            // For this problem to have a single answer, there must be a single valid
            // ordering for the updates. We can get that from the dependency graph.
            // Sort according to that graph (in ascending order)
            update.sort_by(|a, b| {
                if let Some(comes_after_a) = ordering.get(a) {
                    if comes_after_a.contains(b) {
                        return Ordering::Less;
                    }
                } else if let Some(comes_after_b) = ordering.get(b) {
                    if comes_after_b.contains(a) {
                        return Ordering::Greater;
                    }
                }

                // Not necessarily actually equal, but we don't have constraints on their ordering.
                // This is a stable sort, so the original order will be preserved.
                Ordering::Equal
            });

            update
        })
        // Sum up the middle elements
        .map(|update| update[update.len() / 2])
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

        assert_eq!(res, 123);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 6204);
    }
}
