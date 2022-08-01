pub fn solve() {}
//
// fn calculate_area(w: u32, h: u32) -> u64 {
//     let max = w.pow(h) * 2u32.pow(2);
//
//     // (1..=max).map(|i| format!("{:b}", i)).map(|b| )
//
//     0
// }
//
// fn count_edges(binary: &String, width: u32) -> u32 {
//     let chars: Vec<char> = binary.chars().collect();
//     let rows: Vec<Vec<bool>> = chars
//         .chunks(width as usize)
//         .map(|ch| {
//             ch.into_iter()
//                 .map(|c| if c == &'1' { true } else { false })
//                 .collect()
//         })
//         .collect();
//
//     let mut coords: HashMap<usize, usize> = HashMap::new();
//
//     for r in 0..rows.len() {
//         for c in 0..width as usize {
//             if rows[r][c] {
//                 coords.insert(r, c);
//             }
//         }
//     }
//
//     coords.len() as u32
// }

#[cfg(test)]
mod tests {
    #[test]
    fn testing() {}
}
