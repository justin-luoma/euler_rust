use crate::problem_63::digits;
use ibig::UBig;
use num::{BigUint, One};
use std::iter;

pub fn solve(convergent: u32) -> u64 {
    let mut d = UBig::one();
    let mut n = UBig::from(2_u32);

    for i in 2..=convergent + 1 {
        let d_m_1 = d.clone();
        let c = if i % 3 == 0 { 2 * (i / 3) } else { 1 };
        d = n;
        n = c * d.clone() + d_m_1;
    }

    println!("{}", d);

    digits(d).iter().map(|&i| i as u64).sum()
}

#[allow(dead_code)]
fn learn() {
    let (mut numer, mut denom) = (vec![0], vec![1]);
    let mut tmp = Vec::new();

    for number in iter::once(2)
        .chain((1..).flat_map(|k| [1, 2 * k, 1].into_iter()))
        .take(100)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
    {
        tmp.clone_from(&denom);

        let mut remainder = 0;

        for (&digit_numer, digit_denom) in numer.iter().chain(iter::repeat(&0)).zip(&mut denom) {
            *digit_denom *= number;
            *digit_denom += digit_numer;
            *digit_denom += remainder;
            remainder = *digit_denom / 10;
            *digit_denom %= 10;
        }

        while remainder > 0 {
            denom.push(remainder % 10);
            remainder /= 10;
        }

        numer.clone_from(&tmp);
    }

    let result: u32 = denom.into_iter().sum();
}

#[allow(dead_code)]
fn learn_2() {
    let mut h_n: Vec<BigUint> = vec![BigUint::from(0u32), BigUint::from(1u32)];
    let a0: u32 = 2;
    let mut a_n: Vec<BigUint> = vec![
        BigUint::from(0u32),
        BigUint::from(0u32),
        BigUint::from(2u32),
        BigUint::from(1u32),
        BigUint::from(2u32),
    ];

    while a_n.len() < 102 {
        let last_a = a_n.last().unwrap().clone();
        a_n.push(BigUint::from(1u32));
        a_n.push(BigUint::from(1u32));
        a_n.push(last_a + (BigUint::from(2u32)));
    }

    for n in 2..102 {
        let next_h_n = a_n[n].clone() * h_n[n - 1].clone() + h_n[n - 2].clone();
        h_n.push(next_h_n.clone());
        println!("{:?} {:?}", next_h_n, n - 1);
    }
    let last_h_n = h_n.last().unwrap();
    let s = format!("{:?}", last_h_n);
    println!(
        "{}",
        s.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_for_10() {
        assert_eq!(17, solve(10));
    }

    #[test]
    fn test_solve_for_100() {
        assert_eq!(272, solve(100));
    }
}
