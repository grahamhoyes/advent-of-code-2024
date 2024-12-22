const PRUNE_MASK: u64 = (1 << 24) - 1; // % 16777216

/// Iterate a secret value
///
/// prng secrets are computed by an LFSR
pub fn iterate_secret(mut secret: u64) -> u64 {
    secret ^= secret << 6; // * 64
    secret &= PRUNE_MASK;

    secret ^= secret >> 5; // / 32
    secret &= PRUNE_MASK;

    secret ^= secret << 11; // * 1024
    secret &= PRUNE_MASK;

    secret
}

pub fn solution(input: &str) -> u64 {
    input
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .map(|mut secret| {
            for _ in 0..2000 {
                secret = iterate_secret(secret)
            }
            secret
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let res = solution(input);

        assert_eq!(res, 37990510);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../input.txt");
        let res = solution(input);

        assert_eq!(res, 16619522798);
    }
}
