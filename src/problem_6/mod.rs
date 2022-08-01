pub fn solve(max: u64) -> u64 {
    let sum: u64 = (1u64..=max).sum();
    let squared_sum: u64 = (1..=max).map(|x| x * x).sum();

    sum.pow(2) - squared_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_for_10() {
        assert_eq!(2640, solve(10));
    }

    #[test]
    fn test_solve() {
        assert_eq!(25164150, solve(100));
    }
}
