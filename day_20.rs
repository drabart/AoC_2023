use std::collections::{HashMap, VecDeque};

struct FlipFlop {
    state: bool,
    connections: Vec<String>
}

struct Conjunction {
    states: HashMap<String, bool>,
    connections: Vec<String>
}

fn create_flip_flop(line: &str) -> (String, FlipFlop) {
    let (label, connections) = line
        .strip_prefix("%").unwrap()
        .split_once(" -> ").unwrap();
    let connections = connections
        .split(", ")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let flip_flop = FlipFlop {
        state: false,
        connections
    };
    (label.to_string(), flip_flop)
}

fn create_conjunction(line: &str) -> (String, Conjunction) {
    let (label, connections) = line
        .strip_prefix("&").unwrap()
        .split_once(" -> ").unwrap();
    let connections = connections
        .split(", ")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let conjunction = Conjunction {
        states: HashMap::new(),
        connections
    };
    (label.to_string(), conjunction)
}
fn part1(lines: &Vec<&str>) {
    let mut broadcaster: Vec<String> = Vec::new();
    let mut flip_flops: HashMap<String, FlipFlop> = HashMap::new();
    let mut conjunctions: HashMap<String, Conjunction> = HashMap::new();

    lines.iter().for_each(|&line| {
        if line.starts_with("%") {
            let new_flip_flop = create_flip_flop(line);
            flip_flops.insert(new_flip_flop.0, new_flip_flop.1);
        } else if line.starts_with("&") {
            let new_conjunction = create_conjunction(line);
            conjunctions.insert(new_conjunction.0, new_conjunction.1);
        } else {
            let connections = line
                .strip_prefix("broadcaster -> ").unwrap()
                .split(", ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            broadcaster = connections;
        }
    });

    broadcaster.iter().for_each(|connection| {
        if conjunctions.contains_key(connection) {
            let conj = conjunctions.get_mut(connection).unwrap();
            conj.states.insert("broadcaster".parse().unwrap(), false);
        }
    });
    flip_flops.iter().for_each(|(label, flip_flop)| {
        (*flip_flop).connections.iter().for_each(|connection| {
            if conjunctions.contains_key(connection) {
                let conj = conjunctions.get_mut(connection).unwrap();
                conj.states.insert((*label).clone(), false);
            }
        });
    });

    let mut updates = Vec::new();
    for (key, conjunction) in conjunctions.iter() {
        conjunction.connections.iter().for_each(|connection| {
            if conjunctions.contains_key(connection) {
                updates.push((connection.clone(), key.clone()));
            }
        });
    }
    for (connection, key) in updates {
        conjunctions.entry(connection).and_modify(|x| {
            x.states.insert(key, false);
        });
    }

    let mut currently_handled: VecDeque<(String, bool, String)> = VecDeque::new();
    let mut low = 0;
    let mut high = 0;

    for _ in 0..1000 {
        currently_handled.push_back(("broadcaster".parse().unwrap(), false, "".parse().unwrap()));

        while !currently_handled.is_empty() {
            let current_label = currently_handled.pop_front().unwrap();
            // println!("{current_label:?}");
            if current_label.1 {
                high += 1;
            } else {
                low += 1;
            }
            let new_connections: VecDeque<(String, bool, String)>;
            if current_label.0 == "broadcaster" {
                new_connections = broadcaster.iter()
                    .map(|connection| ((*connection).clone(), false, current_label.0.clone()))
                    .collect();
            } else if flip_flops.contains_key(&current_label.0) {
                if current_label.1 {
                    continue;
                }
                let flip_flop = flip_flops.get_mut(&current_label.0).unwrap();
                flip_flop.state = !flip_flop.state;
                new_connections = flip_flop.connections.iter()
                    .map(|connection| ((*connection).clone(), flip_flop.state, current_label.0.clone()))
                    .collect();
            } else if conjunctions.contains_key(&current_label.0){
                let conjunction = conjunctions.get_mut(&current_label.0).unwrap();
                *conjunction.states.get_mut(&current_label.2).unwrap() = current_label.1;
                let send_state = !conjunction.states.iter().all(|state| *state.1);
                new_connections = conjunction.connections.iter()
                    .map(|connection| ((*connection).clone(), send_state, current_label.0.clone()))
                    .collect();
            } else {
                new_connections = VecDeque::new();
            }
            currently_handled.extend(new_connections);
        }
    }

    println!("Result for  part 1: {}", low * high);
}

fn part2(lines: &Vec<&str>) {
    let mut broadcaster: Vec<String> = Vec::new();
    let mut flip_flops: HashMap<String, FlipFlop> = HashMap::new();
    let mut conjunctions: HashMap<String, Conjunction> = HashMap::new();

    lines.iter().for_each(|&line| {
        if line.starts_with("%") {
            let new_flip_flop = create_flip_flop(line);
            flip_flops.insert(new_flip_flop.0, new_flip_flop.1);
        } else if line.starts_with("&") {
            let new_conjunction = create_conjunction(line);
            conjunctions.insert(new_conjunction.0, new_conjunction.1);
        } else {
            let connections = line
                .strip_prefix("broadcaster -> ").unwrap()
                .split(", ")
                .map(|x| x.to_string())
                .collect::<Vec<String>>();
            broadcaster = connections;
        }
    });

    broadcaster.iter().for_each(|connection| {
        if conjunctions.contains_key(connection) {
            let conj = conjunctions.get_mut(connection).unwrap();
            conj.states.insert("broadcaster".parse().unwrap(), false);
        }
    });
    flip_flops.iter().for_each(|(label, flip_flop)| {
        (*flip_flop).connections.iter().for_each(|connection| {
            if conjunctions.contains_key(connection) {
                let conj = conjunctions.get_mut(connection).unwrap();
                conj.states.insert((*label).clone(), false);
            }
        });
    });

    let mut updates = Vec::new();
    for (key, conjunction) in conjunctions.iter() {
        conjunction.connections.iter().for_each(|connection| {
            if conjunctions.contains_key(connection) {
                updates.push((connection.clone(), key.clone()));
            }
        });
    }
    for (connection, key) in updates {
        conjunctions.entry(connection).and_modify(|x| {
            x.states.insert(key, false);
        });
    }

    let last_nodes = vec!["mm", "ff", "fk", "lh"];
    let mut lnv = vec![0i64;4];
    let mut currently_handled: VecDeque<(String, bool, String)> = VecDeque::new();
    let mut button_pressed = 0;
    'outer: loop {
        currently_handled.push_back(("broadcaster".parse().unwrap(), false, "".parse().unwrap()));
        button_pressed += 1;

        while !currently_handled.is_empty() {
            let current_label = currently_handled.pop_front().unwrap();
            // println!("{current_label:?}");
            last_nodes.iter().zip(0..).for_each(|x| {
                if !current_label.1 && *x.0 == current_label.0 && lnv[x.1] == 0{
                    lnv[x.1] = button_pressed;
                }
            });
            if lnv.iter().all(|&x| x > 0) {
                break 'outer;
            }
            let new_connections: VecDeque<(String, bool, String)>;
            if current_label.0 == "broadcaster" {
                new_connections = broadcaster.iter()
                    .map(|connection| ((*connection).clone(), false, current_label.0.clone()))
                    .collect();
            } else if flip_flops.contains_key(&current_label.0) {
                if current_label.1 {
                    continue;
                }
                let flip_flop = flip_flops.get_mut(&current_label.0).unwrap();
                flip_flop.state = !flip_flop.state;
                new_connections = flip_flop.connections.iter()
                    .map(|connection| ((*connection).clone(), flip_flop.state, current_label.0.clone()))
                    .collect();
            } else if conjunctions.contains_key(&current_label.0){
                let conjunction = conjunctions.get_mut(&current_label.0).unwrap();
                *conjunction.states.get_mut(&current_label.2).unwrap() = current_label.1;
                let send_state = !conjunction.states.iter().all(|state| *state.1);
                new_connections = conjunction.connections.iter()
                    .map(|connection| ((*connection).clone(), send_state, current_label.0.clone()))
                    .collect();
            } else {
                new_connections = VecDeque::new();
            }
            currently_handled.extend(new_connections);
        }
    }

    let result = lnv.iter().fold(1, |a, &x| num::integer::lcm(a, x));
    // dbg!(lnv.clone());

    println!("Result for  part 2: {result}");
}

fn main() {
    let data = std::fs::read_to_string("res/d20.in").unwrap();
    let lines = data.split("\n").filter(|&x| x != "").collect::<Vec<&str>>();

    part1(&lines);
    part2(&lines);
}
