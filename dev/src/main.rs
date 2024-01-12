use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = include_str!("./input.example.1");
    let mut graph = HashMap::new();
    for line in input.lines() {
        let (src, dst) = line.split_once(": ").unwrap();
        let dst = dst.split(' ').collect::<Vec<_>>();
        println!("{} {:?}", src, dst);
        for d in dst {
            // graph
            //     .entry(d)
            //     .and_modify(|s: &mut HashSet<&str>| {
            //         s.insert(src);
            //     })
            //     .or_insert(HashSet::from_iter(std::iter::once(src)));
            graph.entry(d).or_insert(HashSet::new()).insert(src);
            graph.entry(src).or_insert(HashSet::new()).insert(d);
        }
    }
    println!("{:?}", graph);

    // randomly select a start node
    let start_node = graph.keys().next().unwrap().clone();

    // Stoer-Wagner minimum cut algorithm
    // let ans = minimum_cut(&mut graph, start_node);
    // println!("{}", ans);
    println!("{:?}", start.elapsed());
}

fn minimum_cut(graph: &mut HashMap<&str, HashSet<&str>>, start_node: &str) -> usize {
    println!("{}", start_node);
    while graph.len() > 1 {
        minimum_cut_phase(graph, start_node);
    }
    0
}

fn minimum_cut_phase(graph: &mut HashMap<&str, HashSet<&str>>, start_node: &str) -> usize {
    0
}
