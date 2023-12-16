use std::collections::VecDeque;
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let start = Instant::now();

    let input = read_to_string("./input").unwrap();
    let input: Vec<Vec<u8>> = input.lines().map(|row| row.bytes().collect()).collect();

    let mut starting_points: Vec<((usize, usize), Dir)> = vec![];
    for r in 0..input.len() {
        starting_points.push(((r, 0), Dir::Right));
        starting_points.push(((r, input[0].len() - 1), Dir::Left));
    }
    for c in 0..input[0].len() {
        starting_points.push(((0, c), Dir::Down));
        starting_points.push(((input.len() - 1, c), Dir::Up));
    }

    let mut best = 0;
    for s in starting_points {
        let res = simulate(&input, s);
        if res > best {
            best = res;
        }
    }
    println!("{best}");
    println!("{:?}", start.elapsed());
}

fn simulate(input: &Vec<Vec<u8>>, start_point: ((usize, usize), Dir)) -> usize {
    let nrows = input.len();
    let ncols = input[0].len();

    let mut is_visited = vec![vec![[false, false, false, false]; ncols]; nrows]; // up down left right
    let mut to_process: VecDeque<((usize, usize), Dir)> = VecDeque::new();
    let (start_row, start_col) = start_point.0;
    let start_dir = start_point.1;
    is_visited[start_row][start_col][start_dir as usize] = true;
    to_process.push_back(start_point);

    while !to_process.is_empty() {
        let ((row, col), dir) = to_process.pop_front().unwrap();
        match input[row][col] {
            b'.' => visit_next(&mut to_process, &mut is_visited, (row, col), dir),
            b'|' => {
                if is_vertical(&dir) {
                    visit_next(&mut to_process, &mut is_visited, (row, col), dir)
                } else {
                    for next_dir in [Dir::Up, Dir::Down] {
                        visit_next(&mut to_process, &mut is_visited, (row, col), next_dir);
                    }
                }
            }
            b'-' => {
                if !is_vertical(&dir) {
                    visit_next(&mut to_process, &mut is_visited, (row, col), dir)
                } else {
                    for next_dir in [Dir::Left, Dir::Right] {
                        visit_next(&mut to_process, &mut is_visited, (row, col), next_dir);
                    }
                }
            }
            b'/' => {
                let next_dir = match dir {
                    Dir::Up => Dir::Right,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Down,
                    Dir::Right => Dir::Up,
                };
                visit_next(&mut to_process, &mut is_visited, (row, col), next_dir)
            }
            b'\\' => {
                let next_dir = match dir {
                    Dir::Up => Dir::Left,
                    Dir::Down => Dir::Right,
                    Dir::Left => Dir::Up,
                    Dir::Right => Dir::Down,
                };
                visit_next(&mut to_process, &mut is_visited, (row, col), next_dir)
            }
            _ => {}
        }
    }

    let mut total = 0;
    for i in 0..nrows {
        for j in 0..ncols {
            for k in 0..4 {
                if is_visited[i][j][k] {
                    total += 1;
                    break;
                }
            }
        }
    }
    total
}

fn is_vertical(d: &Dir) -> bool {
    matches!(d, Dir::Up | Dir::Down)
}

fn get_next_pos((row, col): (usize, usize), dir: &Dir) -> (usize, usize) {
    match dir {
        Dir::Up => (row.overflowing_add_signed(-1).0, col),
        Dir::Down => (row + 1, col),
        Dir::Left => (row, col.overflowing_add_signed(-1).0),
        Dir::Right => (row, col + 1),
    }
}

fn is_within_bounds((row, col): (usize, usize), (nrows, ncols): (usize, usize)) -> bool {
    row < nrows && col < ncols
}

fn visit_next(
    to_process: &mut VecDeque<((usize, usize), Dir)>,
    is_visited: &mut Vec<Vec<[bool; 4]>>,
    (row, col): (usize, usize),
    dir: Dir,
) {
    let (next_row, next_col) = get_next_pos((row, col), &dir);
    if is_within_bounds(
        (next_row, next_col),
        (is_visited.len(), is_visited[0].len()),
    ) && !is_visited[next_row][next_col][dir as usize]
    {
        // just continue
        is_visited[next_row][next_col][dir as usize] = true;
        to_process.push_back(((next_row, next_col), dir));
    }
}
