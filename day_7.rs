use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use std::fs;

fn map_cards_1(s: &str) -> String{
    let k = (*s).as_bytes()
        .into_iter()
        .map(|x| {
            return if *x == b'A' {
                b'E'
            } else if *x == b'K' {
                b'D'
            } else if *x == b'Q' {
                b'C'
            } else if *x == b'J' {
                b'B'
            } else if *x == b'T' {
                b'A'
            } else {
                *x
            }
        })
        .map(|x| x as char)
        .collect::<Vec<char>>();
    let mut s = String::new();
    for c in k {
        s.push(c);
    }
    return s;
}

fn map_cards_2(s: &str) -> String{
    let k = (*s).as_bytes()
        .into_iter()
        .map(|x| {
            return if *x == b'A' {
                b'E'
            } else if *x == b'K' {
                b'D'
            } else if *x == b'Q' {
                b'C'
            } else if *x == b'J' {
                b'1'
            } else if *x == b'T' {
                b'A'
            } else {
                *x
            }
        })
        .map(|x| x as char)
        .collect::<Vec<char>>();
    let mut s = String::new();
    for c in k {
        s.push(c);
    }
    return s;
}

fn compare_hands_1(a: &(&str, i32), b: &(&str, i32)) -> Ordering {
    let mut na = [0;100];
    let mut nb = [0;100];
    (*a).0.as_bytes().into_iter().for_each(|x| na[*x as usize] += 1);
    (*b).0.as_bytes().into_iter().for_each(|x| nb[*x as usize] += 1);
    na.sort_by(|x,y| y.cmp(x));
    nb.sort_by(|x,y| y.cmp(x));
    return if na[0] > nb[0] {
        Greater
    } else if na[0] < nb[0] {
        Less
    } else if na[1] > nb[1] {
        Greater
    } else if na[1] < nb[1] {
        Less
    } else {
        let ap = map_cards_1((*a).0);
        let bp = map_cards_1((*b).0);
        ap.cmp(&bp)
    }
}

fn compare_hands_2(a: &(&str, i32), b: &(&str, i32)) -> Ordering {
    let mut na = [0;100];
    let mut nb = [0;100];
    (*a).0.as_bytes().into_iter().for_each(|x| na[*x as usize] += 1);
    (*b).0.as_bytes().into_iter().for_each(|x| nb[*x as usize] += 1);
    let ja = na[b'J' as usize];
    let jb = nb[b'J' as usize];
    na[b'J' as usize] = 0;
    nb[b'J' as usize] = 0;
    na.sort_by(|x,y| y.cmp(x));
    nb.sort_by(|x,y| y.cmp(x));
    na[0] += ja;
    nb[0] += jb;
    return if na[0] > nb[0] {
        Greater
    } else if na[0] < nb[0] {
        Less
    } else if na[1] > nb[1] {
        Greater
    } else if na[1] < nb[1] {
        Less
    } else {
        let ap = map_cards_2((*a).0);
        let bp = map_cards_2((*b).0);
        ap.cmp(&bp)
    }
}

fn part_1(data: String) {
    let data = data.split("\n");
    let mut hands: Vec<(&str, i32)> = Vec::new();

    for line in data.filter(|x| *x != "") {
        let (cards, bid) = line.split_once(" ").unwrap();
        let bid: i32 = bid.trim().parse().unwrap();
        hands.push((cards, bid));
    }

    hands.sort_by(compare_hands_1);
    // dbg!(&hands);
    let res = hands
        .iter()
        .map(|x| (*x).1)
        .enumerate()
        .map(|x| {x})
        .map(|x| x.1 * (x.0 as i32 + 1))
        .sum::<i32>();
    dbg!(res);
}

fn part_2(data: String) {
    let data = data.split("\n");
    let mut hands: Vec<(&str, i32)> = Vec::new();

    for line in data.filter(|x| *x != "") {
        let (cards, bid) = line.split_once(" ").unwrap();
        let bid: i32 = bid.trim().parse().unwrap();
        hands.push((cards, bid));
    }

    hands.sort_by(compare_hands_2);
    // dbg!(&hands);
    let res = hands
        .iter()
        .map(|x| (*x).1)
        .enumerate()
        .map(|x| {x})
        .map(|x| x.1 * (x.0 as i32 + 1))
        .sum::<i32>();
    dbg!(res);
}

fn main() {
    let data = fs::read_to_string("res/d7.in").unwrap();

    part_1(data.clone());
    part_2(data);
}
