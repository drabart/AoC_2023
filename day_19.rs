use std::collections::HashMap;

fn part_2(rule_set: &HashMap<&str, Vec<(i64, i64, bool, &str)>>) {
    let mut variables = vec![vec![(1, 4000), (1, 4000), (1, 4000), (1, 4000)]];

    let mut sum = 0;
    let mut x = 0;
    while x < variables.len(){
        let mut v = variables[x].clone();
        x += 1;
        let mut label = "in";
        let mut accepted = false;
        loop {
            if label == "A" {
                accepted = true;
                break;
            }
            if label == "R" {
                break;
            }
            let current_rule = rule_set.get(label).unwrap();
            for r in current_rule {
                if r.0 == 4 {
                    label = r.3;
                    break;
                }
                if r.2 {
                    if v[r.0 as usize].1 <= r.1 {
                        continue;
                    }
                    if v[r.0 as usize].0 > r.1 {
                        label = r.3;
                        break;
                    }
                    let mut nv = v.clone();
                    nv[r.0 as usize].1 = r.1;
                    v[r.0 as usize].0 = r.1 + 1;
                    variables.push(nv);
                    label = r.3;
                    break;
                } else {
                    if v[r.0 as usize].0 >= r.1 {
                        continue;
                    }
                    if v[r.0 as usize].1 < r.1 {
                        label = r.3;
                        break;
                    }
                    let mut nv = v.clone();
                    nv[r.0 as usize].0 = r.1;
                    v[r.0 as usize].1 = r.1 - 1;
                    variables.push(nv);
                    label = r.3;
                    break;
                }
            }
        }
        if accepted {
            sum += (v[0].1 - v[0].0 + 1) * (v[1].1 - v[1].0 + 1) *
                (v[2].1 - v[2].0 + 1) * (v[3].1 - v[3].0 + 1);
        }
    };
    println!("Result of part 2: {sum}");
}

fn part_1(rule_set: &HashMap<&str, Vec<(i64, i64, bool, &str)>>, variables: &Vec<Vec<i64>>) {
    let mut variables = variables.iter().map(|v| {
            v.iter().map(|a| (*a, *a)).collect::<Vec<(i64, i64)>>()
        })
        .collect::<Vec<Vec<(i64, i64)>>>();

    let mut sum = 0;
    let mut x = 0;
    while x < variables.len(){
        let mut v = variables[x].clone();
        x += 1;
        let mut label = "in";
        let mut accepted = false;
        loop {
            if label == "A" {
                accepted = true;
                break;
            }
            if label == "R" {
                break;
            }
            let current_rule = rule_set.get(label).unwrap();
            for r in current_rule {
                if r.0 == 4 {
                    label = r.3;
                    break;
                }
                if r.2 {
                    if v[r.0 as usize].1 <= r.1 {
                        continue;
                    }
                    if v[r.0 as usize].0 > r.1 {
                        label = r.3;
                        break;
                    }
                    let mut nv = v.clone();
                    nv[r.0 as usize].1 = r.1;
                    v[r.0 as usize].0 = r.1 + 1;
                    variables.push(nv);
                    label = r.3;
                    break;
                } else {
                    if v[r.0 as usize].0 >= r.1 {
                        continue;
                    }
                    if v[r.0 as usize].1 < r.1 {
                        label = r.3;
                        break;
                    }
                    let mut nv = v.clone();
                    nv[r.0 as usize].0 = r.1;
                    v[r.0 as usize].1 = r.1 - 1;
                    variables.push(nv);
                    label = r.3;
                    break;
                }
            }
        }
        if accepted {
            sum += v.iter().fold(0, |acc, x| acc + (*x).0 );
        }
    };
    println!("Result of part 1: {sum}");
}

fn main() {
    let data = std::fs::read_to_string("res/d19.in").unwrap();
    let (rules, variables) = data.split_once("\n\n").unwrap();

    let variables = variables
        .split("\n")
        .filter(|&x| x != "")
        .map(|vars| {
            vars.strip_prefix("{").unwrap()
                .strip_suffix("}").unwrap()
                .split(",")
                .map(|var| var
                    .split_once("=").unwrap().1
                    .parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    // compare var, compare val, compare greater, result
    let mut rule_set: HashMap<&str, Vec<(i64,i64,bool,&str)>> = HashMap::new();

    rules.split("\n")
        .filter(|&x| x != "")
        .map(|x| {
            let (label, rules) = x.split_once("{").unwrap();
            let rules = rules
                .strip_suffix("}").unwrap()
                .split(",")
                .map(|rule| {
                    if rule.contains(":") {
                        let mut greater = false;
                        let (v,rest) = if rule.contains(">") {
                            greater = true;
                            rule.split_once(">").unwrap()
                        } else {
                            rule.split_once("<").unwrap()
                        };
                        let var = match v {
                            "x" => 0,
                            "m" => 1,
                            "a" => 2,
                            _ => 3
                        };
                        let (val, res) = rest.split_once(":").unwrap();
                        let val = val.parse().unwrap();

                        (var, val, greater, res)
                    } else {
                        (4, 0, false, rule)
                    }
                })
                .collect::<Vec<(i64,i64,bool,&str)>>();
            (label, rules)
        })
        .for_each(|x| {
            rule_set.insert(x.0, x.1);
        });

    part_1(&rule_set, &variables);
    part_2(&rule_set);
}
