mod fibonacci;

use fibonacci::fibonacci;

pub fn solve() -> u32 {
    fibonacci()
        .take_while(|i| i < &4_000_000u32)
        .filter(|i| i % 2 == 0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(4613732, solve());
    }
}
