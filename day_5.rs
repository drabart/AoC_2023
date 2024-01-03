use std::fs;
use std::cmp;

fn part1(lines: &Vec<&str>) {
    let mut seeds: Vec<i64> = lines[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();
    let mut converted: Vec<i64> = Vec::new();
    for line in lines.iter().filter(|l| **l != "" && **l != "\n").skip(1) {
        let first_char = *line.as_bytes().get(0).unwrap();
        if b'a' <= first_char && first_char <= b'z' {
            for seed in seeds {
                if seed != 0 {
                    converted.push(seed);
                }
            }
            seeds = converted;
            converted = Vec::new();
            continue;
        }
        let mapping: Vec<i64> = line
            .split_ascii_whitespace()
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();

        for i in 0..seeds.len() {
            let seed = seeds[i];

            if seed >= mapping[1] && seed < mapping[1] + mapping[2] {
                seeds[i] = 0;
                converted.push(seed + mapping[0] - mapping[1]);
            }
        }
    }
    for seed in seeds {
        if seed != 0 {
            converted.push(seed);
        }
    }
    seeds = converted;

    let minimum = seeds.iter().min().unwrap();
    println!("Result of part 1: {minimum}");
}

fn part2(lines: &Vec<&str>) {
    let seeds_a: Vec<i64> = lines[0]
        .split(": ")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();
    let mut seeds: Vec<(i64, i64)> = Vec::new();
    for i in (0..seeds_a.len()).step_by(2) {
        seeds.push((seeds_a[i], seeds_a[i + 1]));
    }
    let mut converted: Vec<(i64, i64)> = Vec::new();
    for line in lines.iter().filter(|l| **l != "" && **l != "\n").skip(1) {
        let first_char = *line.as_bytes().get(0).unwrap();
        if b'a' <= first_char && first_char <= b'z' {
            for seed in &seeds {
                if (*seed).1 != 0 {
                    converted.push(*seed);
                }
            }
            seeds = converted;
            converted = Vec::new();
            continue;
        }
        let mapping: Vec<i64> = line
            .split_ascii_whitespace()
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();

        for i in 0..seeds.len() {
            let seed = seeds[i];
            if seed.1 == 0 { continue; }
            let start = cmp::max(seed.0, mapping[1]);
            let end = cmp::min(seed.0 + seed.1, mapping[1] + mapping[2]);
            if end <= start { continue; }

            if seed.0 < mapping[1]  { seeds.push((seed.0, mapping[1] - seed.0)); }
            if seed.0 + seed.1 > mapping[1] + mapping[2] { seeds.push((mapping[1] + mapping[2], seed.0 + seed.1 - mapping[1] - mapping[2])); }
            seeds[i].1 = 0;
            let start = mapping[0] + start - mapping[1];
            let end = mapping[0] + end - mapping[1];
            converted.push((start, end - start));
        }
    }
    for seed in &seeds {
        if (*seed).1 != 0 {
            converted.push(*seed);
        }
    }
    seeds = converted;

    let minimum = seeds.iter().min().unwrap().0;
    println!("Result of part 2: {minimum}");
}

fn main() {
    let data = fs::read_to_string("res/d5.in").unwrap();
    let lines: Vec<&str> = data.split("\n").collect();

    part1(&lines);
    part2(&lines);
}
