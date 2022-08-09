use std::collections::HashMap;

use crate::digits;

pub fn solve(perms: usize) -> u64 {
    let cubes: Vec<u64> = (345..10000u64).map(|i| i.pow(3)).collect();

    let mut cube_perms: HashMap<u64, (u64, u8)> = HashMap::new();

    cubes
        .iter()
        .find_map(|&i| {
            let largest_perm = make_largest_perm(i);
            if let Some(v) = cube_perms.get_mut(&largest_perm) {
                v.1 += 1;
                if v.1 == perms as u8 {
                    Some(cube_perms.get(&largest_perm).unwrap().0)
                } else {
                    None
                }
            } else {
                cube_perms.insert(largest_perm, (i, 1));
                None
            }
        })
        .unwrap()
}

fn make_largest_perm(n: u64) -> u64 {
    let mut k = n;
    let mut digits: Vec<u64> = vec![];
    digits.resize(10, 0);

    let mut result = 0;

    while k > 0 {
        digits[k as usize % 10] += 1;
        k /= 10;
    }

    for i in (0..=9).rev() {
        for _ in 0..digits[i] {
            result = result * 10 + i;
        }
    }

    result as u64
}

fn solve_v1(perms: usize) -> u64 {
    let cubes: Vec<u64> = (345..10000u64).map(|i| i.pow(3)).collect();

    let mut set: Vec<u64> = vec![];
    set.resize(perms, 0);

    for i in 0..cubes.len() {
        set.fill(0);
        set[0] = cubes[i];
        if find_perms(0, perms, &mut set, &cubes) {
            break;
        }
    }

    // println!("{:?}", set);

    set[0]
}

fn find_perms(cur: usize, length: usize, set: &mut Vec<u64>, cubes: &Vec<u64>) -> bool {
    let mut set_digits = digits(set[0]);
    for i in 0..set.len() {
        if set[i] != 0 {
            continue;
        }
        for j in 0..cubes.len() {
            let candidate = cubes[j];
            let mut unique = true;
            for k in 0..set.len() {
                if set[k] == candidate {
                    unique = false;
                    break;
                }
            }

            if !unique {
                continue;
            }

            let mut can_digits = digits(candidate);
            if set_digits.len() != can_digits.len() {
                continue;
            }

            set_digits.sort();
            can_digits.sort();
            if set_digits == can_digits {
                set[i] = candidate;

                if i == set.len() - 1 && find_perms(cur + 1, length, set, cubes) {
                    return false;
                } else if i != set.len() - 1 {
                    return find_perms(cur + 1, length, set, cubes);
                } else if i == set.len() - 1 {
                    return true;
                }
            }
        }
        set[i] = 0;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digits_compare() {
        let mut a = digits(41063625);
        let mut b = digits(56623104);
        a.sort();
        b.sort();

        assert_eq!(a, b);
    }

    #[test]
    fn test_make_longest_perm() {
        println!("{}", 345u64.pow(3));
        println!("{}", make_largest_perm(345u64.pow(3)));
    }

    #[test]
    fn test_solve_for_3() {
        assert_eq!(41063625, solve(3));
    }

    #[test]
    fn test_solve_for_5() {
        assert_eq!(127035954683, solve(5));
    }
}
