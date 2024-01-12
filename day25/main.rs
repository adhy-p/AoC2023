use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let input = include_str!("./input");
    let mut graph = BTreeMap::new();
    for line in input.lines() {
        let (src, dst) = line.split_once(": ").unwrap();
        let dst = dst.split(' ').collect::<Vec<_>>();
        for d in dst {
            graph.entry(d).or_insert(BTreeMap::new()).insert(src, 1);
            graph.entry(src).or_insert(BTreeMap::new()).insert(d, 1);
        }
    }
    // let mut graph = gen_test();
    println!("{:?}", graph);

    // randomly select a start node
    let start_node = *graph.keys().next().unwrap();

    // Stoer-Wagner minimum cut algorithm
    // https://dl.acm.org/doi/pdf/10.1145/263867.263872
    let ans = minimum_cut(&mut graph, start_node);
    println!("{}", ans);
    println!("{:?}", start.elapsed());
}

fn minimum_cut<'a>(
    graph: &mut BTreeMap<&'a str, BTreeMap<&'a str, usize>>,
    start_node: &'a str,
) -> usize {
    // println!("start_node: {}", start_node);
    let n = graph.len();
    let mut min_cut = graph.len();
    // a map that keep tracks nodes that has 'consumed' other nodes
    let mut consumed_nodes: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::from_iter(
        graph
            .keys()
            .map(|k| (*k, BTreeSet::from_iter(std::iter::once(*k)))),
    );
    while graph.len() > 1 {
        let (cut_phase_cost, partition) = minimum_cut_phase(graph, &mut consumed_nodes, start_node);
        if cut_phase_cost < min_cut {
            min_cut = cut_phase_cost;
        }
        if min_cut == 3 {
            let ans = partition.len() * (n - partition.len());
            println!("ans: {}", ans);
            return min_cut;
        }
        println!("graph len: {}, cut_cost: {}", graph.len(), cut_phase_cost);
    }
    min_cut
}

fn minimum_cut_phase<'a>(
    graph: &mut BTreeMap<&'a str, BTreeMap<&'a str, usize>>,
    consumed_nodes: &mut BTreeMap<&'a str, BTreeSet<&'a str>>,
    start_node: &'a str,
) -> (usize, BTreeSet<&'a str>) {
    let mut a_set: BTreeSet<&str> = BTreeSet::new();
    let mut last_two_added = VecDeque::new();
    a_set.insert(start_node);
    last_two_added.push_back(start_node);

    while a_set.len() != graph.len() {
        let v = get_most_tightly_connected_vertex(graph, &a_set);
        // println!("added to a_set: {}", v);
        a_set.insert(v);
        last_two_added.push_back(v);
        if last_two_added.len() > 2 {
            last_two_added.pop_front();
        }
    }
    assert_eq!(last_two_added.len(), 2);
    // println!("{:?}", last_two_added);
    let last = last_two_added.pop_back().unwrap();
    let second_last = last_two_added.pop_back().unwrap();

    // cut of the phase: the cut of V that separates the vertex added last
    // from the rest of the graph
    let cut_cost = graph.get(last).unwrap().values().sum::<usize>();
    println!("cut_cost: {}", cut_cost);
    // println!("graph before merging: {:?}", graph);
    // we want the group just before the nodes are merged
    let partition = consumed_nodes.get(last).unwrap().clone();
    merge_vertices(graph, consumed_nodes, second_last, last);
    // println!("graph after merging: {:?}", graph);
    (cut_cost, partition)
}

fn get_most_tightly_connected_vertex<'a>(
    graph: &mut BTreeMap<&'a str, BTreeMap<&'a str, usize>>,
    a_set: &BTreeSet<&'a str>,
) -> &'a str {
    let mut max_weight = 0;
    let mut max_vertex = "";
    for (&node, neighbors) in graph.iter() {
        if a_set.contains(&node) {
            continue;
        }
        let mut w = 0;
        for (&n, &weight) in neighbors {
            if a_set.contains(&n) {
                w += weight;
            }
            if w > max_weight {
                max_weight = w;
                max_vertex = node;
            }
        }
    }
    max_vertex
}

fn merge_vertices<'a>(
    graph: &mut BTreeMap<&'a str, BTreeMap<&'a str, usize>>,
    consumed_nodes: &mut BTreeMap<&'a str, BTreeSet<&'a str>>,
    consumer: &'a str,
    consumed: &'a str,
) {
    // instead of creating new vertex
    // we simply remove the consumed and add all edges attached to it
    // to the consumer. also, combine the edge weight if the neighbor of consumed is
    // also a neighbor of the consumer

    // vertices we need to update
    let neighbors_of_consumed = graph.remove(consumed).unwrap();
    for (neighbor, neigh_weight) in neighbors_of_consumed {
        if neighbor == consumer {
            continue;
        }

        let consumer_map = graph.get_mut(consumer).unwrap();
        consumer_map
            .entry(neighbor)
            .and_modify(|w| *w += neigh_weight)
            .or_insert(neigh_weight);

        let neighbor_map = graph.get_mut(neighbor).unwrap();
        neighbor_map
            .entry(consumer)
            .and_modify(|w| *w += neigh_weight)
            .or_insert(neigh_weight);
        neighbor_map.remove(consumed);
    }
    let consumer_map = graph.get_mut(consumer).unwrap();
    consumer_map.remove(consumed);

    consumed_nodes.entry(consumer).or_default().insert(consumed);
    if let Some(mut consumed) = consumed_nodes.remove(consumed) {
        consumed_nodes
            .entry(consumer)
            .or_default()
            .append(&mut consumed);
    }
}

#[allow(dead_code)]
fn gen_test() -> BTreeMap<&'static str, BTreeMap<&'static str, usize>> {
    let mut graph: BTreeMap<&'static str, BTreeMap<&'static str, usize>> = BTreeMap::new();
    let g = [
        ("1", ("2", 2)),
        ("1", ("5", 3)),
        ("2", ("1", 2)),
        ("2", ("3", 3)),
        ("2", ("5", 2)),
        ("2", ("6", 2)),
        ("3", ("2", 3)),
        ("3", ("4", 4)),
        ("3", ("7", 2)),
        ("4", ("3", 4)),
        ("4", ("7", 2)),
        ("4", ("8", 2)),
        ("5", ("1", 3)),
        ("5", ("2", 2)),
        ("5", ("6", 3)),
        ("6", ("2", 2)),
        ("6", ("5", 3)),
        ("6", ("7", 1)),
        ("7", ("3", 2)),
        ("7", ("4", 2)),
        ("7", ("6", 1)),
        ("7", ("8", 3)),
        ("8", ("4", 2)),
        ("8", ("7", 3)),
    ];
    for (src, (dst, weight)) in g {
        graph.entry(src).or_default().insert(dst, weight);
    }
    graph
}
