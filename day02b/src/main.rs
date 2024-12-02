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

fn solution(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|t| t.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|report| {
            // Convert each report into a set of copies where each element is missing once.
            // This is not an efficient way to do this, but the reports are always quite
            // small and there's only 1000 of them so this is fine.

            // Keep the unmodified report too
            let mut reports: Vec<Vec<i32>> = vec![report.clone()];

            for n in 0..report.len() {
                // Push a copy of the report with just element n removed
                reports.push(
                    report
                        .iter()
                        .enumerate()
                        .filter(|&(i, _)| i != n)
                        .map(|(_, v)| *v)
                        .collect(),
                );
            }

            reports
        })
        .filter(|reports| {
            for report in reports.iter() {
                if is_report_safe(report) {
                    return true;
                }
            }
            false
        })
        .count()
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

        assert_eq!(res, 4);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 364);
    }
}
