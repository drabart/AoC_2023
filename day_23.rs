use std::collections::{BinaryHeap, VecDeque};

fn part1(board: &Vec<Vec<i32>>) {
    let rows = board.len();
    let columns = board[0].len();

    let row = 1;
    let column = 1;

    let dir = vec![(0, 1),  (0, -1), (-1, 0), (1, 0)];

    let mut dist = vec![vec![0;columns];rows];
    let mut priority_queue = BinaryHeap::new();

    priority_queue.push((0, row, column, 0, 0));

    while !priority_queue.is_empty() {
        let node = priority_queue.pop().unwrap();
        let row = node.1;
        let column = node.2;
        // dbg!(node);

        if row == rows - 2 && column == columns - 2 {
            continue;
        }

        if board[row][column] >= 2 {
            let nr = (row as i32 + dir[board[row][column] as usize - 2].0) as usize;
            let nc = (column as i32 + dir[board[row][column] as usize - 2].1) as usize;
            if dist[nr][nc] < dist[row][column] + 1 {
                dist[nr][nc] = dist[row][column] + 1;
                priority_queue.push((dist[nr][nc], nr, nc, row, column));
            }
            continue;
        }

        dir.iter().enumerate().for_each(|(i, &d)| {
            let nr = (row as i32 + d.0) as usize;
            let nc = (column as i32 + d.1) as usize;

            if board[nr][nc] == 1 || (nr == node.3 && nc == node.4){
                return;
            }

            if board[nr][nc] >= 2 && (i^1) == board[nr][nc] as usize - 2 {
                return;
            }

            if dist[nr][nc] < dist[row][column] + 1 {
                dist[nr][nc] = dist[row][column] + 1;
                priority_queue.push((dist[nr][nc], nr, nc, row, column));
            }
        });
    }

    /*
    dist.iter().for_each(|v| {
        v.iter().for_each(|e| {
            print!("{e:2} ");
        });
        println!();
    });
     */

    println!("Result of part 1: {}", dist[rows-2][columns-2] + 2)
}

fn backtrack(node: usize, graph: &Vec<Vec<(usize, i32)>>, dist: &mut Vec<i32>, seen: &mut Vec<i32>) -> i32 {
    if node == 1 {
        return dist[1];
    }
    // println!("{node}");
    seen[node] = 1;
    let mut ret = 0;
    for x in 0..graph[node].len() {
        let e = graph[node][x];

        if seen[e.0] == 1 || dist[node] + e.1 <= dist[e.0] {
            continue;
        }

        let x = dist[e.0];
        dist[e.0] = dist[node] + e.1;
        ret = ret.max(backtrack(e.0, graph, dist, seen));
        dist[e.0] = x;
    }
    seen[node] = 0;
    ret
}

fn part2(board: &Vec<Vec<i32>>) {
    let board = board
        .iter()
        .map(|v| {
            v.iter()
                .map(|&e| if e == 0 || e > 1 { 0 } else { 1 })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut sus_spots = vec![];
    let rows = board.len();
    let columns = board[0].len();

    sus_spots.push((1, 1));
    sus_spots.push((rows-2, columns-2));

    for row in 1..rows-1 {
        for column in 1..columns-1 {
            if board[row][column] == 0 && (board[row-1][column] + board[row+1][column] +
                board[row][column-1] + board[row][column+1]) <= 1 {
                sus_spots.push((row, column));
                // println!("sus {row} {column}");
            }
        }
    }

    // dbg!(&sus_spots);

    let mut graph = vec![vec![];sus_spots.len()];
    let dir = vec![(0, 1),  (0, -1), (-1, 0), (1, 0)];
    let mut seen = vec![vec![0;columns];rows];

    for x in 0..sus_spots.len() {
        let spot = sus_spots[x];
        let mut queue = VecDeque::new();
        queue.push_back((spot, 0));
        seen[spot.0][spot.1] = 1;

        while !queue.is_empty() {
            let ((r, c), ds) = queue.pop_front().unwrap();

            if sus_spots.contains(&(r, c)) && (r, c) != spot {
                let other_spot = sus_spots.iter().position(|&e| e == (r, c) ).unwrap();
                graph[x].push((other_spot, ds));
                graph[other_spot].push((x, ds));
                seen[r][c] = 0;
                continue;
            }

            dir.iter().for_each(|&d| {
                let nr = (r as i32 + d.0) as usize;
                let nc = (c as i32 + d.1) as usize;

                if board[nr][nc] == 1 || seen[nr][nc] == 1 {
                    return;
                }

                seen[nr][nc] = 1;

                queue.push_back(((nr, nc), ds + 1));
            });

        }
    }

    /*
    graph.iter().zip(0..).for_each(|n| {
        print!("{}: ", n.1);
       n.0.iter().for_each(|e| {
           print!("{} {}  ", (*e).0, (*e).1);
       }) ;
        println!();
    });

     */

    let mut dist = vec![0;sus_spots.len()];
    let mut seen = vec![0;sus_spots.len()];

    let res = backtrack(0, &graph, &mut dist, &mut seen);

    println!("Result of part 2: {}", res + 2);
}

fn main() {
    let data = std::fs::read_to_string("res/d23.in").unwrap();
    let mut board = data
        .split("\n")
        .filter(|&x| x != "")
        .map(|line| {
            line.as_bytes().iter().fold(Vec::new(), |mut v, &c| {
                v.push(match c {
                    b'.' => 0,
                    b'#' => 1,
                    b'>' => 2,
                    b'<' => 3,
                    b'^' => 4,
                    _ => 5
                });
                v
            })
        })
        .collect::<Vec<Vec<i32>>>();

    let x = board.len();
    let y = board[0].len();

    board[0][1] = 1;
    board[x-1][y-2] = 1;
    part1(&board);
    part2(&board);
}
