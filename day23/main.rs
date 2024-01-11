use std::collections::{HashSet, VecDeque};
use std::time::Instant;

enum Dir {
    Up,
    Down,
    Left,
    Right,
}
struct Map {
    map: Vec<Vec<u8>>,
}

impl Map {
    fn next_tile(&self, (row, col): (usize, usize), d: Dir) -> Option<(usize, usize)> {
        match d {
            Dir::Up if row != 0 => Some((row - 1, col)),
            Dir::Down if row != self.map.len() - 1 => Some((row + 1, col)),
            Dir::Left if col != 0 => Some((row, col - 1)),
            Dir::Right if col != self.map[0].len() - 1 => Some((row, col + 1)),
            _ => None,
        }
    }

    fn get_next_tiles(&self, (row, col): (usize, usize)) -> Vec<(usize, usize)> {
        let dirs = match self.map[row][col] {
            b'.' | b'^' | b'v' | b'<' | b'>' => vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right],
            // b'.' => vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right],
            // b'^' => vec![Dir::Up],
            // b'v' => vec![Dir::Down],
            // b'<' => vec![Dir::Left],
            // b'>' => vec![Dir::Right],
            _ => unreachable!(),
        };
        let mut next_tiles = Vec::new();
        for d in dirs {
            if let Some((newr, newc)) = self.next_tile((row, col), d) {
                match self.map[newr][newc] {
                    b'#' => {}
                    _ => next_tiles.push((newr, newc)),
                }
            }
        }
        next_tiles
    }

    fn get_longest_hike_bfs(&self) -> usize {
        let start = self.map[0]
            .iter()
            .enumerate()
            .filter(|(_, tile)| **tile == b'.')
            .map(|(idx, _)| (0, idx))
            .next()
            .unwrap();

        let _end = self.map[self.map.len() - 1]
            .iter()
            .enumerate()
            .filter(|(_, tile)| **tile == b'.')
            .map(|(idx, _)| (self.map.len() - 1, idx))
            .next()
            .unwrap();

        let mut ticks = 0;
        let mut to_process = VecDeque::new();
        to_process.push_back((start, start));
        while !to_process.is_empty() {
            let batch_sz = to_process.len();
            for _ in 0..batch_sz {
                let (prev, curr) = to_process.pop_front().unwrap();
                for (nextr, nextc) in self.get_next_tiles(curr) {
                    if (nextr, nextc) != prev {
                        to_process.push_back((curr, (nextr, nextc)));
                    }
                }
            }
            ticks += 1;
        }
        ticks - 1
    }

    fn get_longest_hike_dfs(
        &self,
        is_visited: &mut HashSet<(usize, usize)>,
        curr: (usize, usize),
        target: (usize, usize),
    ) -> usize {
        if curr == target {
            println!("{}", is_visited.len());
            return is_visited.len();
        }
        let mut longest_path = 0;
        for (nextr, nextc) in self.get_next_tiles(curr) {
            if !is_visited.contains(&(nextr, nextc)) {
                is_visited.insert((nextr, nextc));
                let res = self.get_longest_hike_dfs(is_visited, (nextr, nextc), target);
                if res > longest_path {
                    longest_path = res;
                }
                is_visited.remove(&(nextr, nextc));
            }
        }
        longest_path
    }
}
fn main() {
    let start = Instant::now();

    let input = include_str!("./input");
    let map = input
        .lines()
        .map(|line| line.bytes().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let m = Map { map };
    let mut is_visited = HashSet::new();
    let start_tile = m.map[0]
        .iter()
        .enumerate()
        .filter(|(_, tile)| **tile == b'.')
        .map(|(idx, _)| (0, idx))
        .next()
        .unwrap();
    let end_tile = m.map[m.map.len() - 1]
        .iter()
        .enumerate()
        .filter(|(_, tile)| **tile == b'.')
        .map(|(idx, _)| (m.map.len() - 1, idx))
        .next()
        .unwrap();

    println!(
        "{}",
        m.get_longest_hike_dfs(&mut is_visited, start_tile, end_tile)
    );
    println!("{:?}", start.elapsed());
}
