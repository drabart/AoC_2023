use std::collections::HashMap;

fn main() {
    let data = std::fs::read_to_string("res/d8.in").unwrap();
    let (instruction, map) = data.split_once("\n\n").unwrap();
    // dbg!(instruction);
    let maps: HashMap<&str, (&str, &str)> = map
        .split("\n")
        .filter(|x| !x.is_empty())
        // .map(|x| {dbg!(x);x})
        .map(|x| x.split_once(" = ").unwrap())
        .map(|(x,  y)| (x,  y
            .strip_prefix("(").unwrap()
            .strip_suffix(")").unwrap()
            .split_once(", ").unwrap()))
        .collect();

    let mut position: Vec<&str> = maps
        .keys()
        .filter(|x| x.chars().nth_back(0).unwrap() == 'A')
        .map(|x| *x)
        .collect();
    let mut w: Vec<(i64, i64)> = Vec::new();
    position.iter().for_each(|_x| w.push((0, 0)));

    let pos_aaa = position.iter().position(|&x| x == "AAA").unwrap();
    // dbg!(&position);
    let mut steps = 0;
    let mut done = 0;
    while position
        .iter()
        .filter(|x| x.chars().nth_back(0).unwrap() != 'Z')
        .count() != 0 {
        let direction = instruction.chars().nth(steps % instruction.len()).unwrap();
        position = position
            .iter()
            .map(|x| if direction == 'L' {maps.get(x).unwrap().0} else {maps.get(x).unwrap().1})
            .collect();
        steps += 1;
        for (i, x) in position.iter().enumerate() {
            if x.chars().nth_back(0).unwrap() != 'Z' {continue;}
            if w[i].0 == 0 {w[i].0 = steps as i64; done += 1;}
            else if w[i].1 == 0 {w[i].1 = steps as i64; done += 1;}
        }
        if done == position.len() * 2 {
            break;
        }
    }

    // dbg!(&position);

    let result_part_1 = w[pos_aaa].0;
    println!("Result of part 1: {result_part_1}");

    // dbg!(&w);
    let mut steps = w[0].0;
    loop {
        let mut done = true;
        for x in w.iter() {
            if (steps - x.0) % x.1 != 0 {done = false; break;}
        }
        if done {break;}
        steps += w[0].1;
    }
    println!("Result of part 2: {steps}");
}
