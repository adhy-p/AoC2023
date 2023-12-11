use std::cmp::{max, min};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input").unwrap();
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let nrows = input.len();
    let ncols = input[0].len();

    let mut empty_rows: Vec<usize> = vec![];
    let mut empty_cols: Vec<usize> = vec![];

    for i in 0..nrows {
        if input[i].iter().all(|x| *x == '.') {
            empty_rows.push(i);
        }
    }

    for i in 0..ncols {
        if input.iter().map(|x| x[i]).all(|x| x == '.') {
            empty_cols.push(i);
        }
    }

    let mut galaxies: Vec<(usize, usize)> = vec![];
    for i in 0..nrows {
        for j in 0..ncols {
            if input[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }

    // println!("{:?}", empty_rows);
    // println!("{:?}", empty_cols);
    // println!("{:?}", galaxies);

    let mut total_dists_1 = 0;
    let mut total_dists_2 = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let dist =
                galaxies[j].0.abs_diff(galaxies[i].0) + galaxies[j].1.abs_diff(galaxies[i].1);
            let exp_row = {
                let low = min(galaxies[i].0, galaxies[j].0);
                let hi = max(galaxies[i].0, galaxies[j].0);
                empty_rows.iter().filter(|r| **r > low && **r < hi).count()
            };
            let exp_col = {
                let low = min(galaxies[i].1, galaxies[j].1);
                let hi = max(galaxies[i].1, galaxies[j].1);
                empty_cols.iter().filter(|r| **r > low && **r < hi).count()
            };
            total_dists_1 += dist + exp_row + exp_col;
            total_dists_2 += dist + exp_row * 999_999 + exp_col * 999_999;
        }
    }
    println!("{}", total_dists_1);
    println!("{}", total_dists_2);
}
