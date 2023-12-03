use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_to_string("input")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let num_lines = lines.len();
    let mut numbers: Vec<Vec<(usize, usize)>> = vec![vec![]; num_lines];
    let mut symbols: Vec<Vec<usize>> = vec![vec![]; num_lines];
    /* part 2 */
    let mut gears: Vec<Vec<usize>> = vec![vec![]; num_lines];

    for (line_num, line) in lines.iter().enumerate() {
        let mut num_start_idx = line.len();
        for (idx, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    if num_start_idx != line.len() {
                        numbers[line_num].push((num_start_idx, idx - 1));
                        num_start_idx = line.len();
                    }
                }
                c if c.is_numeric() => {
                    if num_start_idx == line.len() {
                        num_start_idx = idx;
                    }
                }
                _ => {
                    if num_start_idx != line.len() {
                        numbers[line_num].push((num_start_idx, idx - 1));
                        num_start_idx = line.len();
                    }
                    symbols[line_num].push(idx);
                    if c == '*' {
                        /* part 2 */
                        gears[line_num].push(idx);
                    }
                }
            }
        }
        if num_start_idx != line.len() {
            numbers[line_num].push((num_start_idx, line.len() - 1));
        }
    }

    /* part 2 */
    let mut total_gear_ratio: u32 = 0;
    for (row, s) in gears.iter().enumerate() {
        for col in s {
            let mut count = 0;
            let mut curr_ratio = 1;
            if row > 0 {
                let (r, c) = add_gears(&lines, &mut numbers, row - 1, col);
                curr_ratio *= r;
                count += c;
            }
            let (r, c) = add_gears(&lines, &mut numbers, row, col);
            curr_ratio *= r;
            count += c;
            if row < num_lines - 1 {
                let (r, c) = add_gears(&lines, &mut numbers, row + 1, col);
                curr_ratio *= r;
                count += c;
            }
            if count == 2 {
                total_gear_ratio += curr_ratio;
            }
        }
    }

    /* part 1 */
    let mut total_part_sum: u32 = 0;
    for (row, s) in symbols.iter().enumerate() {
        for col in s {
            if row > 0 {
                total_part_sum += add_numbers(&lines, &mut numbers, row - 1, col);
            }
            total_part_sum += add_numbers(&lines, &mut numbers, row, col);
            if row < num_lines - 1 {
                total_part_sum += add_numbers(&lines, &mut numbers, row + 1, col);
            }
        }
    }

    println!("{}", total_part_sum);
    println!("{}", total_gear_ratio);
}

fn add_numbers(
    lines: &[String],
    numbers: &mut [Vec<(usize, usize)>],
    row: usize,
    col: &usize,
) -> u32 {
    let mut sum: u32 = 0;
    for (start_idx, end_idx) in numbers[row].iter_mut() {
        if *start_idx == lines[row].len() {
            continue;
        }
        if !(*col + 1 < *start_idx || *col - 1 > *end_idx) {
            sum += lines[row][*start_idx..=*end_idx].parse::<u32>().unwrap();
            // don't consider this number again
            *start_idx = lines[row].len();
            *end_idx = lines[row].len();
        }
    }
    sum
}

fn add_gears(
    lines: &[String],
    numbers: &mut [Vec<(usize, usize)>],
    row: usize,
    col: &usize,
) -> (u32, u32) {
    let mut count = 0;
    let mut ratio: u32 = 1;
    for (start_idx, end_idx) in numbers[row].iter_mut() {
        if !(*col + 1 < *start_idx || *col - 1 > *end_idx) {
            ratio *= lines[row][*start_idx..=*end_idx].parse::<u32>().unwrap();
            count += 1;
        }
    }
    (ratio, count)
}
