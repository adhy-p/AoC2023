use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::read_to_string;

fn main() {
    let input: String = read_to_string("./input").unwrap();
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect();

    let nrows = input.len();
    let ncols = input[0].len();

    let mut graph: Vec<Vec<HashSet<(usize, usize)>>> = vec![vec![HashSet::new(); ncols]; nrows];
    let mut distances: Vec<Vec<i64>> = vec![vec![-1; ncols]; nrows];

    let mut to_process: VecDeque<(usize, usize)> = VecDeque::new();
    let mut start: (usize, usize) = (0, 0);

    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, chr) in row.iter().enumerate() {
            match chr {
                '|' => {
                    add_neighbors(&mut graph, (row_idx, col_idx), (-1, 0));
                    add_neighbors(&mut graph, (row_idx, col_idx), (1, 0));
                }
                '-' => {
                    add_neighbors(&mut graph, (row_idx, col_idx), (0, -1));
                    add_neighbors(&mut graph, (row_idx, col_idx), (0, 1));
                }
                'L' => {
                    add_neighbors(&mut graph, (row_idx, col_idx), (-1, 0));
                    add_neighbors(&mut graph, (row_idx, col_idx), (0, 1));
                }
                'J' => {
                    add_neighbors(&mut graph, (row_idx, col_idx), (-1, 0));
                    add_neighbors(&mut graph, (row_idx, col_idx), (0, -1));
                }
                '7' => {
                    add_neighbors(&mut graph, (row_idx, col_idx), (0, -1));
                    add_neighbors(&mut graph, (row_idx, col_idx), (1, 0));
                }
                'F' => {
                    add_neighbors(&mut graph, (row_idx, col_idx), (0, 1));
                    add_neighbors(&mut graph, (row_idx, col_idx), (1, 0));
                }
                'S' => {
                    start = (row_idx, col_idx);
                    add_neighbors(&mut graph, start, (0, 1));
                    add_neighbors(&mut graph, start, (1, 0));
                    add_neighbors(&mut graph, start, (0, -1));
                    add_neighbors(&mut graph, start, (-1, 0));
                    to_process.push_back(start);
                    distances[row_idx][col_idx] = 0;
                }
                _ => (),
            }
        }
    }

    let mut steps = 1;
    let mut max_steps = 0;

    let mut main_loop: HashSet<(usize, usize)> = HashSet::new();
    main_loop.insert(start);

    while !to_process.is_empty() {
        let iter_count = to_process.len();
        for _ in 0..iter_count {
            let (row, col) = to_process.pop_front().unwrap();
            for (next_row, next_col) in &graph[row][col] {
                if graph[*next_row][*next_col].contains(&(row, col))
                    && distances[*next_row][*next_col] == -1
                {
                    distances[*next_row][*next_col] = steps;
                    max_steps = steps;
                    to_process.push_back((*next_row, *next_col));
                    main_loop.insert((*next_row, *next_col));
                }
            }
        }
        steps += 1;
    }
    println!("{}", max_steps);
    // println!("{:?}", main_loop);
    // println!("{}", main_loop.len());

    let mut num_inside_loop = 0;
    for (row_idx, row) in input.iter().enumerate() {
        for (col_idx, _chr) in row.iter().enumerate() {
            if main_loop.contains(&(row_idx, col_idx)) {
                continue;
            }
            // shoot a ray to the left
            // if it hits the shape odd number of times -> inside
            // else -> outside
            let mut hits = 0;
            for curr_col in 0..col_idx {
                if main_loop.contains(&(row_idx, curr_col))
                    && (input[row_idx][curr_col] == 'F'
                        || input[row_idx][curr_col] == '7'
                        || input[row_idx][curr_col] == '|')
                {
                    hits += 1;
                }
            }
            if hits % 2 == 1 {
                num_inside_loop += 1;
            }
        }
    }
    println!("{}", num_inside_loop);
}

fn add_neighbors(
    graph: &mut Vec<Vec<HashSet<(usize, usize)>>>,
    coord: (usize, usize),
    delta: (i64, i64),
) {
    let row: i64 = coord.0 as i64 + delta.0;
    let col: i64 = coord.1 as i64 + delta.1;
    let nrows = graph.len() as i64;
    let ncols = graph[0].len() as i64;
    if row >= 0 && row < nrows && col >= 0 && col < ncols {
        let row = row as usize;
        let col = col as usize;
        graph[coord.0][coord.1].insert((row, col));
    }
}
