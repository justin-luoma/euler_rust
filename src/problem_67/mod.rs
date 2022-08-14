use crate::problem_18::solve;

pub fn solve_67() -> u64 {
    solve("./src/problem_67/input.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(7273, solve_67());
    }
}
