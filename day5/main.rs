use std::cmp::max;
use std::cmp::min;
use std::collections::VecDeque;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input").unwrap();

    let mut is_reading_seed = true;

    /* part 1 */
    let mut seeds: Vec<i64> = vec![];
    let mut mapped_seeds: Vec<i64> = vec![];
    /* part 2 */
    let mut seeds_range: VecDeque<(i64, i64)> = VecDeque::new();
    let mut mapped_seeds_range: VecDeque<(i64, i64)> = VecDeque::new();

    for line in input.lines() {
        if line.is_empty() {
            if is_reading_seed {
                is_reading_seed = false;
                mapped_seeds = seeds.clone();
            } else {
                seeds = mapped_seeds.clone();
                seeds_range.extend(mapped_seeds_range);
                mapped_seeds_range = VecDeque::new();
            }
            continue;
        }
        if is_reading_seed {
            let (_, seeds_str) = line.split_once(':').unwrap();
            let seeds_vec = seeds_str
                .trim()
                .split(' ')
                .map(|x| x.parse::<i64>().unwrap());
            /* part 1 */
            seeds.extend(seeds_vec);
            /* part 2 */
            seeds_range.extend(seeds.chunks(2).map(|x| (x[0], x[1])));
        } else {
            if line.find(':').is_some() {
                continue;
            }
            let (dst, src, len) = parse_line(line);
            // println!("update {} - {} to {} - {}", src, src + len - 1, dst, dst + len - 1);
            /* part 1 */
            for (idx, seed) in seeds.iter().enumerate() {
                if *seed >= src && *seed <= src + len {
                    mapped_seeds[idx] = *seed - src + dst;
                }
            }
            /* part 2 */
            let num_seeds = seeds_range.len();
            let mut idx = 0;
            while let Some((seed, range)) = seeds_range.pop_front() {
                let seed_start = seed;
                let seed_end = seed + range - 1;

                let in_range_start = max(seed_start, src);
                let in_range_end = min(seed_end, src + len - 1);

                if in_range_start <= in_range_end {
                    if seed_start < in_range_start {
                        seeds_range.push_back((seed_start, in_range_start - seed_start));
                    }
                    if seed_end > in_range_end {
                        seeds_range.push_back((in_range_end + 1, seed_end - in_range_end));
                    }
                    mapped_seeds_range.push_back((
                        in_range_start + (dst - src),
                        in_range_end - in_range_start + 1,
                    ));
                } else {
                    seeds_range.push_back((seed, range));
                }
                idx += 1;
                if idx == num_seeds {
                    break;
                }
            }
            // println!("{:?}", seeds_range);
            // println!("{:?}", mapped_seeds_range);
        }
    }
    seeds = mapped_seeds;
    seeds_range.extend(mapped_seeds_range);
    // println!("{:?}", seeds_range);
    println!("{}", seeds.iter().min().unwrap());
    println!("{}", seeds_range.iter().min().unwrap().0);
}

fn parse_line(line: &str) -> (i64, i64, i64) {
    let mut data = line.trim().split(' ').map(|x| x.parse::<i64>().unwrap());
    let dst = data.next().unwrap();
    let src = data.next().unwrap();
    let len = data.next().unwrap();
    (dst, src, len)
}
