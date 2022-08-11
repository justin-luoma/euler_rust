use std::collections::VecDeque;

pub fn solve_v2(max: u64) -> u64 {
    let primes = get_primes(max);

    let mut working = primes;
    let mut starting_primes: VecDeque<u64> = VecDeque::new();

    let mut max_prime = 0;
    let mut length = 0;
    loop {
        let sum = working.iter().sum::<u64>();
        if sum > max as u64 {
            working.pop_back();
            continue;
        } else if starting_primes.is_empty() {
            starting_primes = working.clone();
        }

        if crate::is_prime(sum, 2) && sum > max_prime && working.len() > length {
            length = working.len();
            max_prime = sum;
        } else {
            if sum < max_prime {
                break;
            }
            working.pop_back();
            if working.is_empty() {
                break;
            }
        }
    }
    let mut working = starting_primes;
    while !working.is_empty() {
        working.pop_front();
        let sum: u64 = working.iter().sum();
        if crate::is_prime(sum, 2) && sum > max_prime && working.len() > length {
            length = working.len();
            max_prime = sum;
        } else if sum < max_prime {
            break;
        }
    }

    max_prime
}

fn get_primes(max: u64) -> VecDeque<u64> {
    let mut primes = VecDeque::from([2, 3, 5, 7, 11, 13, 17]);

    (*primes.back().unwrap()..)
        .filter(|&candidate| {
            if !primes.iter().any(|p| candidate % p == 0) {
                primes.push_back(candidate);
                true
            } else {
                false
            }
        })
        .take_while(|&prime| prime < max / 2)
        .last();

    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_v2_for_100() {
        assert_eq!(41, solve_v2(100));
    }

    #[test]
    fn test_solve_v2_for_1000() {
        assert_eq!(953, solve_v2(1000));
    }

    #[test]
    #[ignore]
    fn test_solve_v2_for_1_000_000() {
        assert_eq!(997651, solve_v2(1_000_000));
    }
}
