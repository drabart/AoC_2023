use std::collections::HashSet;

fn n(board: &mut Vec<Vec<u8>>) {
    let n = board.len();
    for _times in 0..n {
        for row in 1..n {
            for column in 0..n {
                if board[row][column] != b'O' { continue; }
                if board[row - 1][column] != b'.' { continue; }
                else { board[row][column] = b'.'; board[row - 1][column] = b'O'; }
            }
        }
    }
}

fn s(board: &mut Vec<Vec<u8>>) {
    let n = board.len();
    for _times in 0..n {
        for row in (0..n-1).rev() {
            for column in 0..n {
                if board[row][column] != b'O' { continue; }
                if board[row + 1][column] != b'.' { continue; }
                else { board[row][column] = b'.'; board[row + 1][column] = b'O'; }
            }
        }
    }
}

fn w(board: &mut Vec<Vec<u8>>) {
    let n = board.len();
    for _times in 0..n {
        for column in 1..n {
            for row in 0..n {
                if board[row][column] != b'O' { continue; }
                if board[row][column - 1] != b'.' { continue; }
                else { board[row][column] = b'.'; board[row][column - 1] = b'O'; }
            }
        }
    }
}

fn e(board: &mut Vec<Vec<u8>>) {
    let n = board.len();
    for _times in 0..n {
        for column in (0..n-1).rev() {
            for row in 0..n {
                if board[row][column] != b'O' { continue; }
                if board[row][column + 1] != b'.' { continue; }
                else { board[row][column] = b'.'; board[row][column + 1] = b'O'; }
            }
        }
    }
}

fn hs(board: &Vec<Vec<u8>>) -> i64 {
    board.iter()
        .zip(1..)
        .fold(1i64, |sm, line| {
            sm + line.0.iter().zip(1..).fold(1i64, |s, c| {
                (s + (*(c.0) as i64) * c.1) % 213969421
            }) % 378199347282773
        })
}

fn mv(board: &mut Vec<Vec<u8>>) {
    n(board);
    w(board);
    s(board);
    e(board);
}

fn sm(board: &Vec<Vec<u8>>) -> usize {
    board.iter().zip((1..=board.len()).rev()).fold(0, |sm, line| {
        sm + line.0.iter().fold(0, |s, &c| if c == b'O' { s + line.1 } else { s })
    })
}

fn part_1(mut lines: Vec<Vec<u8>>) {
    n(&mut lines);
    let s = sm(&lines);
    println!("Result of part 1: {s}");
}

fn main() {
    let data = std::fs::read_to_string("res/d14.in").unwrap();
    let mut lines = data
        .split("\n")
        .filter(|&x|  x!="")
        .map(|x|  x.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();

    part_1(lines.clone());

    // mv(&mut lines);

    /*
    lines.iter().for_each(|line| {
       line.iter().for_each(|c| {
           print!("{}", *c as char);
       });
        println!();
    });
     */

    let mut hm: HashSet<i64> = HashSet::new();
    let mut coll: Vec<(Vec<Vec<u8>>, i64)> = vec![(lines.clone(), hs(&lines))];

    for x in 1.. {
        mv(&mut lines);
        let h = hs(&lines);
        let s = sm(&lines) as i64;
        coll.push((lines.clone(), h+s));
        // dbg!(h, s);
        if hm.contains(&(h+s)) {
            println!("Found it! {x}");
            break;
        }
        hm.insert(h+s);
    }

    let shs = coll.last().unwrap().1;
    let x = coll.iter().position(|x| hs(&(*x).0) + sm(&(*x).0) as i64 == shs).unwrap();
    let y = coll.len() - 1 - x;
    let q = 1000000000;
    let k = (x + 1) + (q - (x + 1)) % y;
    let sum = sm(&(coll[k].0));

    println!("Result of part 2: {sum}");
}
