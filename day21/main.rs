use std::{collections::HashSet, time::Instant};

type Cell = (i64, i64);
type Grid = Vec<Vec<char>>;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

const DIRS: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Right, Dir::Left];

struct Map {
    start_node: Cell,
    grid: Grid,
}

impl Map {
    fn new(input: &str) -> Self {
        let grid: Grid = input.lines().map(|lines| lines.chars().collect()).collect();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 'S' {
                    return Map {
                        start_node: (i as i64, j as i64),
                        grid,
                    };
                }
            }
        }
        unreachable!();
    }

    fn get_next_cell(&self, (row, col): Cell, dir: Dir) -> Option<Cell> {
        // part 1
        // match dir {
        //     Dir::Down if row != self.grid.len() - 1 => Some((row + 1, col)),
        //     Dir::Right if col != self.grid[0].len() - 1 => Some((row, col + 1)),
        //     Dir::Up if row != 0 => Some((row - 1, col)),
        //     Dir::Left if col != 0 => Some((row, col - 1)),
        //     _ => None,
        // }

        match dir {
            Dir::Down => Some((row + 1, col)),
            Dir::Right => Some((row, col + 1)),
            Dir::Up => Some((row - 1, col)),
            Dir::Left => Some((row, col - 1)),
        }
    }

    fn simulate(&self, num_steps: usize) -> i64 {
        // not used in this solution, but a good observation:
        // if we can reach a grid x with even number of steps remaining,
        // we can always go back to that grid by moving back and forth
        // until we run out of steps.
        let mut to_process = HashSet::new();
        to_process.insert(self.start_node);
        for _ in 1..=num_steps {
            if to_process.is_empty() {
                break;
            }
            let mut tmp = HashSet::new();
            for (row, col) in to_process.into_iter() {
                // println!("{:?}", (row, col));
                for dir in DIRS {
                    if let Some((next_row, next_col)) = Self::get_next_cell(self, (row, col), dir) {
                        // TODO: refactor the modulo (rem_euclid) to a separate fn
                        if self.grid[next_row.rem_euclid(self.grid.len() as i64) as usize]
                            [next_col.rem_euclid(self.grid[0].len() as i64) as usize]
                            == '#'
                            || tmp.contains(&(next_row, next_col))
                        {
                            continue;
                        }
                        tmp.insert((next_row, next_col));
                    }
                }
            }
            to_process = tmp;
        }
        to_process.len() as i64
    }
}

fn part2(m: Map) {
    // hoping to solve this by myself one day

    // HyperNeutrino: https://www.youtube.com/watch?v=C5wYxR6ZAPM
    // let f(x) be the number of points that can be explored after x steps
    // we want to find f(x), f(x + 2N), f(x + 4N), ..., f(x + 2KN)
    // where K is a constant and N is the length of the grid
    // x + 2KN must be equal to the number of steps we want,
    // thus x must be equal to 26501365 % 2N
    let (y1, y2, y3) = (
        m.simulate(65),
        m.simulate(65 + 131),
        m.simulate(65 + 2 * 131),
    );
    // println!("{y1} {y2} {y3}");
    let a = y1 / 2 - y2 + y3 / 2;
    let b = -3 * (y1 / 2) + 2 * y2 - y3 / 2;
    let c = y1;
    let x = (26501365 - 65) / 131;
    let y = a * x * x + b * x + c;
    println!("{}", y);
}
fn main() {
    let start = Instant::now();

    let input = include_str!("./input");
    // println!("{:?}", instr);
    let m = Map::new(input);
    let total = m.simulate(64);
    println!("{}", total);

    part2(m);

    println!("{:?}", start.elapsed());
}
