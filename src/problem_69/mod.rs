use num::Integer;
use prime_sieve::primes;

pub fn solve_v2(n: u64) -> u64 {
    let primes = primes(200);

    let mut i = 0;
    let mut result = 1;
    while result * primes[i] < n {
        result *= primes[i];
        i += 1;
    }

    result
}

pub fn solve(n: u64) -> u64 {
    let mut max = 0.0;
    let mut result = 0;

    for i in 2..=n {
        if i.is_odd() {
            continue;
        }
        let mut values: Vec<u64> = vec![];
        for j in (1..=i - 1).rev() {
            if is_relatively_prime(i, j) {
                values.push(j);
            }
        }
        let totient = i as f32 / values.len() as f32;
        if totient > max {
            max = totient;
            result = i;
        }
    }

    result
}

fn learn() -> u32 {
    const MAX: u32 = 1_000_000;

    (1..=MAX)
        .map(|number| {
            (
                number,
                number as f32 / {
                    let mut divisors = Vec::new();
                    let mut number_mut = number;
                    let mut divisor: u32 = 2;

                    while divisor.pow(2) <= number_mut {
                        if number_mut % divisor == 0 {
                            divisors.push((divisor, 0));

                            while number_mut % divisor == 0 {
                                divisors.last_mut().unwrap().1 += 1;
                                number_mut /= divisor;
                            }
                        }

                        divisor += 1;
                    }

                    if number_mut > 1 {
                        divisors.push((number_mut, 1));
                    }

                    divisors
                }
                .into_iter()
                .map(|(prime, count)| prime.pow(count - 1) * (prime - 1))
                .product::<u32>() as f32,
            )
        })
        .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(number, _)| number)
        .unwrap()
}

fn is_relatively_prime(n: u64, i: u64) -> bool {
    n.gcd(&i) == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_relatively_prime() {
        for i in (1..=6).rev().skip(3) {
            assert!(is_relatively_prime(7, i), "{} wasn't true", i);
        }
        assert!(is_relatively_prime(3, 2));
    }

    #[test]
    fn test_solve_for_10() {
        assert_eq!(6, solve(10));
    }

    #[test]
    fn test_solve_for_1_000_000() {
        assert_eq!(510510, solve(1_000_000));
    }

    #[test]
    fn test_solve_v2_for_1_000_000() {
        assert_eq!(510510, solve_v2(1_000_000));
    }
}
