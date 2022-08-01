pub fn solve(max: u32) -> u32 {
    (1..max).filter(|n| n % 3 == 0 || n % 5 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_for_10_expect_23() {
        assert_eq!(23, solve(10));
    }

    #[test]
    fn test_solve_for_1000_expect_233168() {
        assert_eq!(233168, solve(1000));
    }
}
