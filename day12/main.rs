use std::fs::read_to_string;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = read_to_string("./input").unwrap();
    let mut total_ways_1 = 0;
    let mut total_ways_2 = 0;
    for line in input.lines() {
        let (map, arragements) = line.split_once(' ').unwrap();
        let map1: Vec<char> = map.chars().collect();
        let arragements1: Vec<usize> = arragements
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let max_group_size: usize = *arragements1.iter().max().unwrap();
        let mut cache: Vec<Vec<Vec<usize>>> =
            vec![
                vec![vec![usize::MAX; max_group_size + 1]; arragements1.len()];
                map1.len()
            ];
        total_ways_1 += find_ways(&mut cache, &map1, &arragements1, 0);

        let mut map2 = map1.clone();
        let mut arragements2 = arragements1.clone();
        for _ in 0..4 {
            map2.push('?');
            map2.extend(map1.clone());
            arragements2.extend(arragements1.clone());
        }
        let mut cache: Vec<Vec<Vec<usize>>> =
            vec![
                vec![vec![usize::MAX; max_group_size + 1]; arragements2.len()];
                map2.len()
            ];
        total_ways_2 += find_ways(&mut cache, &map2, &arragements2, 0);
    }
    println!("{}", total_ways_1);
    println!("{}", total_ways_2);
    println!("{:?}", start.elapsed());
}

fn find_ways(
    cache: &mut Vec<Vec<Vec<usize>>>,
    map: &[char],
    arragements: &[usize],
    curr_grp_size: usize,
) -> usize {
    // println!("find ways: {:?} {:?} {} {} {}", map, arragements, map_idx, arr_idx, curr_grp_size);
    if arragements.is_empty() {
        if map.is_empty() || map.iter().filter(|x| **x == '#').count() == 0 {
            // println!("find ways: {:?} {:?} {} {} {} returns 1", map, arragements, map_idx, arr_idx, curr_grp_size);
            return 1;
        }
        return 0;
    }
    if map.is_empty() {
        if arragements.len() == 1 && curr_grp_size == arragements[0] {
            // println!("find ways: {:?} {:?} {} {} {} returns 1", map, arragements, map_idx, arr_idx, curr_grp_size);
            return 1;
        }
        return 0;
    }

    if cache[map.len()-1][arragements.len()-1][curr_grp_size] != usize::MAX {
        return cache[map.len()-1][arragements.len()-1][curr_grp_size];
    }

    cache[map.len()-1][arragements.len()-1][curr_grp_size] = match map[0] {
        '#' => {
            if curr_grp_size + 1 > arragements[0] {
                return 0;
            }
            find_ways(cache, &map[1..], arragements, curr_grp_size + 1)
        }
        '.' => {
            if curr_grp_size != 0 {
                if curr_grp_size != arragements[0] {
                    return 0;
                }
                return find_ways(cache, &map[1..], &arragements[1..], 0);
            }
            find_ways(cache, &map[1..], arragements, curr_grp_size)
        }
        '?' => {
            let mut ways = 0;
            // case 1: #
            if curr_grp_size < arragements[0] {
                ways += find_ways(cache, &map[1..], arragements, curr_grp_size + 1);
            }
            // case 2: .
            if curr_grp_size != 0 {
                if curr_grp_size == arragements[0] {
                    ways += find_ways(cache, &map[1..], &arragements[1..], 0);
                }
            } else {
                ways += find_ways(cache, &map[1..], arragements, curr_grp_size)
            }
            ways
        }
        _ => panic!("invalid map input"),
    };
    cache[map.len()-1][arragements.len()-1][curr_grp_size]
}
