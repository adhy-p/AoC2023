use std::time::Instant;

type Cell = (i64, i64);

fn get_next_cell((row, col): Cell, dir: Dir, dist: i64) -> Cell {
    match dir {
        Dir::Down => (row + dist, col),
        Dir::Right => (row, col + dist),
        Dir::Up => (row - dist, col),
        Dir::Left => (row, col - dist),
    }
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

impl From<&str> for Dir {
    fn from(s: &str) -> Self {
        match s {
            "U" => Dir::Up,
            "D" => Dir::Down,
            "R" => Dir::Right,
            "L" => Dir::Left,
            _ => panic!("invalid &str"),
        }
    }
}

impl From<i64> for Dir {
    fn from(i: i64) -> Self {
        match i {
            0 => Dir::Right,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Up,
            _ => panic!("invalid int"),
        }
    }
}

fn main() {
    let start = Instant::now();

    let input = include_str!("./input");

    let mut curr_pos1: Cell = (0, 0);
    let mut curr_pos2: Cell = (0, 0);

    let mut total_area_1 = 0;
    let mut total_area_2 = 0;
    let mut total_dist1 = 0;
    let mut total_dist2 = 0;

    for line in input.lines() {
        let instr: Vec<&str> = line.split(' ').collect();
        assert_eq!(instr.len(), 3);

        /* part 1 */
        let dir = Dir::from(instr[0]);
        let dist = instr[1].parse::<i64>().expect("should be an integer");
        let next_pos1 = get_next_cell(curr_pos1, dir, dist);
        total_dist1 += dist;
        total_area_1 += (curr_pos1.0 * next_pos1.1) - next_pos1.0 * curr_pos1.1;
        curr_pos1 = next_pos1;

        /* part 2 */
        let color = instr[2]
            .trim_matches('(')
            .trim_matches(')')
            .trim_matches('#');
        let dist =
            i64::from_str_radix(&color[..color.len() - 1], 16).expect("should be a valid hex");
        let dir = Dir::from(
            i64::from_str_radix(&color[color.len() - 1..], 16).expect("should be a valid hex"),
        );

        let next_pos2 = get_next_cell(curr_pos2, dir, dist);
        total_dist2 += dist;
        total_area_2 += curr_pos2.0 * next_pos2.1 - next_pos2.0 * curr_pos2.1;
        curr_pos2 = next_pos2;
    }
    total_area_1 = total_area_1.abs() / 2;
    total_area_2 = total_area_2.abs() / 2;
    println!("{}", total_area_1 + total_dist1 / 2 + 1);
    println!("{}", total_area_2 + total_dist2 / 2 + 1);

    println!("{:?}", start.elapsed());
}
