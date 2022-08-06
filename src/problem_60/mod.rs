use crate::{build_primes, is_prime, is_prime_v2};
use std::collections::HashSet;

use itertools::Itertools;

pub fn solve(set: u8) -> u64 {
    const MAX: u64 = 10_000;
    let primes: Vec<u64> = prime_sieve::primes(MAX);

    let mut min_prime_set = u64::MAX;

    let max_len = primes.len();
    let max_loops = set as usize;

    let mut pairs: Vec<HashSet<u64>> = Vec::with_capacity(max_len);
    for _ in 0..max_len {
        pairs.push(HashSet::new())
    }

    let mut loops: Vec<usize> = vec![];
    loops.resize(max_loops, 1);

    let mut active_loop = 0;

    loop {
        let active_i = loops[active_loop];
        if active_i >= max_len - 1 && active_loop != 0 {
            loops[active_loop - 1] += 1;
            active_loop -= 1;
            continue;
        }

        let mut sum = 0;
        for i in 0..=active_loop {
            sum += primes[loops[i]];
        }
        sum *= max_loops as u64 - active_loop as u64;
        if sum >= min_prime_set {
            if active_loop == 0 {
                break;
            } else {
                loops[active_loop - 1] += 1;
                if loops[active_loop - 1] == max_len {
                    if active_loop - 1 == 0 {
                        break;
                    }
                    active_loop -= 1;
                }
                active_loop -= 1;
                continue;
            }
        }

        if active_loop == 0 {
            if pairs[active_i].is_empty() {
                pairs[active_i] = make_pairs(active_i as u64, &primes);
            }
        } else {
            let mut should_continue = false;
            for i in 0..active_loop {
                if !pairs[loops[i]].contains(&primes[active_i]) {
                    should_continue = true;
                }
                if should_continue {
                    break;
                }
            }
            if should_continue {
                loops[active_loop] += 1;
                continue;
            }
            if pairs[active_i].is_empty() {
                pairs[active_i] = make_pairs(active_i as u64, &primes);
            }

            if active_loop == max_loops - 1 {
                let mut sum = 0;
                for i in 0..max_loops {
                    sum += primes[loops[i]];
                }
                if min_prime_set > sum {
                    for i in 0..max_loops {
                        print!("{} ", primes[loops[i]]);
                    }
                    println!();
                    min_prime_set = sum;
                }
                loops[active_loop - 1] += 1;
                active_loop -= 1;
                continue;
            }
        }

        if active_loop == 0 && active_i == max_len - 1 {
            break;
        }

        if active_loop == max_loops - 1 {
            loops[active_loop] += 1;
            if loops[active_loop] >= max_len - 1 {
                active_loop -= 1;
                loops[active_loop] += 1;
            }
            continue;
        }

        if active_loop + 1 < max_loops {
            loops[active_loop + 1] = active_i + 1;
            active_loop += 1;
        }
    }

    min_prime_set
}

fn make_pairs(a: u64, primes: &Vec<u64>) -> HashSet<u64> {
    let mut pairs: HashSet<u64> = HashSet::new();
    for b in a + 1..primes.len() as u64 {
        if is_prime_v2(concat(primes[a as usize], primes[b as usize]))
            && is_prime_v2(concat(primes[b as usize], primes[a as usize]))
        {
            pairs.insert(primes[b as usize]);
        }
    }

    pairs
}

fn concat(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut c = b;
    while c > 0 {
        a *= 10;
        c /= 10;
    }

    a + b
}

// takes 289621 seconds to calculate for 5
fn solve_slow(count: usize) -> u64 {
    let primes = build_primes(10000);
    primes
        .into_iter()
        .combinations(count)
        .find(|com| {
            com.iter().combinations(2).all(|two| {
                is_prime(concat(*two[0], *two[1]), 2) && is_prime(concat(*two[1], *two[0]), 2)
            })
        })
        .map(|values| values.iter().sum::<u64>())
        .unwrap()
}

fn concat_old(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_for_4() {
        assert_eq!(792, solve(4));
    }

    #[test]
    fn test_solve_for_5() {
        assert_eq!(26033, solve(5));
    }

    #[test]
    fn test_concat() {
        assert_eq!(123456, concat(123, 456));
    }
}
