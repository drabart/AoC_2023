use std::collections::{HashMap, VecDeque};

fn dfs(graph: &HashMap<&str, Vec<usize>>, edges: &Vec<(&str, i32)>, seen: &mut HashMap<String, i32>, node: String, drain: &str, path: &mut VecDeque<usize>) -> bool {
    seen.insert(node.clone(), 1);

    if node == drain {
        return true;
    }

    for conn in graph.get(&*node).unwrap() {
        if edges[*conn].1 == 0 {
            continue
        }
        let u = edges[*conn].0;
        if seen.contains_key(u) {
            continue;
        }
        path.push_back(*conn);
        if dfs(graph, edges, seen, u.parse().unwrap(), drain, path) {
            return true;
        }
        path.push_back(*conn);
    }

    return false;
}

fn min_cut(graph: &HashMap<&str, Vec<usize>>, edges: &Vec<(&str, i32)>, src: &str, drain: &str) -> Option<Vec<(String, String)>>{
    // println!("{src} {drain}");
    let mut edges = edges.clone();

    for flow_units in 0..4 {
        let mut seen: HashMap<String, i32> = HashMap::new();
        let mut path = VecDeque::<usize>::new();

        let path_found = dfs(&graph, &edges, &mut seen, src.parse().unwrap(), drain, &mut path);

        if flow_units == 3 && path_found {
            // println!("More than 3 flow!");
            return None;
        }

        if !path_found {
            if flow_units != 3 {
                // println!("Not enough ({}) flow units", flow_units - 1);
                return None;
            }
            let mut removed = vec![];

            for v in graph.iter() {
                for edge in v.1 {
                    let u = edges[*edge].0;
                    if seen.contains_key(*v.0) && !seen.contains_key(u) {
                        removed.push(((*v.0).parse().unwrap(), u.parse().unwrap()));
                    }
                }
            }

            return Some(removed);
        }

        for edge in path {
            edges[edge].1 -= 1;
            edges[edge ^ 1].1 += 1;
        }

        /*
        for v in graph.iter() {
            print!("{}: ", *v.0);
            for conn in v.1 {
                let u = edges[*conn];
                print!("({}, {})", u.0, u.1);
            }
            println!();
        }

         */
    }

    // should never get here
    println!("Well it got here");
    return None;
}

fn part1() {
    let data = std::fs::read_to_string("res/d25.in").unwrap();
    let lines = data.split("\n").filter(|&x| x!="");

    let mut edges = vec![];
    let mut graph: HashMap<&str, Vec<usize>> = HashMap::new();

    lines.for_each(|line| {
        let (label, connections) = line.split_once(": ").unwrap();
        if !graph.contains_key(label) {
            graph.insert(label, Vec::new());
        }
        connections.split(" ").for_each(|connection| {
            edges.push((connection, 1));
            edges.push((label, 1));

            graph.entry(label).and_modify(|conn| {
                conn.push(edges.len()-2);
            });
            if !graph.contains_key(connection) {
                graph.insert(connection, Vec::new());
            }
            graph.entry(connection).and_modify(|conn| {
                conn.push(edges.len()-1);
            });
        });
    });

    let mut removed = vec![];
    let mut it = 613;

    while removed.len() < 3 {
        it = (it * 31 + 13) % graph.len();
        let source = *graph.keys().nth(it).unwrap();
        it = (it * 31 + 13) % graph.len();
        let drain = *graph.keys().nth(it).unwrap();

        let ret = min_cut(&graph, &edges, source, drain);

        // return;

        if ret.is_none() {
            continue;
        }
        else {
            removed = ret.unwrap();
        }
    }

    let mut seen: HashMap<&str, i32> = HashMap::new();
    let mut queue = VecDeque::new();
    let s = *graph.keys().next().unwrap();
    queue.push_back(s);
    seen.insert(s, 1);

    while !queue.is_empty() {
        let v = queue.pop_back().unwrap();

        for conn in graph.get(v).unwrap() {
            if edges[*conn].1 == 0 {
                continue
            }
            let u = edges[*conn].0;
            if removed.contains(&(v.parse().unwrap(), u.parse().unwrap())) ||
                removed.contains(&(u.parse().unwrap(), v.parse().unwrap())) {
                continue;
            }
            if seen.contains_key(u) {
                continue;
            }
            seen.insert(u, 1);
            queue.push_back(u);
        }
    }

    dbg!(seen.len(), graph.len());

    let result = seen.len() * (graph.len() - seen.len());

    println!("Result of part 1: {result}");
}

fn main() {
    part1();
}
