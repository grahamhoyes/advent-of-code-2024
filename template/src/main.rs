mod part_1;
mod part_2;

fn main() {
    // Usage: cargo run -- <part> <input|example>
    let args: Vec<String> = std::env::args().collect();
    let part = args.get(1).expect("No part provided");
    let input = args.get(2).expect("No input file provided").clone() + ".txt";

    let input = std::fs::read_to_string(input).expect("Failed to read input file");

    let res = match part.as_str() {
        "1" => part_1::solution(&input),
        "2" => part_2::solution(&input),
        _ => panic!("Invalid part provided"),
    };

    println!("Result: {}", res);
}
