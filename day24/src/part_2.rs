use crate::part_1::{Gate, Network};
use itertools::Itertools;

pub enum AdderValidation {
    Valid { sum: String, carry: String },
    Invalid { gate: String },
}

/// Validate that inputs a, b, and c_in are tied to a correct full adder.
///
/// Outputs the name of the sum output and the carry output.
pub fn validate_full_adder(network: &Network, a: String, b: String, c_in: &str) -> AdderValidation {
    println!("Validating full adder: {} + {} + {}", a, b, c_in);
    // Sum output bit
    let Some(first_xor_output) = network.find_gate(&a, &b, Gate::Xor) else {
        return AdderValidation::Invalid {
            gate: "Sum first XOR gate".to_string(),
        };
    };

    let Some(sum_output) = network.find_gate(&first_xor_output, c_in, Gate::Xor) else {
        return AdderValidation::Invalid {
            gate: "Sum second XOR gate".to_string(),
        };
    };

    // Carry block
    let Some(first_and) = network.find_gate(&first_xor_output, c_in, Gate::And) else {
        return AdderValidation::Invalid {
            gate: "Carry first AND gate".to_string(),
        };
    };

    let Some(second_and) = network.find_gate(&a, &b, Gate::And) else {
        return AdderValidation::Invalid {
            gate: "Carry second AND gate".to_string(),
        };
    };

    let Some(carry_output) = network.find_gate(&first_and, &second_and, Gate::Or) else {
        return AdderValidation::Invalid {
            gate: format!("Carry OR gate. Expecting {} OR {}", first_and, second_and),
        };
    };

    AdderValidation::Valid {
        sum: sum_output.clone(),
        carry: carry_output.clone(),
    }
}

/// Validate that inputs a and b are tied to a correct half adder.
///
/// Outputs the name of the sum output and the carry output.
pub fn validate_half_adder(network: &Network, a: &str, b: &str) -> Option<(String, String)> {
    let sum_output = network
        .find_gate(a, b, Gate::Xor)
        .expect("Failed to find sum output");
    let carry_output = network
        .find_gate(a, b, Gate::And)
        .expect("Failed to find carry output");

    Some((sum_output, carry_output))
}

/// This is not an automated solution.
///
/// It relies on the fact that the actual input (but not the example) is structured
/// as a sequence of full adders. Using the above validation functions, we can walk
/// through the adder network and validate that each stage is structured properly.
///
/// When an invalid stage is detected, we break and print the gate that failed.
/// By inspecting the stage in the printed mermaid diagram, we can pretty easily
/// see which gates need to be swapped. Those gates are added to `to_swap`, repeating
/// manually until the full network is valid.
pub fn solution(input: &str) -> String {
    let mut network = Network::from_input(input);

    let to_swap = [
        ("z10", "kmb"),
        ("z15", "tvp"),
        ("dpg", "z25"),
        ("vdk", "mmf"),
    ];

    for (wire1, wire2) in to_swap {
        network.swap_outputs(wire1, wire2);
    }

    network
        .write_mermaid("diagram.mermaid")
        .expect("Failed to write mermaid file");

    // Traverse the network, validating that it makes a proper adder
    let (_sum, mut carry) =
        validate_half_adder(&network, "x00", "y00").expect("Initial half adder is invalid");

    // Loop, validating full adders
    for i in 1..45 {
        let a = format!("x{:0>2}", i);
        let b = format!("y{:0>2}", i);

        match validate_full_adder(&network, a, b, &carry) {
            AdderValidation::Valid {
                carry: new_carry, ..
            } => {
                carry = new_carry;
            }
            AdderValidation::Invalid { gate } => {
                panic!("Failed to validate full adder at bit {}: {}", i, gate);
            }
        }
    }

    to_swap
        .iter()
        .flat_map(|(a, b)| [a.to_string(), b.to_string()])
        .sorted()
        .join(",")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_example() {
    //     let input = include_str!("../example.txt");
    //     let res = solution(input);
    //
    //     assert_eq!(res, "");
    // }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, "dpg,kmb,mmf,tvp,vdk,z10,z15,z25");
    }
}
