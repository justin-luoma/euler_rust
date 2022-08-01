use std::borrow::Borrow;

pub fn solve(n: u64) -> u64 {
    let mut primes: Vec<u64> = vec![2];

    let mut remainder = n;

    while remainder != 1 {
        let factor = (2..)
            .filter(|i| {
                if !primes.iter().any(|p| i % p == 0) {
                    primes.push(*i);
                    true
                } else {
                    false
                }
            })
            .take_while(|i| i < n.borrow())
            .find(|prime| remainder % prime == 0)
            .unwrap();
        remainder /= factor;
    }
    *primes.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::problem_3::solve;

    #[test]
    fn test() {
        assert_eq!(29, solve(13_195));
    }

    #[test]
    fn test_solve() {
        println!("{}", solve(600851475143))
    }
}
