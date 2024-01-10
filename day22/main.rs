use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

#[derive(Debug)]
struct Brick {
    x: (i64, i64),
    y: (i64, i64),
    z: (i64, i64),
}

fn main() {
    let start = Instant::now();

    let input = include_str!("./input");

    let mut bricks = input
        .lines()
        .map(|row| {
            let (start, end) = row.split_once('~').unwrap();

            let mut start = start.split(',');
            let x0 = start.next().unwrap().parse::<i64>().unwrap();
            let y0 = start.next().unwrap().parse::<i64>().unwrap();
            let z0 = start.next().unwrap().parse::<i64>().unwrap();

            let mut end = end.split(',');
            let x1 = end.next().unwrap().parse::<i64>().unwrap();
            let y1 = end.next().unwrap().parse::<i64>().unwrap();
            let z1 = end.next().unwrap().parse::<i64>().unwrap();

            Brick {
                x: (x0, x1),
                y: (y0, y1),
                z: (z0, z1),
            }
        })
        .collect::<Vec<_>>();
    bricks.sort_by(|lhs, rhs| lhs.z.cmp(&rhs.z));
    // println!("{:?}", bricks);

    let n = bricks.len();

    let mut support_graph: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    let mut brick_map: HashMap<(i64, i64), (usize, i64)> = HashMap::new();
    let mut bad_bricks = HashSet::new();

    for (idx, brick) in bricks.into_iter().enumerate() {
        let Brick {
            x: (x0, x1),
            y: (y0, y1),
            z: (z0, z1),
        } = brick;

        let mut supports = HashSet::new();
        let mut highest_support = 0;
        for x in x0..=x1 {
            for y in y0..=y1 {
                if let Some((node_idx, height)) = brick_map.get(&(x, y)) {
                    if *height > highest_support {
                        supports.clear();
                        supports.insert(*node_idx);
                        highest_support = *height;
                    } else if *height == highest_support {
                        supports.insert(*node_idx);
                    }
                }
            }
        }
        if supports.len() == 1 {
            let bad = *supports.iter().next().unwrap();
            bad_bricks.insert(bad);
        }
        for s in supports {
            support_graph[s].insert(idx);
        }

        for x in x0..=x1 {
            for y in y0..=y1 {
                brick_map.insert((x, y), (idx, highest_support + (z1 - z0 + 1)));
            }
        }
        // println!("{:?}", brick_map);
    }

    let num_can_remove = n - bad_bricks.len();
    println!("{}", num_can_remove);

    let mut in_degree = vec![0; n];
    for neighbors in support_graph.iter() {
        for &n in neighbors {
            in_degree[n] += 1;
        }
    }

    let mut total_falls = 0;
    for i in bad_bricks {
        let mut in_deg = in_degree.clone();
        let mut q = VecDeque::new();
        let mut curr_falls = 0;
        q.push_back(i);

        while let Some(node) = q.pop_front() {
            for &neighbors in support_graph[node].iter() {
                in_deg[neighbors] -= 1;
                if in_deg[neighbors] == 0 {
                    q.push_back(neighbors);
                    curr_falls += 1;
                }
            }
        }
        total_falls += curr_falls;
    }
    println!("{}", total_falls);
    println!("{:?}", start.elapsed());
}
