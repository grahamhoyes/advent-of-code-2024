use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Gate {
    And,
    Or,
    Xor,
}

pub struct Network {
    pub values: HashMap<String, u8>,
    pub dependencies: HashMap<String, (String, Gate, String)>,
}

impl Network {
    pub fn from_input(input: &str) -> Self {
        let (inputs, raw_network) = input.split_once("\n\n").unwrap();

        let values = inputs
            .lines()
            .map(|line| {
                let (name, value) = line.split_once(": ").unwrap();

                (name.to_string(), value.parse().unwrap())
            })
            .collect();

        let dependencies = raw_network
            .lines()
            .map(|line| {
                let (op, output) = line.split_once(" -> ").unwrap();

                let (lhs, gate, rhs) = op.split(" ").collect_tuple().unwrap();

                let gate = match gate {
                    "AND" => Gate::And,
                    "OR" => Gate::Or,
                    "XOR" => Gate::Xor,
                    _ => panic!("Unrecognized gate {}", gate),
                };

                (output.to_string(), (lhs.to_string(), gate, rhs.to_string()))
            })
            .collect();

        Self {
            values,
            dependencies,
        }
    }

    /// Feed values in the network forward once.
    ///
    /// Returns true if the network was fully satisfied, or false if
    /// another iteration is required.
    pub fn feed_forward_once(&mut self) -> bool {
        let mut satisfied = true;

        for (output, (lhs, gate, rhs)) in self.dependencies.iter() {
            if self.values.contains_key(output) {
                continue;
            }

            if let (Some(lhs_val), Some(rhs_val)) = (self.values.get(lhs), self.values.get(rhs)) {
                let res = match gate {
                    Gate::And => lhs_val & rhs_val,
                    Gate::Or => lhs_val | rhs_val,
                    Gate::Xor => lhs_val ^ rhs_val,
                };

                self.values.insert(output.clone(), res);
            } else {
                satisfied = false;
            }
        }

        satisfied
    }

    /// Evaluate the network
    pub fn evaluate(&mut self) {
        while !self.feed_forward_once() {}
    }

    /// Get the output from the z wires
    pub fn get_output(&self) -> u64 {
        let output_wires = self
            .values
            .keys()
            .filter(|wire| wire.starts_with("z"))
            .sorted()
            .rev();

        let mut output: u64 = 0;

        for wire in output_wires {
            output = (output << 1) | (self.values[wire] as u64);
        }

        output
    }
}

pub fn solution(input: &str) -> u64 {
    let mut network = Network::from_input(input);

    network.evaluate();

    network.get_output()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 2024);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 51715173446832);
    }
}
