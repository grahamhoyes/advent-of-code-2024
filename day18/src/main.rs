mod part_1;
mod part_2;

fn main() {
    // Usage: cargo run -- <part> <input|example>
    let args: Vec<String> = std::env::args().collect();
    let part = args.get(1).expect("No part provided");
    let input = args.get(2).expect("No input file provided").clone() + ".txt";

    let (board_size, initial_drop) = match input.as_str() {
        "example.txt" => ((7, 7), 12),
        "input.txt" => ((71, 71), 1024),
        i => panic!("Invalid input provided: {}", i),
    };

    let input = std::fs::read_to_string(input).expect("Failed to read input file");

    match part.as_str() {
        "1" => {
            let res = part_1::solution(&input, board_size, initial_drop);
            println!("Result: {}", res);
        }
        "2" => {
            let res = part_2::solution(&input, board_size, initial_drop);
            println!("Result: {:?}", res);
        }
        _ => panic!("Invalid part provided"),
    };
}
