pub fn is_palindrome(n: u128) -> bool {
    n == reverse(n)
}

pub fn reverse(n: u128) -> u128 {
    let radix = 10;
    let mut n = n;
    let mut reversed = 0;

    while n != 0 {
        reversed = reversed * radix + n % radix;
        n /= radix;
    }

    reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(321, reverse(123));
        assert_eq!(9009, reverse(9009));
        assert_eq!(808, reverse(8080));
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(9009));
        assert!(is_palindrome(808));
        assert!(is_palindrome(4668731596684224866951378664));
        assert!(!is_palindrome(8080));
    }
}
