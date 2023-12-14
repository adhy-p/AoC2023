use core::fmt;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

fn calc_load(map: &[Vec<char>]) -> usize {
    map.iter()
        .rev()
        .enumerate()
        .map(|(idx, row)| row.iter().filter(|&&x| x == 'O').count() * (idx + 1))
        .sum()
}

fn shift_left(map: &mut Vec<Vec<char>>) {
    let n_rows = map.len();
    let n_cols = map[0].len();

    for row in 0..n_rows {
        let mut dot_idx = 0;
        for col in 0..n_cols {
            match map[row][col] {
                'O' if dot_idx != n_cols => {
                    while dot_idx < col && map[row][dot_idx] != '.' {
                        dot_idx += 1;
                        if dot_idx == n_cols {
                            break;
                        }
                    }
                    if dot_idx != col {
                        map[row][dot_idx] = 'O';
                        map[row][col] = '.';
                    }
                }
                '#' => {
                    dot_idx = col;
                }
                _ => {}
            }
        }
    }
}

fn shift_right(map: &mut Vec<Vec<char>>) {
    let n_rows = map.len();
    let n_cols = map[0].len();

    for row in 0..n_rows {
        let mut dot_idx = n_cols - 1;
        for col in (0..n_cols).rev() {
            match map[row][col] {
                'O' => {
                    while dot_idx > col && map[row][dot_idx] != '.' {
                        dot_idx -= 1;
                    }
                    if dot_idx != col {
                        map[row][col] = '.';
                        map[row][dot_idx] = 'O';
                    }
                }
                '#' => {
                    dot_idx = col;
                }
                _ => {}
            }
        }
    }
}

fn shift_up(map: &mut Vec<Vec<char>>) {
    let n_rows = map.len();
    let n_cols = map[0].len();

    for col in 0..n_cols {
        let mut dot_idx = 0;
        for row in 0..n_rows {
            match map[row][col] {
                'O' if dot_idx != n_rows => {
                    while dot_idx < row && map[dot_idx][col] != '.' {
                        dot_idx += 1;
                    }
                    if dot_idx != row {
                        map[dot_idx][col] = 'O';
                        map[row][col] = '.';
                    }
                }
                '#' => {
                    dot_idx = row;
                }
                _ => {}
            }
        }
    }
}

fn shift_down(map: &mut Vec<Vec<char>>) {
    let n_rows = map.len();
    let n_cols = map[0].len();

    for col in 0..n_cols {
        let mut dot_idx = n_rows - 1;
        for row in (0..n_rows).rev() {
            match map[row][col] {
                'O' => {
                    while dot_idx > row && map[dot_idx][col] != '.' {
                        dot_idx -= 1;
                    }
                    if dot_idx != row {
                        map[dot_idx][col] = 'O';
                        map[row][col] = '.';
                    }
                }
                '#' => {
                    dot_idx = row;
                }
                _ => {}
            }
        }
    }
}

struct VecPrinter(Vec<Vec<char>>);

impl fmt::Display for VecPrinter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = &self.0;
        for r in v {
            for c in r {
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

/* part 2 */
fn main() {
    let start = Instant::now();

    let input = read_to_string("./input").unwrap();
    let mut map: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    let mut total: usize = 0;
    const NUM_ITER: usize = 1_000_000_000;

    let mut history: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    history.insert(map.clone(), 0);

    let mut remain = 0;
    for i in 1..=NUM_ITER {
        shift_up(&mut map);
        shift_left(&mut map);
        shift_down(&mut map);
        shift_right(&mut map);

        if !history.contains_key(&map) {
            history.insert(map.clone(), i);
        } else {
            let prev = history.get(&map).unwrap();
            let cycle_len = i - prev;
            remain = (NUM_ITER - i) % cycle_len;
            // println!("loop detected: ({}..{}), len {}.", prev, i, cycle_len);
            // println!("remain iter: {remain}");
            break;
        }
    }

    for _ in 0..remain {
        shift_up(&mut map);
        shift_left(&mut map);
        shift_down(&mut map);
        shift_right(&mut map);
    }
    total += calc_load(&map);
    println!("{}", total);
    println!("{:?}", start.elapsed());
}

/* part 1 */
// fn main() {
//     let start = Instant::now();
//     let input = read_to_string("./input").unwrap();
//     let mut total: usize = 0;
//     let map: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
//     for col_idx in 0..map[0].len() {
//         let mut rock_stack = 0;
//         for (row_idx, row) in map.iter().rev().enumerate() {
//             match row[col_idx] {
//                 'O' => {
//                     rock_stack += 1;
//                 }
//                 '#' => {
//                     if rock_stack == 0 {
//                         continue;
//                     }
//                     total += ((row_idx - rock_stack + 1) * 2 + (rock_stack - 1)) * rock_stack / 2;
//                     rock_stack = 0;
//                 }
//                 '.' => {}
//                 _ => {
//                     panic!("invalid rock");
//                 }
//             }
//         }
//         if rock_stack != 0 {
//             total += ((map.len() - rock_stack + 1) * 2 + (rock_stack - 1)) * rock_stack / 2;
//         }
//     }
//     println!("{}", total);
//     println!("{:?}", start.elapsed());
// }
