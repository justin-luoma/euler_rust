use num::integer::Roots;

pub fn solve(bound: u32) -> u32 {
    let mut result = 0;

    for n in 2..=bound {
        let limit = (n.sqrt()) as u32;
        if limit * limit == n {
            continue;
        }

        let mut period = 0;
        let mut d = 1;
        let mut m = 0;
        let mut a = limit;

        print!("{}: {}; ", n, a);

        loop {
            m = d * a - m;
            d = (n - m * m) / d;
            a = (limit + m) / d;
            print!("{} ", a);
            period += 1;
            if a == 2 * limit {
                break;
            }
        }

        println!("p={}", period);

        if period % 2 == 1 {
            result += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_for_13() {
        assert_eq!(4, solve(13));
    }

    #[test]
    fn test_solve_for_1000() {
        assert_eq!(4, solve(10_000));
    }
}
