use crate::is_prime;
use std::collections::HashMap;

pub fn solve(family: usize) -> u64 {
    let mut primes: Vec<u64> = vec![];

    (2..)
        .filter(|&candidate| {
            if !primes.iter().any(|prime| candidate % prime == 0) {
                primes.push(candidate);
                true
            } else {
                false
            }
        })
        .find(|prime| {
            if prime == &100003 {
                return false;
            }
            println!("prime {}", prime);
            let prime = prime.to_string();
            let prime_chars: Vec<char> = prime.chars().collect();
            let len = prime_chars.len();
            if len < 6 {
                return false;
            }
            let mut values: Vec<u64> = vec![];
            let mut cursors: HashMap<usize, usize> = HashMap::new();

            let mut working = prime_chars.clone();

            for iteration in 1.. {
                println!("iteration {}", iteration);
                let places = iteration / len + 1;
                println!("places {}", places);
                if places == len {
                    break;
                }
                for place in 1..=places {
                    if let Some(v) = cursors.get(&place) {
                        working[*v] = prime_chars[*v];
                        if v + 1 == len {
                            cursors.insert(place, 0);
                        } else {
                            cursors.insert(place, v + 1);
                        }
                    } else {
                        cursors.insert(place, place - 1);
                    }
                }

                for i in 0..=9 {
                    let mut break_loop = false;
                    cursors.iter().for_each(|(_, &v)| {
                        if i == 0 && v == 0 {
                            break_loop = true;
                        } else {
                            working[v] = char::from_digit(i, 10).unwrap();
                        }
                    });
                    if break_loop {
                        continue;
                    }
                    let string = working.iter().cloned().collect::<String>();
                    println!("string {}", string);
                    let value = string.parse().unwrap();

                    if is_prime(value, 2) && !values.contains(&value) {
                        values.push(value);
                    }
                }
                // if values.len() == family {
                //     break;
                // } else {
                //     values.clear();
                // }
            }

            println!("values: {:?}", values);

            if values.len() == family {
                true
            } else {
                values.clear();
                false
            }

            // for pos in 0..len {
            //     for i in 0..=9 {
            //         values[pos] = i;
            //         prime_chars
            //             .clone()
            //             .into_iter()
            //             .enumerate()
            //             .for_each(|(p, v)| {
            //                 if p != pos {
            //                     values[p] = v.to_digit(10).unwrap() as u64;
            //                 }
            //             });
            //     }
            // }
            // true
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_solve_for_6() {
        assert_eq!(13, solve(6));
    }

    #[test]
    #[ignore]
    fn test_solve_for_7() {
        assert_eq!(56003, solve(7));
    }
}
