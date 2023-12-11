use std::cmp::{max, min};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input").unwrap();
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let nrows = input.len();
    let ncols = input[0].len();
    let empty_rows: Vec<usize> = (0..nrows)
        .filter(|i| input[*i].iter().all(|x| *x == '.'))
        .collect();
    let empty_cols: Vec<usize> = (0..ncols)
        .filter(|i| input.iter().map(|x| x[*i]).all(|x| x == '.'))
        .collect();

    let mut galaxies: Vec<(usize, usize)> = input
        .iter()
        .enumerate()
        .flat_map(|(row_idx, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, ch)| **ch == '#')
                .map(move |(col_idx, _)| (row_idx, col_idx))
        })
        .collect();

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