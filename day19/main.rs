use std::cmp::{max, min};
use std::collections::VecDeque;
use std::{collections::HashMap, time::Instant};

type Rating = HashMap<char, u64>;

#[derive(Clone, Copy, Debug)]
enum Dest<'a> {
    Accept,
    Reject,
    OtherRule(&'a str),
}

impl<'a> From<&'a str> for Dest<'a> {
    fn from(s: &'a str) -> Self {
        match s {
            "A" => Dest::Accept,
            "R" => Dest::Reject,
            _ => Dest::OtherRule(s),
        }
    }
}

#[derive(Debug)]
enum Comparator {
    LessThan,
    GreaterThan,
}

impl From<char> for Comparator {
    fn from(c: char) -> Self {
        match c {
            '<' => Comparator::LessThan,
            '>' => Comparator::GreaterThan,
            _ => panic!("invalid comparator"),
        }
    }
}

#[derive(Debug)]
struct Cond {
    key: char,
    comp: Comparator,
    value: u64,
}

impl From<&str> for Cond {
    fn from(s: &str) -> Self {
        let mut it = s.chars();
        let key = it.next().unwrap();
        let comp = Comparator::from(it.next().unwrap());
        let value: u64 = it
            .map(|c| c.to_digit(10).unwrap())
            .fold(0, |acc, val| acc * 10 + val)
            .into();
        Cond { key, comp, value }
    }
}

impl Cond {
    fn matches_with(&self, r: &Rating) -> bool {
        match self.comp {
            Comparator::LessThan => r[&self.key] < self.value,
            Comparator::GreaterThan => r[&self.key] > self.value,
        }
    }
}

#[derive(Debug)]
struct Rule<'a> {
    cond: Option<Cond>,
    dest: Dest<'a>,
}

impl<'a> From<&'a str> for Rule<'a> {
    fn from(s: &'a str) -> Self {
        if let Some((cond, dest)) = s.split_once(':') {
            Rule {
                cond: Some(Cond::from(cond)),
                dest: Dest::from(dest),
            }
        } else {
            Rule {
                cond: None,
                dest: Dest::from(s),
            }
        }
    }
}

impl<'a> Rule<'a> {
    fn matches_with(&self, r: &Rating) -> bool {
        match &self.cond {
            None => true,
            Some(c) => c.matches_with(r),
        }
    }
}

fn main() {
    let start = Instant::now();

    let input = include_str!("./input");
    let (workflows, ratings) = input.split_once("\n\n").unwrap();

    let workflows = parse_workflows(workflows);
    let ratings: Vec<Rating> = parse_ratings(ratings);

    // println!("{workflows:?}");
    // println!("{ratings:?}");

    let total = simulate_part1(&workflows, &ratings);
    println!("{}", total);
    let total = simulate_part2(&workflows);
    println!("{}", total);
    println!("{:?}", start.elapsed());
}

fn do_workflow<'a>(rating: &Rating, rules_vec: &Vec<Rule<'a>>) -> Dest<'a> {
    for rule in rules_vec {
        if rule.matches_with(rating) {
            return rule.dest;
        }
    }
    unreachable!();
}

fn simulate_part1(workflows: &HashMap<&str, Vec<Rule>>, ratings: &Vec<Rating>) -> u64 {
    let mut total = 0;
    for rating in ratings {
        let mut workflow_name = Dest::OtherRule("in");
        loop {
            match workflow_name {
                Dest::Accept => {
                    total += rating.iter().map(|(_, v)| *v).sum::<u64>();
                    break;
                }
                Dest::Reject => {
                    break;
                }
                Dest::OtherRule(name) => {
                    let rules_vec = workflows.get(name).expect("rule should exist");
                    workflow_name = do_workflow(rating, rules_vec);
                }
            }
        }
    }
    total
}

#[derive(Clone, Copy, Debug)]
struct Range(u64, u64);

fn simulate_part2(workflows: &HashMap<&str, Vec<Rule>>) -> u64 {
    let mut total = 0;
    type State = [Range; 4]; // x, m, a, s
    let state: State = [Range(1, 4000); 4];

    let mut to_process: VecDeque<(Dest, State)> = VecDeque::new();
    to_process.push_back((Dest::OtherRule("in"), state));

    while let Some((dest, state)) = to_process.pop_front() {
        match dest {
            Dest::Accept => {
                total += state.into_iter().fold(1, |acc, x| acc * (1 + x.1 - x.0));
            }
            Dest::Reject => {}
            Dest::OtherRule(name) => {
                // TODO: refactor to functions :D
                let mut curr_state = state;
                let rules_vec = workflows.get(name).expect("rule should exist");
                for Rule { cond: c, dest: d } in rules_vec {
                    let state_copy = curr_state;
                    match c {
                        Some(cond) => {
                            let Cond { key, comp, value } = cond;
                            let (range_true, range_false) = match *comp {
                                Comparator::GreaterThan => ((*value + 1, u64::MAX), (0, *value)),
                                Comparator::LessThan => ((0, *value - 1), (*value, u64::MAX)),
                            };
                            let idx: usize = match *key {
                                'x' => 0,
                                'm' => 1,
                                'a' => 2,
                                's' => 3,
                                _ => panic!("invalid key"),
                            };
                            // pred true -> push to process
                            curr_state[idx] = Range(
                                max(state_copy[idx].0, range_true.0),
                                min(state_copy[idx].1, range_true.1),
                            );
                            to_process.push_back((*d, curr_state));
                            // pred false -> update state
                            curr_state[idx] = Range(
                                max(state_copy[idx].0, range_false.0),
                                min(state_copy[idx].1, range_false.1),
                            );
                        }
                        None => {
                            to_process.push_back((*d, curr_state));
                        }
                    }
                }
            }
        }
    }
    total
}

fn parse_ratings(ratings: &str) -> Vec<Rating> {
    let mut v = Vec::new();
    for line in ratings.lines() {
        let line = &line[1..line.len() - 1];
        let m = line
            .split(',')
            .map(|item| {
                let (key, val) = item.split_once('=').unwrap();
                let key = key.chars().next().unwrap();
                let val = val.parse::<u64>().expect("should be an integer");
                (key, val)
            })
            .collect::<HashMap<char, u64>>();
        v.push(m);
    }
    v
}

fn parse_workflows(workflows: &str) -> HashMap<&str, Vec<Rule>> {
    let mut m = HashMap::new();
    for line in workflows.lines() {
        let (name, rules) = line.split_once('{').unwrap();
        let rules = rules.strip_suffix('}').unwrap();
        let rules = rules.split(',').map(Rule::from).collect::<Vec<_>>();
        m.insert(name, rules);
    }
    m
}
