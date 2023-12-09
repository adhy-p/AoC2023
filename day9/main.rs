use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input").unwrap();
    let mut total_next = 0;
    let mut total_prev = 0;
    for line in input.lines() {
        let numbers: Vec<i64> = line.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
        let (prev_num, next_num) = extrapolate(&numbers);
        // println!("{:?}, prev: {}, next: {}", numbers, prev_num, next_num);
        total_next += next_num;
        total_prev += prev_num;
    }
    println!("{}", total_next);
    println!("{}", total_prev);
}

fn extrapolate(numbers: &Vec<i64>) -> (i64, i64) {
    let s = numbers.iter().filter(|x| **x != 0).count();
    if s == 0 {
        return (0, 0);
    }

    let diffs = numbers.windows(2).map(|x| x[1] - x[0]).collect();
    // println!("diffs {:?}", diffs);
    let (prev_diff, next_diff) = extrapolate(&diffs);
    // println!("extrapolated diffs: prev: {}, next: {}", prev_diff, next_diff);

    let next_number = if let Some(last) = numbers.last() {
        last + next_diff
    } else {
        panic!("empty array. should never happen");
    };

    let prev_number = if let Some(first) = numbers.first() {
        first - prev_diff
    } else {
        panic!("empty array. should never happen");
    };

    (prev_number, next_number)
}
