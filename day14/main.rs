use core::fmt;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::mem::swap;
use std::time::Instant;

fn calc_load(map: &[Vec<char>]) -> usize {
    map.iter()
        .rev()
        .enumerate()
        .map(|(idx, row)| row.iter().filter(|&&x| x == 'O').count() * (idx + 1))
        .sum()
}

enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn shift_left(map: &mut Vec<Vec<char>>) {
    let n_cols = map[0].len();
    for row in map {
        let mut dot_idx = n_cols;
        for i in 0..n_cols {
            match row[i] {
                'O' => {
                    if dot_idx != n_cols {
                        row.swap(dot_idx, i);
                        while row[dot_idx] != '.' {
                            dot_idx += 1;
                            if dot_idx == n_cols {
                                break;
                            }
                        }
                    }
                }
                '.' => {
                    if dot_idx == n_cols {
                        dot_idx = i;
                    }
                }
                '#' => {
                    dot_idx = n_cols;
                }
                _ => {}
            }
        }
    }
}

fn shift_right(map: &mut Vec<Vec<char>>) {
    let n_colss = map[0].len();
    for row in map {
        let mut dot_idx = n_colss;
        for i in (0..n_colss).rev() {
            match row[i] {
                'O' => {
                    if dot_idx != row.len() {
                        row.swap(dot_idx, i);
                        while row[dot_idx] != '.' {
                            if dot_idx == 0 {
                                dot_idx = n_colss;
                                break;
                            }
                            dot_idx -= 1;
                        }
                    }
                }
                '.' => {
                    if dot_idx == n_colss {
                        dot_idx = i;
                    }
                }
                '#' => {
                    dot_idx = n_colss;
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
        let mut dot_idx = n_rows;
        for row in 0..n_rows {
            match map[row][col] {
                'O' => {
                    if dot_idx != n_rows {
                        let (dot, rock) = map.split_at_mut(row);
                        swap(&mut rock[0][col], &mut dot[dot_idx][col]);
                        while map[dot_idx][col] != '.' {
                            dot_idx += 1;
                            if dot_idx == n_rows {
                                break;
                            }
                        }
                    }
                }
                '.' => {
                    if dot_idx == n_rows {
                        dot_idx = row;
                    }
                }
                '#' => {
                    dot_idx = n_rows;
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
        let mut dot_idx = n_rows;
        for row in (0..n_rows).rev() {
            match map[row][col] {
                'O' => {
                    if dot_idx != n_rows {
                        // split at mut is honestly annoying when rev is involved
                        map[dot_idx][col] = 'O';
                        map[row][col] = '.';
                        while map[dot_idx][col] != '.' {
                            dot_idx -= 1;
                            if dot_idx == 0 {
                                dot_idx = n_rows;
                                break;
                            }
                        }
                    }
                }
                '.' => {
                    if dot_idx == n_rows {
                        dot_idx = row;
                    }
                }
                '#' => {
                    dot_idx = n_rows;
                }
                _ => {}
            }
        }
    }
}

fn tilt(map: &mut Vec<Vec<char>>, dir: Direction) {
    match dir {
        Direction::Up => shift_up(map),
        Direction::Left => shift_left(map),
        Direction::Down => {
            shift_down(map);
        }
        Direction::Right => shift_right(map),
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
    let mut total: usize = 0;
    let mut map: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();
    const NUM_ITER: usize = 1_000_000_000;
    const DIRS: [Direction; 4] = [
        Direction::Up,
        Direction::Left,
        Direction::Down,
        Direction::Right,
    ];
    let mut history: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    history.insert(map.clone(), 0);
    let mut remain = usize::MAX;
    for i in 1..=NUM_ITER {
        for d in DIRS {
            tilt(&mut map, d);
        }
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
        for d in DIRS {
            tilt(&mut map, d);
        }
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