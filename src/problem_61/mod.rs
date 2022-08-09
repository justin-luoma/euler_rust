use std::collections::VecDeque;

use crate::problem_61::figurate::figurate_sieve;

mod figurate;

pub fn solve() -> u64 {
    const MAX: u64 = 100;
    let triangle: Vec<u64> = figurate_sieve(3, 175)
        .iter()
        .cloned()
        .filter(|&i| i >= 1000 && i < 10000)
        .collect();
    let square: Vec<u64> = figurate_sieve(4, MAX)
        .iter()
        .cloned()
        .filter(|&i| i >= 1000 && i < 10000)
        .collect();
    let pentagon: Vec<u64> = figurate_sieve(5, MAX)
        .iter()
        .cloned()
        .filter(|&i| i >= 1000 && i < 10000)
        .collect();
    let hexagon: Vec<u64> = figurate_sieve(6, MAX)
        .iter()
        .cloned()
        .filter(|&i| i >= 1000 && i < 10000)
        .collect();
    let heptagon: Vec<u64> = figurate_sieve(7, MAX)
        .iter()
        .cloned()
        .filter(|&i| i >= 1000 && i < 10000)
        .collect();
    let octagon: Vec<u64> = figurate_sieve(8, MAX)
        .iter()
        .cloned()
        .filter(|&i| i >= 1000 && i < 10000)
        .collect();

    let nums: Vec<Vec<u64>> = vec![triangle, square, pentagon, hexagon, heptagon, octagon];

    let mut set: Vec<u64> = vec![];
    set.resize(6, 0);

    for i in 0..nums[5].len() {
        set[5] = nums[5][i];
        if find_next(5, 1, &mut set, &nums) {
            break;
        }
    }

    let sum = set.iter().sum();

    // println!("{:?}", set);
    //
    // set.iter().for_each(|v| {
    //     print!("{} ", v);
    //
    //     if let Ok(i) = triangle.binary_search(v) {
    //         print!("triangle: {} ", i);
    //     }
    //     if let Ok(i) = square.binary_search(v) {
    //         print!("square: {} ", i);
    //     }
    //     if let Ok(i) = pentagon.binary_search(v) {
    //         print!("pentagon: {} ", i);
    //     }
    //     if let Ok(i) = hexagon.binary_search(v) {
    //         print!("hexagon: {} ", i);
    //     }
    //     if let Ok(i) = heptagon.binary_search(v) {
    //         print!("heptagon: {} ", i);
    //     }
    //     if let Ok(i) = octagon.binary_search(v) {
    //         print!("octagon: {} ", i);
    //     }
    //     println!();
    // });

    sum
}

fn find_next(last: usize, length: usize, set: &mut Vec<u64>, nums: &Vec<Vec<u64>>) -> bool {
    for i in 0..set.len() {
        if set[i] != 0 {
            continue;
        }
        for j in 0..nums[i].len() {
            let mut unique = true;
            for k in 0..set.len() {
                if nums[i][j] == set[k] {
                    unique = false;
                    break;
                }
            }

            if unique && (nums[i][j] / 100) as u32 == (set[last] % 100) as u32 {
                set[i] = nums[i][j];
                if length == 5 && (set[5] / 100) as u32 == (set[i] % 100) as u32 {
                    return true;
                }
                if find_next(i, length + 1, set, nums) {
                    return true;
                }
            }
        }
        set[i] = 0;
    }
    false
}

fn digits(n: u64) -> Vec<u8> {
    let mut d = VecDeque::new();
    let mut i = n;
    while i > 0 {
        let v = i % 10;
        d.push_front(v as u8);
        i /= 10;
    }

    d.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(28684, solve());
    }

    #[test]
    fn test_digits() {
        assert_eq!(vec![8, 2, 6, 4], digits(8264));
    }
}
