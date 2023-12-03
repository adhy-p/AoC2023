use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let lines = read_lines("input");

    let mut total = 0;
    for line in &lines {
        total += parse_part1(&line);
    }
    println!("part1: {total}");

    total = 0;
    let map = part2_init();
    for line in lines {
        total += parse_part2(&line, &map);
    }
    println!("part2: {total}");
}

fn parse_part1(line: &String) -> u32 {
    let v: Vec<&str> = line.matches(char::is_numeric).collect();
    let first: &str = v.first().unwrap();
    let first: u32 = first.parse().unwrap();
    let last: &str = v.last().unwrap();
    let last: u32 = last.parse().unwrap();
    first * 10 + last
}

fn part2_init() -> HashMap<String, u32> {
    let mut m = HashMap::new();
    m.insert(String::from("zero"), 0);
    m.insert(String::from("one"), 1);
    m.insert(String::from("two"), 2);
    m.insert(String::from("three"), 3);
    m.insert(String::from("four"), 4);
    m.insert(String::from("five"), 5);
    m.insert(String::from("six"), 6);
    m.insert(String::from("seven"), 7);
    m.insert(String::from("eight"), 8);
    m.insert(String::from("nine"), 9);
    m
}

fn parse_part2(line: &String, map: &HashMap<String, u32>) -> u32 {
    let mut first = 0;
    let mut last = 0;
    let mut first_idx = line.len();
    let mut last_idx = 0;

    for (idx, c) in line.chars().enumerate() {
        if let Some(num) = c.to_digit(10) {
            if idx < first_idx {
                first = num;
                first_idx = idx;
            }
            last = num;
            last_idx = idx;
        }
    }

    for (key, num) in map.iter() {
        if let Some(idx) = line.find(key) {
            if idx < first_idx {
                first = *num;
                first_idx = idx;
            }
        }
        if let Some(idx) = line.rfind(key) {
            if idx > last_idx {
                last = *num;
                last_idx = idx;
            }
            
        }
    }
    // println!("{}: {}", line, first * 10 + last);
    first * 10 + last
}


fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}
