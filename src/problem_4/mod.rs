pub mod palindrome;

pub fn solve() -> u64 {
    let mut solutions: Vec<u64> = vec![];

    for i in (1..=999).rev() {
        for j in (1..=999).rev() {
            let product = i * j;
            if palindrome::is_palindrome(product) {
                solutions.push(product as u64);
            }
        }
    }

    *solutions.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        println!("{}", solve());
        assert_eq!(906609, solve());
    }
}
