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

    /*let nums: Vec<HashMap<u64, (u8, u8)>> = vec![
        triangle
            .iter()
            .cloned()
            .filter(|&i| i >= 1000 && i < 10000)
            .fold(HashMap::new(), |mut map, v| {
                let digits = digits(v);
                map.insert(
                    v,
                    (
                        concat(digits[0] as u64, digits[1] as u64) as u8,
                        concat(digits[2] as u64, digits[3] as u64) as u8,
                    ),
                );
                map
            }),
        square
            .iter()
            .cloned()
            .filter(|&i| i >= 1000 && i < 10000)
            .fold(HashMap::new(), |mut map, v| {
                let digits = digits(v);
                map.insert(
                    v,
                    (
                        concat(digits[0] as u64, digits[1] as u64) as u8,
                        concat(digits[2] as u64, digits[3] as u64) as u8,
                    ),
                );
                map
            }),
        pentagon
            .iter()
            .cloned()
            .filter(|&i| i >= 1000 && i < 10000)
            .fold(HashMap::new(), |mut map, v| {
                let digits = digits(v);
                map.insert(
                    v,
                    (
                        concat(digits[0] as u64, digits[1] as u64) as u8,
                        concat(digits[2] as u64, digits[3] as u64) as u8,
                    ),
                );
                map
            }),
        hexagon
            .iter()
            .cloned()
            .filter(|&i| i >= 1000 && i < 10000)
            .fold(HashMap::new(), |mut map, v| {
                let digits = digits(v);
                map.insert(
                    v,
                    (
                        concat(digits[0] as u64, digits[1] as u64) as u8,
                        concat(digits[2] as u64, digits[3] as u64) as u8,
                    ),
                );
                map
            }),
        heptagon
            .iter()
            .cloned()
            .filter(|&i| i >= 1000 && i < 10000)
            .fold(HashMap::new(), |mut map, v| {
                let digits = digits(v);
                map.insert(
                    v,
                    (
                        concat(digits[0] as u64, digits[1] as u64) as u8,
                        concat(digits[2] as u64, digits[3] as u64) as u8,
                    ),
                );
                map
            }),
        octogon
            .iter()
            .cloned()
            .filter(|&i| i >= 1000 && i < 10000)
            .fold(HashMap::new(), |mut map, v| {
                let digits = digits(v);
                let front = concat(digits[0] as u64, digits[1] as u64) as u8;
                let back = concat(digits[2] as u64, digits[3] as u64) as u8;
                map.insert(v, (front, back));
                map
            }),
    ];

    for a in 0..nums[0].len() {
        let a_vals = nums[0].clone();
        let a_val = a_vals.keys().nth(a).unwrap();
        let a_i = triangle.binary_search(a_val).unwrap();
        let (a_front, a_back) = a_vals.values().nth(a).unwrap();

        for b in 0..nums[1].len() {
            let b_vals = nums[1].clone();
            let b_val = b_vals.keys().nth(b).unwrap();
            let b_i = square.binary_search(b_val).unwrap();
            // if a_i == b_i {
            //     continue;
            // }
            let (b_front, b_back) = b_vals.values().nth(b).unwrap();

            for c in 0..nums[2].len() {
                let c_vals = nums[2].clone();
                let c_val = c_vals.keys().nth(c).unwrap();
                let c_i = pentagon.binary_search(c_val).unwrap();
                // if a_i == c_i || b_i == c_i {
                //     continue;
                // }
                let (c_front, c_back) = c_vals.values().nth(c).unwrap();

                for d in 0..nums[3].len() {
                    let d_vals = nums[3].clone();
                    let d_val = d_vals.keys().nth(d).unwrap();
                    let d_i = hexagon.binary_search(d_val).unwrap();
                    // if a_i == d_i || b_i == d_i || c_i == d_i {
                    //     continue;
                    // }
                    let (d_front, d_back) = d_vals.values().nth(d).unwrap();

                    for e in 0..nums[4].len() {
                        let e_vals = nums[4].clone();
                        let e_val = e_vals.keys().nth(e).unwrap();
                        let e_i = heptagon.binary_search(e_val).unwrap();
                        // if a_i == e_i || b_i == e_i || c_i == e_i || d_i == e_i {
                        //     continue;
                        // }
                        let (e_front, e_back) = e_vals.values().nth(e).unwrap();

                        for f in 0..nums[5].len() {
                            let f_vals = nums[5].clone();
                            let f_val = f_vals.keys().nth(f).unwrap();
                            let f_i = octogon.binary_search(f_val).unwrap();
                            // if a_i == f_i || b_i == f_i || c_i == f_i || d_i == f_i || e_i == f_i {
                            //     continue;
                            // }
                            let (f_front, f_back) = f_vals.values().nth(f).unwrap();

                            let mut should_break = false;
                            for first in 0..=5 {
                                if should_break {
                                    break;
                                }

                                for second in 0..=5 {
                                    if should_break {
                                        break;
                                    }
                                    if first == second {
                                        continue;
                                    }

                                    for third in 0..=5 {
                                        if should_break {
                                            break;
                                        }
                                        if first == third || second == third {
                                            continue;
                                        }

                                        for fourth in 0..=5 {
                                            if should_break {
                                                break;
                                            }
                                            if first == fourth
                                                || second == fourth
                                                || third == fourth
                                            {
                                                continue;
                                            }

                                            for fifth in 0..=5 {
                                                if should_break {
                                                    break;
                                                }
                                                if first == fifth
                                                    || second == fifth
                                                    || third == fifth
                                                    || fourth == fifth
                                                {
                                                    continue;
                                                }

                                                for sixth in 0..=5 {
                                                    if should_break {
                                                        break;
                                                    }
                                                    if first == sixth
                                                        || second == sixth
                                                        || third == sixth
                                                        || fourth == sixth
                                                        || fifth == sixth
                                                    {
                                                        continue;
                                                    }
                                                    let order = HashMap::from([
                                                        (first, 'a'),
                                                        (second, 'b'),
                                                        (third, 'c'),
                                                        (fourth, 'd'),
                                                        (fifth, 'e'),
                                                        (sixth, 'f'),
                                                    ]);

                                                    let mut solution: Vec<(&u8, &u8)> = vec![];

                                                    // let first = match order[&0] {
                                                    //     'a' => (a_front, a_back),
                                                    //     'b' => (b_front, b_back),
                                                    //     'c' => (c_front, c_back),
                                                    //     'd' => (d_front, d_back),
                                                    //     'e' => (e_front, e_back),
                                                    //     'f' => (f_front, f_back),
                                                    //     _ => {unreachable!()}
                                                    // };

                                                    order
                                                        .iter()
                                                        .sorted_by(|(a, _), (b, _)| Ord::cmp(a, b))
                                                        .for_each(|(order, letter)| {
                                                            solution.push(match letter {
                                                                'a' => (a_front, a_back),
                                                                'b' => (b_front, b_back),
                                                                'c' => (c_front, c_back),
                                                                'd' => (d_front, d_back),
                                                                'e' => (e_front, e_back),
                                                                'f' => (f_front, f_back),
                                                                _ => {
                                                                    unreachable!()
                                                                }
                                                            });
                                                        });

                                                    if solution[0].0
                                                        != solution[solution.len() - 1].1
                                                        && solution[solution.len() - 1].0
                                                            == solution[solution.len() - 2].1
                                                    {
                                                        should_break = true;
                                                        break;
                                                    }
                                                    if solution[0].1 == solution[1].0
                                                        && solution[1].1 == solution[2].0
                                                        && solution[2].1 == solution[3].0
                                                        && solution[3].1 == solution[4].0
                                                        && solution[4].1 == solution[5].0
                                                    {
                                                        println!("{:?}", solution);
                                                    } else {
                                                        should_break = true;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let mut fronts: Vec<HashMap<u8, u64>> = vec![];
    fronts.resize(6, HashMap::new());
    let mut backs: Vec<HashMap<u8, u64>> = vec![];
    backs.resize(6, HashMap::new());

    let digits: Vec<HashMap<u64, (u8, u8, u8, u8)>> = nums
        .iter()
        .enumerate()
        .map(|(i, v)| {
            v.iter().fold(HashMap::new(), |mut m, &v| {
                let d = digits(v);
                if d.len() == 4 {
                    m.insert(v, (d[0], d[1], d[2], d[3]));
                    fronts[i].insert(concat(d[0] as u64, d[1] as u64) as u8, v);
                    backs[i].insert(concat(d[2] as u64, d[3] as u64) as u8, v);
                }
                m
            })
        })
        .collect();

    let tri_front = fronts[0].clone();
    let tri_back = backs[0].clone();

    let sq_front = fronts[1].clone();
    let sq_back = backs[1].clone();

    let penta_front = fronts[2].clone();
    let penta_back = backs[2].clone();

    let mut valid_penta: Vec<u64> = vec![];

    let valid_tris_penta: Vec<u64> = tri_back
        .iter()
        .filter(|(k, _)| {
            if let Some(v) = penta_front.get(k) {
                valid_penta.push(*v);
                true
            } else {
                false
            }
        })
        .map(|(_, &v)| v)
        .collect();

    let mut valid_square: Vec<u64> = vec![];

    let valid_tris_square: Vec<u64> = tri_front
        .iter()
        .filter(|(k, _)| {
            if let Some(v) = sq_back.get(k) {
                valid_square.push(*v);
                true
            } else {
                false
            }
        })
        .map(|(_, &v)| v)
        .collect();

    let valid_tris: Vec<u64> = valid_tris_penta
        .iter()
        .cloned()
        .filter(|v| valid_tris_square.contains(v))
        .collect();

    // println!("{:?}", valid_tris);

    let sol: (&u64, u8, u8) = valid_tris
        .iter()
        .map(|v| {
            let (a, b, c, d) = *digits[0].get(v).unwrap();
            let front = concat(a as u64, b as u64) as u8;
            let back = concat(c as u64, d as u64) as u8;

            (v, front, back)
        })
        .filter(|(_, front, back)| penta_front.contains_key(back) && sq_front.contains_key(front))
        .find(|(v, front, back)| {
            println!("tri {}", v);
            let sq = sq_front.get(front).unwrap();
            let penta = penta_front.get(back).unwrap();
            println!("square {}", sq);
            println!("pentagon {}", penta);
            let sq_digits = digits[1].get(sq).unwrap();
            let penta_digits = digits[2].get(penta).unwrap();

            penta_digits.2 == sq_digits.0 && penta_digits.3 == sq_digits.1
        })
        // .map(|(v, f, b)| )
        .unwrap();

    println!("{:?}", sol);

    println!("{} {} {}", tri, penta, sq);

    let [a, b, c] = triangle
        .iter()
        .cloned()
        .enumerate()
        .filter(|(_i, tri)| nums[0].contains(tri))
        .flat_map(|a| {
            square
                .iter()
                .cloned()
                .enumerate()
                .filter(|(_i, sq)| nums[1].contains(sq))
                .map(move |b| [a, b])
        })
        .filter(|&[a, b]| {
            let (i, tri) = a;
            let (j, sq) = b;
            let tri_digits = digits[0].get(&tri).unwrap();
            let sq_digits = digits[1].get(&sq).unwrap();

            // i != j &&
            // (tri_digits.2 == sq_digits.0 && tri_digits.3 == sq_digits.1)
            // ||
            (tri_digits.0 == sq_digits.2 && tri_digits.1 == sq_digits.3)
        })
        .flat_map(|[a, b]| {
            pentagon
                .iter()
                .cloned()
                .enumerate()
                .filter(|(_i, penta)| nums[2].contains(penta))
                .map(move |c| [a, b, c])
        })
        .find(|&[a, b, c]| {
            let (i, tri) = a;
            let (j, sq) = b;
            let (k, penta) = c;
            let tri_digits = digits[0].get(&tri).unwrap();
            let sq_digits = digits[1].get(&sq).unwrap();
            let penta_digits = digits[2].get(&penta).unwrap();

            // (i != j && i != k && j != k)
            //     &&
            // (tri_digits.2 == sq_digits.0
            //     && tri_digits.3 == sq_digits.1
            //     && sq_digits.2 == penta_digits.0
            //     && sq_digits.3 == penta_digits.1
            //     && penta_digits.2 == tri_digits.0
            //     && penta_digits.3 == tri_digits.1)
            // ||
            (tri_digits.2 == penta_digits.0
                && tri_digits.3 == penta_digits.1
                && sq_digits.2 == tri_digits.0
                && sq_digits.3 == tri_digits.1
                && penta_digits.2 == sq_digits.0
                && penta_digits.3 == sq_digits.1)
        })
        .unwrap();

    for a in 0..triangle.len() {
        let tri = triangle[a];
        if !nums[0].contains(&tri) {
            continue;
        }
        let tri_digits = digits[0].get(&tri).unwrap();

        for b in a..pentagon.len() {
            let penta = pentagon[b];
            if !nums[2].contains(&penta) {
                continue;
            }
            let penta_digits = digits[2].get(&penta).unwrap();

            for c in b..square.len() {
                let square = square[c];
                if !nums[1].contains(&square) {
                    continue;
                }
                let square_digits = digits[1].get(&square).unwrap();

                if (tri_digits.2 == penta_digits.0 && tri_digits.3 == penta_digits.1)
                    && (penta_digits.2 == square_digits.0 && penta_digits.3 == square_digits.1)
                    && (square_digits.2 == tri_digits.0 && square_digits.3 == tri_digits.1)
                {
                    println!("{:?} {:?} {:?}", a, b, c);
                }
            }
        }
    }

    println!("{:?} {:?} {:?}", a, b, c);*/

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
