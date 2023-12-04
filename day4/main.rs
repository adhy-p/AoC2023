use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_to_string("input")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    /* part 1 */
    let mut total_points = 0;
    /* part 2 */
    let mut num_copies = vec![1; lines.len()];

    for (idx, l) in lines.iter().enumerate() {
        let (_, cards) = l.split_once(':').unwrap();
        let (winning, owned) = cards.split_once('|').unwrap();
        let winning: HashSet<u32> = winning
            .trim()
            .split(' ')
            .filter(|x| (!x.is_empty() && *x != " "))
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        /* part 1 */
        let matches = owned
            .trim()
            .split(' ')
            .filter(|x| (!x.is_empty() && *x != " "))
            .map(|s| s.parse::<u32>().unwrap())
            .filter(|x| winning.contains(x))
            .count();
        if matches > 0 {
            total_points += 1 << (matches - 1);
        }
        /* part 2 */
        for i in idx + 1..=idx + matches {
            if i >= num_copies.len() {
                break;
            }
            num_copies[i] += num_copies[idx];
        }
    }
    println!("{total_points}");
    println!("{}", num_copies.iter().sum::<i32>());
}
