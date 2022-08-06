use ibig::{ubig, UBig};

pub fn solve() -> UBig {
    let mut max = ubig!(0);

    for a in 0..100u32 {
        for b in 0..100 {
            let value = UBig::from(a).pow(b);
            let sum = sum_digits(value);
            if sum > max {
                max = sum;
            }
        }
    }

    max
}

fn sum_digits(n: UBig) -> UBig {
    let mut sum = ubig!(0);
    let mut i = n;
    while i != ubig!(0) {
        sum += &i % 10;
        i /= 10;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(ubig!(972), solve());
    }

    #[test]
    fn test_sum_digits() {
        assert_eq!(ubig!(6), sum_digits(ubig!(123)));
        assert_eq!(ubig!(1), sum_digits(ubig!(1_000_000_000)));
    }
}
