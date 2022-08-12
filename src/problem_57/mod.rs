use num::{BigUint, ToPrimitive, Zero};
use std::collections::VecDeque;
use std::ops::Rem;

pub fn solve(e: u32) -> u32 {
    let mut count = 0;

    let mut numerator = BigUint::from(3_u8);
    let mut denominator = BigUint::from(2_u8);
    for _ in 1..e {
        numerator += 2_u8 * denominator.clone();
        denominator = numerator.clone() - denominator.clone();
        // println!("{}/{}", numerator, denominator);
        if digits(numerator.clone()).len() > digits(denominator.clone()).len() {
            count += 1;
        }
    }

    count
}

fn digits(n: BigUint) -> Vec<u8> {
    let mut d = VecDeque::new();
    let mut i = n;
    while i > BigUint::zero() {
        let v: BigUint = i.clone().rem(10_u8);
        d.push_front(v.to_u8().unwrap());
        i /= 10_u8;
    }

    d.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_for_1() {
        assert_eq!(0, solve(1));
    }

    #[test]
    fn test_solve_for_8() {
        assert_eq!(1, solve(8));
    }

    #[test]
    fn test_solve_for_1000() {
        assert_eq!(153, solve(1000));
    }
}
