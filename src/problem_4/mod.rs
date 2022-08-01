pub fn solve() -> u64 {
    let mut solutions: Vec<u64> = vec![];

    for i in (1..=999).rev() {
        for j in (1..=999).rev() {
            let product = i * j;
            if is_palindrome(product) {
                solutions.push(product);
            }
        }
    }

    *solutions.iter().max().unwrap()
}

fn is_palindrome(n: u64) -> bool {
    n == reverse(n)
}

fn reverse(n: u64) -> u64 {
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
    fn test_solve() {
        println!("{}", solve());
        assert_eq!(906609, solve());
    }

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
        assert!(!is_palindrome(8080));
    }
}
