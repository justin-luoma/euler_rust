use ibig::UBig;
use num::Zero;
use std::collections::VecDeque;
use std::ops::Rem;

pub fn solve() -> u64 {
    let mut count = 0;

    for i in 1..=9u32 {
        for j in 1..30 {
            let power = UBig::from(i).pow(j);
            let length = digits(power.clone()).len();
            if j == length {
                count += 1;
                // println!("{}^{}: {}", i, j, power);
            }
            // println!("{} {} {}", j, length, j < length as u32);
        }
    }

    count
}

fn digits(n: UBig) -> Vec<u8> {
    let mut d = VecDeque::new();
    let mut i = n;
    while i > UBig::zero() {
        let v: i32 = i.clone().rem(10);
        d.push_front(v as u8);
        i /= 10;
    }

    d.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ibig::ubig;

    #[test]
    fn test_solve() {
        assert_eq!(49, solve());
    }

    #[test]
    fn test_digits() {
        assert_eq!(vec![8, 2, 6, 4], digits(ubig!(8264)));
    }
}
