use ibig::IBig;
use num::integer::Roots;
use num::{One, Zero};

pub fn solve(max: u32) -> u32 {
    let mut solution = IBig::zero();
    let mut result = 0;

    for D in 2..=max {
        let limit = D.sqrt() as u128;
        if limit * limit == D as u128 {
            continue;
        }

        let mut m: u128 = 0;
        let mut d: u128 = 1;
        let mut a = limit as u128;

        let mut numer_m_1 = IBig::one();
        let mut numer = IBig::from(a);

        let mut denom_m_1 = IBig::zero();
        let mut denom = IBig::one();

        while numer.clone() * numer.clone() - IBig::from(D) * denom.clone() * denom.clone()
            != IBig::one()
        {
            m = d * a - m;
            d = (D as u128 - m * m) / d;
            a = (limit + m) / d;

            let numer_m_2 = numer_m_1.clone();
            numer_m_1 = numer.clone();

            let denom_m_2 = denom_m_1.clone();
            denom_m_1 = denom.clone();

            numer = a * numer_m_1.clone() + numer_m_2;
            denom = a * denom_m_1.clone() + denom_m_2;
        }

        println!("{}^2 - {} x {}^2 = 1", numer, D, denom);

        if numer > solution {
            solution = numer.clone();
            result = D;
        }
    }

    println!("D: {}, largest x: {}", result, solution);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_for_7() {
        assert_eq!(5, solve(7));
    }

    #[test]
    fn test_solve_for_1_000() {
        assert_eq!(661, solve(1_000));
    }
}
