pub fn solve(n: u32) -> u64 {
    // nth_prime(n as usize)
    nth(n)
}

// fn nth_prime(n: usize) -> u64 {
//     let mut primes = vec![];
//
//     (2..)
//         .filter(|i| {
//             if !primes.iter().any(|p| i % p == 0) {
//                 primes.push(*i);
//                 true
//             } else {
//                 false
//             }
//         })
//         .nth(n - 1)
//         .unwrap()
// }

fn nth(n: u32) -> u64 {
    if n == 0 {
        return 2;
    }
    let mut prime = 3;
    let mut i = 1;
    loop {
        if crate::is_prime(prime, 2) {
            i += 1;
        }
        if i == n {
            return prime;
        }
        prime += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_6() {
        assert_eq!(13, solve(6));
    }

    #[test]
    fn test_solve() {
        assert_eq!(104743, solve(10001));
    }
}
