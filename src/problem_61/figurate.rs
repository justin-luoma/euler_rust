pub fn figurate_sieve(sides: u8, max: u64) -> Vec<u64> {
    let mut results: Vec<u64> = vec![1];
    results.push(sides as u64);

    for i in 3..=max {
        results.push(figurate(sides as u64, i));
    }

    results
}

fn figurate(s: u64, n: u64) -> u64 {
    assert!(s >= 3, "must have at least 3 sides");
    let s = s;
    let n = n;

    ((s - 2) * n * (n - 1) / 2) + n
}

#[cfg(test)]
mod tests {
    use crate::problem_61::figurate::figurate;

    #[test]
    fn test_figurate_3() {
        assert_eq!(1, figurate(3, 1));
        assert_eq!(3, figurate(3, 2));
        assert_eq!(6, figurate(3, 3));
        assert_eq!(10, figurate(3, 4));
        assert_eq!(15, figurate(3, 5));
        assert_eq!(8128, figurate(3, 127));
    }

    #[test]
    fn test_figurate_4() {
        assert_eq!(1, figurate(4, 1));
        assert_eq!(4, figurate(4, 2));
        assert_eq!(9, figurate(4, 3));
        assert_eq!(16, figurate(4, 4));
        assert_eq!(25, figurate(4, 5));
        assert_eq!(8281, figurate(4, 91));
    }

    #[test]
    fn test_figurate_5() {
        assert_eq!(1, figurate(5, 1));
        assert_eq!(5, figurate(5, 2));
        assert_eq!(12, figurate(5, 3));
        assert_eq!(22, figurate(5, 4));
        assert_eq!(35, figurate(5, 5));
        assert_eq!(2882, figurate(5, 44));
    }
}
