use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn main() {
    let input = read_to_string("./input").unwrap();
    let mut input = input.lines();
    let mut steps = 0;
    let mut graph: HashMap<&str, (&str, &str)> = HashMap::new();
    let instruction = input.next().unwrap();

    // println!("{instruction}");
    input.next();

    /* part 2 */
    let mut to_process: Vec<&str> = Vec::new();

    for line in input {
        let (src, dst) = line.split_once('=').unwrap();
        let (left, right) = dst.split_once(',').unwrap();
        let src = src.trim();
        let (left, right) = (left.trim(), right.trim());
        let left = &left[1..];
        let right = &right[..right.len() - 1];
        graph.insert(src, (left, right));
        if src.ends_with('A') {
            to_process.push(src);
        }
        // println!("{src} {left} {right}");
    }

    let mut node = "AAA";
    let dst = "ZZZ";
    'part1: loop {
        for i in instruction.chars() {
            if i == 'L' {
                node = graph.get(node).unwrap().0;
            } else {
                node = graph.get(node).unwrap().1;
            }
            steps += 1;
            if node == dst {
                println!("{}", steps);
                break 'part1;
            }
        }
    }

    /* part 2 */
    let num_nodes = to_process.len();
    let mut steps_vec: Vec<Vec<usize>> = vec![vec![]; num_nodes];
    let mut seen: Vec<HashSet<&str>> = vec![HashSet::new(); num_nodes];
    for i in 0..num_nodes {
        steps = 0;
        let mut node = to_process[i];
        'part2: loop {
            for instr in instruction.chars() {
                if instr == 'L' {
                    node = graph.get(node).unwrap().0;
                } else {
                    node = graph.get(node).unwrap().1;
                }
                steps += 1;
                if (node).ends_with('Z') {
                    /* apparently this is not necessary and we can break immediately */
                    if !seen[i].contains(node) {
                        // println!("{}: found 'Z' ({}) after {} steps", to_process[i], node, steps);
                        seen[i].insert(node);
                    } else {
                        // println!("({}) was already seen. exiting loop.", node);
                        break 'part2;
                    }
                    steps_vec[i].push(steps);
                }
            }
        }
    }
    // println!("{:?}", steps_vec);
    println!(
        "{:?}",
        steps_vec
            .into_iter()
            .map(|x| x.into_iter().fold(1, lcm))
            .fold(1, lcm)
    );
}
