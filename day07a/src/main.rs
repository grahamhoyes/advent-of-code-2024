/// A calibration value is valid if there's some combination of + and *
/// operators between the measurements (left to right, ignoring order of operations)
/// that combines them to the value.
fn valid_calibration(value: u64, measurements: &[u64]) -> bool {
    // Since there are only two possible operators, we can use bitmaps to get
    // all the possible combinations
    let num_operator_slots = (measurements.len() - 1) as u32;

    for bitmap in 0..2u32.pow(num_operator_slots) {
        let mut total = measurements[0];
        print!("{value} : {total} ");

        // iterate over each bit in the bitmap
        for i in 0..num_operator_slots {
            let operator = if bitmap & (1 << i) == 0 { '+' } else { '*' };
            let right = measurements[i as usize + 1];
            print!("{operator} {right} ");

            total = match operator {
                '+' => total + right,
                '*' => total * right,
                _ => unreachable!(),
            };

            // if total > value {
            //     return false;
            // }
        }

        print!("= {total} ");

        if total == value {
            println!("PASS");
            println!("===PASS===\n");
            return true;
        } else {
            println!("FAIL")
        }
    }

    println!("===FAIL===\n");
    false
}

fn solution(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (value, measurements) = l.split_once(':').unwrap();
            let value = value.parse::<u64>().unwrap();
            let measurements: Vec<u64> = measurements
                .split_whitespace()
                .map(|t| t.parse::<u64>().unwrap())
                .collect();

            (value, measurements)
        })
        .filter(|(value, measurements)| valid_calibration(*value, measurements))
        .map(|(value, _)| value)
        .sum()
}

fn main() {
    let input = include_str!("../example.txt");
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

        assert_eq!(res, 3749);
    }

    #[test]
    fn test_input() {
        // 1463644435865 too low
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 0);
    }
}
