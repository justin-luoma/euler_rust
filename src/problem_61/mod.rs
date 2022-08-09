mod figurate;

use crate::problem_61::figurate::figurate_sieve;
use std::collections::VecDeque;

pub fn solve() -> u64 {
    let mut triangle: Vec<u64> = vec![];
    let mut square: Vec<u64> = vec![];
    let mut pentagon: Vec<u64> = vec![];
    let mut hexagon: Vec<u64> = vec![];
    let mut heptagon: Vec<u64> = vec![];
    let mut octagon: Vec<u64> = vec![];

    const MAX: u64 = 100;
    rayon::scope(|s| {
        s.spawn(|_| triangle = figurate_sieve(3, 175));
        s.spawn(|_| square = figurate_sieve(4, MAX));
        s.spawn(|_| pentagon = figurate_sieve(5, MAX));
        s.spawn(|_| hexagon = figurate_sieve(6, MAX));
        s.spawn(|_| heptagon = figurate_sieve(7, MAX));
        s.spawn(|_| octagon = figurate_sieve(8, MAX));
    });

    let nums: Vec<Vec<u64>> = vec![
        triangle
            .iter()
            .cloned()
            .filter(|&i| i >= 1000 && i < 10000)
            .collect(),
        square
            .iter()
            .cloned()
            .filter(|&i| i >= 1000 && i < 10000)
            .collect(),
        pentagon
            .iter()
            .cloned()
            .filter(|&i| i >= 1000 && i < 10000)
            .collect(),
        hexagon
            .iter()
            .cloned()
            .filter(|&i| i >= 1000 && i < 10000)
            .collect(),
        heptagon
            .iter()
            .cloned()
            .filter(|&i| i >= 1000 && i < 10000)
            .collect(),
        octagon
            .iter()
            .cloned()
            .filter(|&i| i >= 1000 && i < 10000)
            .collect(),
    ];

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
