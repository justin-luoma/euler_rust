use crate::problem_4::palindrome::{is_palindrome, reverse};

pub fn solve() -> usize {
    let mut count = 0;
    for i in 1..10_000 {
        if is_lychrel_number(i) {
            count += 1;
        }
    }

    count
}

fn is_lychrel_number(n: u128) -> bool {
    let reversed = reverse(n);
    let mut sum = n + reversed;

    for _ in 0..=50 {
        if is_palindrome(sum) {
            return false;
        }
        sum = sum + reverse(sum);
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(249, solve());
    }

    #[test]
    fn test_is_lychrel_number() {
        assert!(!is_lychrel_number(47));
        assert!(is_lychrel_number(196));
    }
}
