use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input").unwrap();
    let mut input = input.lines();

    let (_, time) = input.next().unwrap().split_once(':').unwrap();
    /* part 1 */
    let time_vec: Vec<u64> = time
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    /* part 2 */
    let mut time = time.to_string();
    time.retain(|c| !c.is_whitespace());
    let one_time = time.parse::<u64>().unwrap();

    let (_, dist) = input.next().unwrap().split_once(':').unwrap();
    /* part 1 */
    let dist_vec: Vec<u64> = dist
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    /* part 2 */
    let mut dist = dist.to_string();
    dist.retain(|c| !c.is_whitespace());
    let one_dist = dist.parse::<u64>().unwrap();

    // println!("{:?}", time);
    // println!("{:?}", dist);

    let mut winning_ways = 1;
    for (idx, t) in time_vec.iter().enumerate() {
        let mut num_ways = 0;
        for time_wait in 0..=*t {
            let speed = time_wait;
            let remain_time = t - time_wait;
            if speed * remain_time > dist_vec[idx] {
                num_ways += 1;
            }
        }
        winning_ways *= num_ways;
    }

    let mut num_ways = 0;
    for time_wait in 0..=one_time {
        let speed = time_wait;
        let remain_time = one_time - time_wait;
        if speed * remain_time > one_dist {
            num_ways += 1;
        }
    }
    println!("{}", winning_ways);
    println!("{}", num_ways);
}
