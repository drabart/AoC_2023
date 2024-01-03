use std::fs;

fn part1(lines: &Vec<&str>) {
    let mut sum = 0;

    for i in 0..lines.len() {
        let line = *lines.get(i).unwrap();
        let (_card_id, rest) = line.split_once(": ").unwrap();
        let (winning, chosen) = rest.split_once("|").unwrap();
        let winning: Vec<&str> = winning.split_whitespace().collect();
        let winning: Vec<i32> = winning
            .into_iter()
            .map(|s| s.trim())
            .map(|s| s.parse().unwrap())
            .collect();
        let chosen: Vec<&str> = chosen.split_whitespace().collect();
        let chosen: Vec<i32> = chosen
            .into_iter()
            .map(|s| s.trim())
            .map(|s| s.parse().unwrap())
            .collect();

        let mut hits = 0;
        for number in winning {
            if chosen.contains(&number) {
                hits += 1;
            }
        }

        if hits > 0 {
            sum += 1 << (hits - 1);
        }
    }

    println!("Result of part 1: {sum}");
}

fn part2(lines: &Vec<&str>) {
    let mut cards: Vec<i32> = Vec::new();
    for _ in 0..lines.len() {cards.push(1);}

    for i in 0..lines.len() {
        let line = *lines.get(i).unwrap();
        let (_card_id, rest) = line.split_once(": ").unwrap();
        let (winning, chosen) = rest.split_once("|").unwrap();
        let winning: Vec<&str> = winning.split_whitespace().collect();
        let winning: Vec<i32> = winning
            .into_iter()
            .map(|s| s.trim())
            .map(|s| s.parse().unwrap())
            .collect();
        let chosen: Vec<&str> = chosen.split_whitespace().collect();
        let chosen: Vec<i32> = chosen
            .into_iter()
            .map(|s| s.trim())
            .map(|s| s.parse().unwrap())
            .collect();

        let mut hits = 0;
        for number in winning {
            if chosen.contains(&number) {
                hits += 1;
            }
        }

        for x in i+1..i+1+hits {
            cards[x] += cards[i];
        }
    }
    let sum = cards.into_iter().sum::<i32>();

    println!("Result of part 2: {sum}");
}


fn main() {
    let data = fs::read_to_string("res/d4.in").unwrap();
    let lines: Vec<&str> = data.split("\n").filter(|&x| x != "").collect();

    part1(&lines);
    part2(&lines);
}
