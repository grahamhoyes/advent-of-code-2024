use crate::part_1::{Gate, Network};
use itertools::Itertools;

pub fn solution(input: &str) -> String {
    let original_network = Network::from_input(input);

    let (x, y) = original_network.get_inputs();
    let target = x + y;

    // All possible pairs of wires that could be swapped
    let wire_pairs: Vec<(&String, &String)> = original_network
        .dependencies
        .keys()
        .tuple_combinations()
        .collect();

    for (i, eight_wires) in original_network
        .dependencies
        .keys()
        .combinations(8)
        .enumerate()
    {
        let mut network = original_network.clone();

        for i in (0..7).step_by(2) {
            let wire1 = eight_wires[i];
            let wire2 = eight_wires[i + 1];

            let a = network.dependencies.get_mut(wire1).unwrap() as *mut _;
            let b = network.dependencies.get_mut(wire2).unwrap() as *mut _;

            unsafe {
                std::ptr::swap(a, b);
            }
        }

        if let Some(output) = network.evaluate(100) {
            if output == target {
                let swapped = eight_wires.into_iter().sorted().collect_vec();

                return swapped.into_iter().sorted().join(",");
            }
        }

        if i % 1_000 == 0 {
            println!("{}: {:?}", i, eight_wires);
        }
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
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
