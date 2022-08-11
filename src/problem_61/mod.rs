use crate::problem_61::figurate::figurate_sieve;

mod figurate;

pub fn solve() -> u64 {
    const MAX: u64 = 150;

    let nums: Vec<Vec<u64>> = (3..=8)
        .into_iter()
        .map(|sides| {
            figurate_sieve(sides, MAX)
                .iter()
                .cloned()
                .filter(|&i| i >= 1000 && i < 10000)
                .collect()
        })
        .collect();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(28684, solve());
    }
}
