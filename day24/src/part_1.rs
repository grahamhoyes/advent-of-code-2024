use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Gate {
    And,
    Or,
    Xor,
}

impl std::fmt::Display for Gate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Gate::And => write!(f, "AND"),
            Gate::Or => write!(f, "OR"),
            Gate::Xor => write!(f, "XOR"),
        }
    }
}

#[derive(Clone)]
pub struct Network {
    pub values: HashMap<String, u8>,
    pub dependencies: HashMap<String, (String, Gate, String)>,
    // Similar to dependencies, except the two strings are sorted
    pub gates: HashMap<(String, Gate, String), String>,
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

        let dependencies: HashMap<_, _> = raw_network
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

        let gates = dependencies
            .iter()
            .map(|(output, (lhs, gate, rhs))| {
                // Sort so the smallest one is first
                let (lhs, rhs) = if lhs < rhs {
                    (lhs.clone(), rhs.clone())
                } else {
                    (rhs.clone(), lhs.clone())
                };

                ((lhs, gate.clone(), rhs), output.clone())
            })
            .collect();

        Self {
            values,
            dependencies,
            gates,
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
    pub fn evaluate(&mut self, max_iterations: usize) -> Option<u64> {
        for _ in 0..max_iterations {
            if self.feed_forward_once() {
                return Some(self.get_output());
            }
        }

        None
    }

    #[allow(dead_code)]
    /// Write the network as a mermaid graph to a file
    pub fn write_mermaid(&self, file: &str) -> std::io::Result<()> {
        let mut file = File::create(file)?;

        writeln!(file, "graph LR;")?;

        let input_wires = self
            .values
            .keys()
            .map(|key| key.to_string())
            .sorted()
            .collect_vec();

        // Print each input node
        for wire in input_wires {
            if wire.starts_with("x") || wire.starts_with("y") {
                // Inputs get a rounded rectangle
                writeln!(file, "{}([{}]);", wire, wire)?;
            }
        }

        for (output, (lhs, gate, rhs)) in self.dependencies.iter() {
            if output.starts_with("z") {
                // Make final outputs look distinct
                writeln!(file, "{}[[{} {} {} -> {}]]", output, lhs, gate, rhs, output)?;
            } else {
                writeln!(file, "{}({} {} {} -> {})", output, lhs, gate, rhs, output)?;
            }

            // Edges pointing in to the gate
            writeln!(file, "{} --> {}", lhs, output)?;
            writeln!(file, "{} --> {}", rhs, output)?;
        }

        Ok(())
    }

    pub fn bits_to_val(&self, prefix: &str) -> u64 {
        self.values
            .iter()
            .filter(|(wire, _)| wire.starts_with(prefix))
            .sorted()
            .rev()
            .fold(0, |acc, (_, val)| (acc << 1) | (*val as u64))
    }

    /// Get the x and y inputs
    #[allow(dead_code)]
    pub fn get_inputs(&self) -> (u64, u64) {
        (self.bits_to_val("x"), self.bits_to_val("y"))
    }

    /// Get the output from the z wires
    pub fn get_output(&self) -> u64 {
        self.bits_to_val("z")
    }

    pub fn set_input(&mut self, name: &str, value: u64) {
        for (key, val) in self.values.iter_mut() {
            if key.starts_with(name) {
                // Strip the prefix
                let bit_place = key.strip_prefix(name).unwrap().parse::<usize>().unwrap();

                *val = ((value >> bit_place) & 1) as u8;
            }
        }
    }

    /// Swap the output wires of two gates
    pub fn swap_outputs(&mut self, wire1: &str, wire2: &str) {
        let a = self.dependencies.get(wire1).unwrap().clone();
        let b = self.dependencies.get(wire2).unwrap().clone();

        self.dependencies.insert(wire1.to_string(), b.clone());
        self.dependencies.insert(wire2.to_string(), a.clone());

        // Swap the gates too, getting the order right
        self.set_gate(&a.0, &a.2, a.1, wire2);
        self.set_gate(&b.0, &b.2, b.1, wire1);
    }

    pub fn find_gate(&self, a: &str, b: &str, gate: Gate) -> Option<String> {
        // Sort the inputs to match how we store them
        let (a, b) = if a < b { (a, b) } else { (b, a) };

        self.gates
            .get(&(a.to_string(), gate, b.to_string()))
            .cloned()
    }

    pub fn set_gate(&mut self, a: &str, b: &str, gate: Gate, output: &str) {
        let (a, b) = if a < b { (a, b) } else { (b, a) };

        self.gates
            .insert((a.to_string(), gate, b.to_string()), output.to_string());
    }
}

pub fn solution(input: &str) -> String {
    let mut network = Network::from_input(input);

    network.evaluate(100).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, "2024");
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, "51715173446832");
    }
}
