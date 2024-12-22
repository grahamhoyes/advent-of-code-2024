fn valid_calibration(target: u64, measurements: &[u64], accum: u64) -> bool {
    if measurements.is_empty() {
        return accum == target;
    }

    let value = measurements[0];
    let rest = &measurements[1..];

    valid_calibration(target, rest, accum + value)
        || valid_calibration(target, rest, accum * value)
        || valid_calibration(target, rest, format!("{accum}{value}").parse().unwrap())
}

pub fn solution(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let (value, measurements) = l.split_once(':').unwrap();
            let value = value.parse::<u64>().unwrap();
            let measurements: Vec<u64> = measurements
                .split_whitespace()
                .map(|t| t.parse().unwrap())
                .collect();

            (value, measurements)
        })
        .filter(|(value, measurements)| valid_calibration(*value, measurements, 0))
        .map(|(value, _)| value)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 11387);
    }

    #[test]
    fn test_input() {
        // 1463644435865 too low
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 426214131924213);
    }
}
