use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    time::Instant,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

#[derive(Debug)]
enum Module {
    FlipFlop {
        state: bool,
        outputs: Vec<String>,
    },
    Conjuction {
        inputs: BTreeMap<String, Pulse>,
        outputs: Vec<String>,
    },
    Broadcaster {
        outputs: Vec<String>,
    },
}

impl Module {
    fn receive_pulse(&mut self, src: String, pulse: Pulse) -> Vec<(Pulse, String)> {
        match self {
            Self::FlipFlop { state, outputs } => match pulse {
                Pulse::High => Vec::new(),
                Pulse::Low => {
                    let pending_pulse = match *state {
                        false => Pulse::High,
                        true => Pulse::Low,
                    };
                    *state = !*state;
                    let mut next_pulses = Vec::new();
                    for dst in outputs {
                        next_pulses.push((pending_pulse, dst.clone()));
                    }
                    next_pulses
                }
            },
            Self::Conjuction { inputs, outputs } => {
                inputs.entry(src).and_modify(|p| *p = pulse);
                let pending_pulse = if inputs.iter().all(|(_, &v)| v == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                let mut next_pulses = Vec::new();
                for dst in outputs {
                    next_pulses.push((pending_pulse, dst.clone()));
                }
                next_pulses
            }
            Self::Broadcaster { outputs } => {
                let mut next_pulses = Vec::new();
                for dst in outputs {
                    next_pulses.push((pulse, dst.clone()));
                }
                next_pulses
            }
        }
    }

    fn parse(name: &str, outputs: &Vec<&str>) -> (String, Self) {
        let first_char = name.chars().next().unwrap();
        let outputs = outputs.clone().into_iter().map(String::from).collect();
        match first_char {
            '%' => (
                name[1..].to_string(),
                Module::FlipFlop {
                    state: false,
                    outputs,
                },
            ),
            '&' => (
                name[1..].to_string(),
                Module::Conjuction {
                    inputs: BTreeMap::new(),
                    outputs,
                },
            ),
            _ => {
                assert_eq!(name, "broadcaster");
                (name.to_string(), Module::Broadcaster { outputs })
            }
        }
    }

    fn serialize(&self) -> String {
        match self {
            Self::FlipFlop { state, .. } => format!("FF{:?}", *state),
            Self::Conjuction { inputs, .. } => format!("C{:?}", inputs),
            Self::Broadcaster { .. } => "Broadcaster".to_string(),
        }
    }
}
fn main() {
    let start = Instant::now();

    let input = include_str!("./input");
    let graph = parse_input(input);
    let mut graph = graph;

    // println!("{:?}", instr);
    // println!("{:?}", name_to_modtype);

    let total = simulate(&mut graph);
    println!("{}", total);
    println!("{:?}", start.elapsed());
}

fn gen_key(m: &BTreeMap<String, Module>) -> String {
    let mut s = String::new();
    for (k, v) in m {
        s.push_str(k);
        s.push_str(&v.serialize());
    }
    s
}

/**
 * rx receives pulse from vf (Conjunction)
 * vf receives pulses from [pk, mk, pm] (all being Conjunction)
 */
fn simulate(graph: &mut BTreeMap<String, Module>) -> usize {
    const NUM_ITER: u32 = 999_999_999;
    let mut n_low_pulses = 0;
    let mut n_hi_pulses = 0;

    // println!("{:?}", graph);
    let mut history: HashMap<String, u32> = HashMap::new();
    history.insert(gen_key(graph), 0);
    for i in 1..=NUM_ITER {
        // println!("=====");
        let mut to_process = VecDeque::new();
        to_process.push_back((Pulse::Low, "button".to_string(), "broadcaster".to_string()));
        while let Some((pulse, src, dest)) = to_process.pop_front() {
            // println!("{} sends {:?} to {}", src, pulse, dest);
            let module = graph.get_mut(&dest);
            match pulse {
                Pulse::Low => {
                    n_low_pulses += 1;
                }
                Pulse::High => {
                    n_hi_pulses += 1;
                }
            }
            if module.is_none() {
                // untyped module. does not forward pulse
                continue;
            }
            let module = module.unwrap();
            // println!("{} {}", n_low_pulses, n_hi_pulses);
            let next_pulses = module.receive_pulse(src, pulse);
            if (dest == *"pm" || dest == *"mk" || dest == *"pk" || dest == *"hf")
                && next_pulses.iter().next().unwrap().0 == Pulse::High
            {
                // println!("{}: {}", dest, i);
                // lcm {3881, 3889, 4013, 4021}
                // 243548140870057
            }
            for (pulse, next_dst) in next_pulses {
                to_process.push_back((pulse, dest.clone(), next_dst));
            }
        }
        history.insert(gen_key(graph), i);
    }
    println!("{} {}", n_low_pulses, n_hi_pulses);
    n_low_pulses * n_hi_pulses
}

fn parse_input(input: &str) -> BTreeMap<String, Module> {
    let mut graph: BTreeMap<String, Module> = BTreeMap::new();
    let mut incoming_edges: HashMap<&str, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let (incoming, outgoing) = line.split_once(" -> ").unwrap();
        let outgoing = outgoing.split(',').map(|s| s.trim()).collect::<Vec<_>>();
        let (name, mod_type) = Module::parse(incoming, &outgoing);
        graph.insert(name.clone(), mod_type);
        outgoing.iter().for_each(|&out| {
            (*incoming_edges.entry(out).or_default()).push(name.clone());
        });
    }

    /* initialize conjuction modules */
    for (k, v) in incoming_edges {
        let module = graph.get_mut(k);
        if module.is_none() {
            println!("should be 'output': {}", k);
            continue;
        }
        let module = module.unwrap();
        if let Module::Conjuction { inputs: _, outputs } = module {
            let mut m = BTreeMap::new();
            for incoming in &v {
                m.insert((*incoming).to_string(), Pulse::Low);
            }
            *module = Module::Conjuction {
                inputs: m,
                outputs: outputs.to_vec(),
            };
        }
    }
    graph
}
