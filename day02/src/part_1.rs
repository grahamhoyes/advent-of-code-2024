/// A report is safe if:
/// - All values are either all increasing or all decreasing
/// - Each subsequent value differs by at least one and at most three
fn is_report_safe(report: &[i32]) -> bool {
    assert!(report.len() >= 2);

    let sign = (report[0] - report[1]).signum();

    for (a, b) in report.iter().zip(report.iter().skip(1)) {
        let diff = a - b;

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        if diff.signum() != sign {
            return false;
        }
    }

    true
}

pub fn solution(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|t| t.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|report| if is_report_safe(&report) { 1 } else { 0 })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 2);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 299);
    }
}
