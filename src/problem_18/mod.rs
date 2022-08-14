use io::BufReader;
use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn solve(path: &str) -> u64 {
    let file = File::open(path).expect("Couldn't open file");

    let mut values: Vec<Vec<u64>> = BufReader::new(file)
        .lines()
        .flatten()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.to_string().parse().unwrap())
                .collect()
        })
        .collect();

    for row in (0..values.len() - 1).rev() {
        for i in 0..values[row].len() {
            let b = values[row + 1][i];
            let c = values[row + 1][i + 1];
            let a = values[row].get_mut(i).unwrap();

            *a += b.max(c);
        }
    }

    values[0][0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_for_test() {
        assert_eq!(23, solve("./src/problem_18/test.txt"));
    }

    #[test]
    fn test_solve_for_input() {
        assert_eq!(1074, solve("./src/problem_18/input.txt"));
    }
}
