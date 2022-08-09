pub mod problem_1;
pub mod problem_2;
pub mod problem_3;
pub mod problem_4;
pub mod problem_5;
pub mod problem_50;
pub mod problem_51;
pub mod problem_52;
pub mod problem_55;
pub mod problem_56;
pub mod problem_6;
#[allow(dead_code)]
pub mod problem_60;
#[allow(dead_code)]
#[allow(clippy::needless_range_loop)]
pub mod problem_61;
pub mod problem_7;
pub mod problem_701;
pub mod problem_8;

fn is_prime(n: u64, i: u64) -> bool {
    if n <= 2 {
        return n == 2;
    }
    if n % i == 0 {
        return false;
    }
    if i * i > n {
        return true;
    }

    is_prime(n, i + 1)
}

fn is_prime_v2(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    if n < 9 {
        return true;
    }
    if n % 3 == 0 {
        return false;
    }

    let mut counter = 5;
    while (counter * counter) <= n {
        if n % counter == 0 {
            return false;
        }
        if n % (counter + 2) == 0 {
            return false;
        }
        counter += 6;
    }

    true
}

fn concat(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a * 10;
    }
    let mut a = a;
    let mut c = b;
    while c > 0 {
        a *= 10;
        c /= 10;
    }

    a + b
}

fn build_primes(max: u64) -> Vec<u64> {
    let mut primes = vec![];

    (2..)
        .filter(|&candidate| {
            if !primes.iter().any(|p| candidate % p == 0) {
                primes.push(candidate);
                true
            } else {
                false
            }
        })
        .take_while(|&p| p < max)
        .last();

    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert!(is_prime(41, 2));
    }
}
