use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = read_to_string("./input").unwrap();
    let mut total_1: usize = 0;
    let mut total_2: usize = 0;
    let mut map: Vec<&str> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            // process current map
            if let Some(v) = find_reflective_row(&map, 0) {
                total_1 += 100 * (v + 1);
            }
            if let Some(v) = find_reflective_col(&map, 0) {
                total_1 += v + 1;
            }
            if let Some(v) = find_reflective_row(&map, 1) {
                total_2 += 100 * (v + 1);
            }
            if let Some(v) = find_reflective_col(&map, 1) {
                total_2 += v + 1;
            }
            map.clear();
        } else {
            map.push(line);
        }
    }
    if let Some(v) = find_reflective_row(&map, 0) {
        total_1 += 100 * (v + 1);
    }
    if let Some(v) = find_reflective_col(&map, 0) {
        total_1 += v + 1;
    }
    if let Some(v) = find_reflective_row(&map, 1) {
        total_2 += 100 * (v + 1);
    }
    if let Some(v) = find_reflective_col(&map, 1) {
        total_2 += v + 1;
    }
    println!("{}", total_1);
    println!("{}", total_2);
    println!("{:?}", start.elapsed());
}

fn find_reflective_row(map: &[&str], num_diff: usize) -> Option<usize> {
    for top in 0..map.len() - 1 {
        let bot = top + 1;
        let mut itop = top;
        let mut ibot = bot;
        let mut diff = 0;
        loop {
            for (c1, c2) in map[itop].chars().zip(map[ibot].chars()) {
                if c1 != c2 {
                    diff += 1;
                }
            }
            if itop == 0 || ibot == map.len() - 1 {
                break;
            }
            itop -= 1;
            ibot += 1;
        }
        if diff == num_diff {
            return Some(top);
        }
    }
    None
}

fn find_reflective_col(map: &[&str], num_diff: usize) -> Option<usize> {
    for left in 0..map[0].len() - 1 {
        let right = left + 1;
        let mut ileft = left;
        let mut iright = right;
        let mut diff = 0;
        loop {
            let lcol = map.iter().map(|&x| x.chars().nth(ileft).unwrap());
            let rcol = map.iter().map(|&x| x.chars().nth(iright).unwrap());
            for (c1, c2) in lcol.zip(rcol) {
                if c1 != c2 {
                    diff += 1;
                }
            }
            if ileft == 0 || iright == map[0].len() - 1 {
                break;
            }
            ileft -= 1;
            iright += 1;
        }
        if diff == num_diff {
            return Some(left);
        }
    }
    None
}
