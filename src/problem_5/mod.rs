use std::collections::HashMap;

pub fn solve(max: u64) -> u64 {
    // (1..=max)
    //     .map(|i| prime_factors(i).iter().max().unwrap())
    //     .product()
    (2520..)
        .step_by(2)
        .find(|i| (1..=max).all(|n| i % n == 0))
        .unwrap()
}

pub fn solve_v2(max: u64) -> u64 {
    (1..=max)
        .map(prime_factors)
        .map(greatest_power)
        .fold(HashMap::new(), |mut acc, gp| {
            for (k, cur) in gp {
                if let Some(v) = acc.get_mut(&k) {
                    if cur > *v {
                        *v = cur;
                    }
                } else {
                    acc.insert(k, cur);
                }
            }
            // gp.iter().for_each(|(k, &cur)| {
            //
            // });
            acc
        })
        .values()
        .product()

    // 0
}

fn prime_factors(n: u64) -> Vec<u64> {
    if n == 1 {
        return vec![1];
    }

    let mut factors: Vec<u64> = vec![];

    let mut primes = vec![2, 3, 5, 7, 11, 13];

    let mut remainder = n;

    while remainder != 1 {
        if let Some(factor) = primes.iter().find(|&prime| remainder % prime == 0) {
            factors.push(*factor);
            remainder /= factor;
        } else {
            let factor = (*primes.last().unwrap()..=n)
                .filter(|i| {
                    if !primes.iter().any(|prime| i % prime == 0) {
                        primes.push(*i);
                        true
                    } else {
                        false
                    }
                })
                .find(|prime| remainder % prime == 0)
                .unwrap();
            factors.push(factor);
            remainder /= factor;
        }
    }

    factors
}

fn greatest_power(nums: Vec<u64>) -> HashMap<u64, u64> {
    let powers = nums.iter().fold(HashMap::new(), |mut acc, num| {
        if let Some(n) = acc.get_mut(num) {
            *n *= num;
        } else {
            acc.insert(*num, *num);
        }
        acc
    });

    powers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_factors() {
        assert_eq!(vec![1], prime_factors(1));
        assert_eq!(vec![2], prime_factors(2));
        assert_eq!(vec![3], prime_factors(3));
        assert_eq!(vec![2, 7], prime_factors(14));
        assert_eq!(vec![2, 2, 2], prime_factors(8));
        assert_eq!(vec![2, 2, 5], prime_factors(20));
        assert_eq!(vec![83], prime_factors(83));

        println!("{:?}", prime_factors(11));
    }

    #[test]
    fn test_greatest_power() {
        let expected_20 = HashMap::from([(2, 4), (5, 5)]);
        assert_eq!(expected_20, greatest_power(prime_factors(20)));

        let expected_19 = HashMap::from([(19, 19)]);
        assert_eq!(expected_19, greatest_power(prime_factors(19)));
    }

    #[test]
    fn test_solve_max_10() {
        assert_eq!(2520, solve(10));
        assert_eq!(2520, solve_v2(10));
    }

    #[test]
    fn test_solve_max_20() {
        assert_eq!(232792560, solve(20));
        assert_eq!(232792560, solve_v2(20));
    }

    #[test]
    fn test_solve_max_30() {
        // assert_eq!(232792560, solve(20));
        assert_eq!(2329089562800, solve_v2(30));
    }

    #[test]
    #[ignore]
    fn test_large() {
        println!("{}", solve_v2(30));
        println!("{}", solve(30));
    }
}
