use std::collections::VecDeque;

fn rotate(clockwise: bool, mut dir_id: i32) ->(i32, i32) {
    if clockwise {
        dir_id = (dir_id * 2) % 15;
    } else {
        dir_id = dir_id / 2;
        if dir_id == 0 {dir_id += 8};
    }
    match dir_id {
        1 => (0,1),
        2 => (1,0),
        4 => (0,-1),
        8 => (-1,0),
        _ => (0,0)
    }
}

fn valid_position(board: &Vec<Vec<u8>>, pos: (i32, i32)) -> bool {
    return pos.0 >= 0 && pos.1 >= 0 && pos.0 < board.len() as i32 && pos.1 < board[0].len() as i32;
}

fn next_pos(pos: (i32, i32), dir: (i32, i32)) -> (i32, i32) {
    return (pos.0 + dir.0, pos.1 + dir.1);
}

fn bfs(board: &Vec<Vec<u8>>,
       ipos:(i32, i32), idir:(i32, i32),
       previous_states: &mut Vec<Vec<Vec<Vec<i32>>>>,
       max_fwd: i32, min_fwd: i32) {

    let mut deq = VecDeque::from([(ipos, idir, 0, 0)]);

    while !deq.is_empty() {
        let pos = deq.front().unwrap().0;
        let dir= deq.front().unwrap().1;
        let moves = deq.front().unwrap().2;
        let mut dist= deq.front().unwrap().3;
        deq.pop_front();

        let dir_id = match dir {
            (0, 1) => 1,
            (1, 0) => 2,
            (0, -1) => 4,
            (-1, 0) => 8,
            _ => 0
        };

        dist += board[pos.0 as usize][pos.1 as usize] as i32;
        let ps = previous_states[pos.0 as usize][pos.1 as usize][dir_id as usize][moves as usize];
        if pos == (board.len() as i32 - 1, board[0].len() as i32 - 1) {
            if moves >= min_fwd + 1 {
                if ps > dist {
                    previous_states[pos.0 as usize][pos.1 as usize][dir_id as usize][moves as usize] = dist;
                }
            }
            continue;
        }
        if ps > dist {
            previous_states[pos.0 as usize][pos.1 as usize][dir_id as usize][moves as usize] = dist;
        } else {
            continue;
        }

        if moves < max_fwd {
            let np = next_pos(pos, dir);
            if valid_position(board, np) {
                deq.push_back((np, dir, moves + 1, dist));
            }
        }

        if moves >= min_fwd {
            let nd = rotate(true, dir_id);
            let np = next_pos(pos, nd);
            if valid_position(board, np) {
                deq.push_back((np, nd, 1, dist));
            }
            let nd = rotate(false, dir_id);
            let np = next_pos(pos, nd);
            if valid_position(board, np) {
                deq.push_back((np, nd, 1, dist));
            }
        }
    }
}

fn part_1(board: &Vec<Vec<u8>>) {
    let mut states = vec![vec![vec![vec![i32::MAX; 12]; 10]; board[0].len()]; board.len()];
    bfs(board, (0, 0), (0, 1), &mut states, 3, 0);

    // dbg!(&states[(*board).len() -1][(*board)[0].len() - 1]);

    let res = states[(*board).len() -1][(*board)[0].len() - 1]
        .iter()
        .fold(i32::MAX, |a, l| {
            a.min( l.iter().fold(i32::MAX, |a, &x| if x > 0 { a.min(x) } else { a } ))
        });
    let res = res - (*board)[0][0] as i32;

    println!("Result of part 1: {res}");
}

fn main() {
    let data = std::fs::read_to_string("res/d17.in").unwrap();
    let board = data
        .split("\n")
        .filter(|&x| x!="")
        .map(|x| x.as_bytes().iter().fold(Vec::new(), |mut v, &c| {
            v.push(c - b'0');
            v
        }))
        .collect::<Vec<Vec<u8>>>();

    part_1(&board);

    let mut states = vec![vec![vec![vec![i32::MAX; 12]; 10]; board[0].len()]; board.len()];
    bfs(&board, (0, 0), (0, 1), &mut states, 10, 4);

    let res = states[board.len() -1][board[0].len() - 1]
        .iter()
        .fold(i32::MAX, |a, l| {
        a.min( l.iter().fold(i32::MAX, |a, &x| if x > 0 { a.min(x) } else { a } ))
    });
    let res = res - board[0][0] as i32 - 3;

    println!("Result of part 2: {res}");
}
