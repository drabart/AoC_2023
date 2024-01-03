use std::fs;
use std::iter::zip;

fn parse_data_2(raw_result: &Vec<u64>) -> Vec<u64> {
    let mut result: u64 = 0;
    for x in raw_result {
        result = result * 10u64.pow((*x).ilog10() + 1) + x;
    }
    return Vec::from([result]);
}

fn parse_data_1(entry: &str) -> Vec<u64> {
    let raw_result: Vec<u64> = entry
        .split(":")
        .nth(1)
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.trim().parse::<u64>().unwrap())
        .collect();

    return raw_result;
}

fn main() {
    let data = fs::read_to_string("res/d6.in").unwrap();
    let (time, distance) = data.split_once("\n").unwrap();
    let time = parse_data_1(time);
    let distance = parse_data_1(distance);
    // dbg!(time.clone(), distance.clone());
    let mut result = 1;
    for (t, d) in zip(time.clone(), distance.clone()) {
        let mut counter = 0;
        for i in 0..=t {
            if i * (t - i) > d {
                counter += 1;
            }
        }
        result *= counter;
    }
    println!("Result of part 1: {result}");
    let time = parse_data_2(&time);
    let distance = parse_data_2(&distance);
    let mut result = 1;
    for (t, d) in zip(time, distance) {
        let mut counter = 0;
        for i in 0..=t {
            if i * (t - i) > d {
                counter += 1;
            }
        }
        result *= counter;
    }
    println!("Result of part 2: {result}");
}
