pub fn solve() -> u64 {
    (1000..)
        .find(|&i| {
            if contain_same_digits(i, i * 2)
                && contain_same_digits(i, i * 3)
                && contain_same_digits(i, i * 4)
                && contain_same_digits(i, i * 5)
                && contain_same_digits(i, i * 6)
            {
                return true;
            }
            false
        })
        .unwrap()
}

fn contain_same_digits(n1: u64, n2: u64) -> bool {
    let mut chars1: Vec<char> = n1.to_string().chars().collect();
    chars1.sort();
    let mut chars2: Vec<char> = n2.to_string().chars().collect();
    chars2.sort();

    chars1 == chars2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contain_same_digits() {
        assert!(contain_same_digits(125874, 251748))
    }
}
