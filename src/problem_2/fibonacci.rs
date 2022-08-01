pub struct Fibonacci {
    cur: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.cur + self.next;

        self.cur = self.next;
        self.next = new_next;

        Some(self.cur)
    }
}

pub fn fibonacci() -> Fibonacci {
    Fibonacci { cur: 0, next: 1 }
}
