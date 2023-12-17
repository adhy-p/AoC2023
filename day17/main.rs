use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::{Hash, Hasher};
use std::time::Instant;

// const MAX_STRAIGHT: usize = 3;
const ULTRA_MIN_STRAIGHT: usize = 4;
const ULTRA_MAX_STRAIGHT: usize = 10;
const DIRS: [Dir; 4] = [Dir::Down, Dir::Right, Dir::Up, Dir::Left];

fn get_next_cell((row, col): (usize, usize), dir: Dir) -> (usize, usize) {
    match dir {
        Dir::Down => (row + 1, col),
        Dir::Right => (row, col + 1),
        Dir::Up => (row.overflowing_add_signed(-1).0, col),
        Dir::Left => (row, col.overflowing_add_signed(-1).0),
    }
}

fn get_reverse(dir: Dir) -> Dir {
    match dir {
        Dir::Down => Dir::Up,
        Dir::Right => Dir::Left,
        Dir::Up => Dir::Down,
        Dir::Left => Dir::Right,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Down,
    Right,
    Up,
    Left,
}

#[derive(Debug, Clone, Copy)]
struct State {
    cell: (usize, usize),
    dir: Dir,
    n_straight: usize,
    total_heat_loss: usize,
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cell == other.cell
            && self.dir == other.dir
            // && self.total_heat_loss == other.total_heat_loss
            && self.n_straight == other.n_straight
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.cell.hash(state);
        self.dir.hash(state);
        self.n_straight.hash(state);
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare the rest - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .total_heat_loss
            .cmp(&self.total_heat_loss)
            .then_with(|| self.dir.cmp(&other.dir))
            .then_with(|| self.cell.cmp(&other.cell))
            .then_with(|| self.n_straight.cmp(&other.n_straight))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn get_next_states(&self, input: &Vec<Vec<u8>>) -> Vec<Self> {
        let mut v = Vec::new();
        for dir in DIRS {
            if self.n_straight < ULTRA_MIN_STRAIGHT && dir != self.dir {
                // remove this for part 1
                continue;
            }
            let n_straight = if dir == self.dir {
                self.n_straight + 1
            } else {
                1
            };
            // change ULTRA_MAX_STRAIGHT to MAX_STRAIGHT for part 1
            if n_straight > ULTRA_MAX_STRAIGHT || dir == get_reverse(self.dir) {
                continue;
            }
            let cell = get_next_cell(self.cell, dir);
            let (next_row, next_col) = cell;
            if next_row >= input.len() || next_col >= input[0].len() {
                continue;
            }
            let total_heat_loss = self.total_heat_loss + input[next_row][next_col] as usize;
            v.push(State {
                cell,
                dir,
                n_straight,
                total_heat_loss,
            })
        }
        v
    }
}

fn main() {
    let start = Instant::now();

    let input = include_str!("./input");
    let input: Vec<Vec<u8>> = input
        .lines()
        .map(|row| row.bytes().map(|b| b - b'0').collect())
        .collect();

    let nrows = input.len();
    let ncols = input[0].len();

    let mut to_process: BinaryHeap<State> = BinaryHeap::new();
    let mut heat_losses: HashMap<State, usize> = HashMap::new();
    let init_states = [
        State {
            cell: (0, 1),
            dir: Dir::Right,
            n_straight: 1,
            total_heat_loss: input[0][1] as usize,
        },
        State {
            cell: (1, 0),
            dir: Dir::Down,
            n_straight: 1,
            total_heat_loss: input[1][0] as usize,
        },
    ];
    for i in init_states {
        to_process.push(i);
    }

    while let Some(state) = to_process.pop() {
        // println!("state: {:?}", state);
        if state.cell == (nrows - 1, ncols - 1) {
            println!("{}", state.total_heat_loss);
            break;
        }
        if let Some(&l) = heat_losses.get(&state) {
            if l < state.total_heat_loss {
                continue;
            }
        }
        let next_states = state.get_next_states(&input);
        // println!("next_state {:?}", next_states);
        for next_state in next_states {
            if let Some(&l) = heat_losses.get(&next_state) {
                if l < next_state.total_heat_loss {
                    continue;
                }
            }
            to_process.push(next_state);
            heat_losses.insert(next_state, next_state.total_heat_loss);
        }
        // println!("{:?}", to_process);
        // println!();
    }
    println!("{:?}", start.elapsed());
}
