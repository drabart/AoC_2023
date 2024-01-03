use std::collections::VecDeque;

fn bfs(board: &Vec<Vec<u8>>, max_steps: i32, start_row: usize, start_column: usize, reminder: i32) -> i32{
    let rows = board.len();
    let columns = board[0].len();

    let mut tiles = VecDeque::from([(start_row, start_column, 0)]);
    let mut seen = vec![vec![0;columns];rows];
    let dir = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut result = 0;

    seen[start_row][start_column] = 1;

    while !tiles.is_empty() {
        let tile = tiles.pop_front().unwrap();

        if tile.2 > max_steps {
            break;
        }

        if tile.2 % 2 == reminder {
            result += 1;
        }

        for d in &dir {
            let new_row = tile.0 as i32 - (*d).0;
            let new_column = tile.1 as i32 - (*d).1;
            if new_row < 0 || new_column < 0 ||
                new_row >= rows as i32 || new_column >= columns as i32 {
                continue;
            }
            let new_row = new_row as usize;
            let new_column = new_column as usize;
            if seen[new_row][new_column] == 1 || board[new_row][new_column] == 1 {
                continue;
            }
            seen[new_row][new_column] = 1;
            tiles.push_back((new_row, new_column, tile.2 + 1))
        }
    }
    return result;
}

fn part1(data: &String) {
    let mut start_row = 0;
    let mut start_column = 0;
    let board = data
        .split("\n")
        .filter(|&x| x != "")
        .enumerate()
        .map(|line| {
            line.1.as_bytes().iter().enumerate().fold(Vec::new(), |mut v, tile| {
                if *tile.1 == b'#' {
                    v.push(1);
                } else if *tile.1 == b'.' {
                    v.push(0);
                } else {
                    start_row = line.0;
                    start_column = tile.0;
                    v.push(0);
                };
                v
            })
        })
        .collect::<Vec<Vec<u8>>>();

    let spots = bfs(&board, 64, start_row, start_column, 0);

    println!("Result of part 1: {spots}");
}

fn part2(data: &String) {
    let mut start_row = 0;
    let mut start_column = 0;
    let board = data
        .split("\n")
        .filter(|&x| x != "")
        .enumerate()
        .map(|line| {
            line.1.as_bytes().iter().enumerate().fold(Vec::new(), |mut v, tile| {
                if *tile.1 == b'#' {
                    v.push(1);
                } else if *tile.1 == b'.' {
                    v.push(0);
                } else {
                    start_row = line.0;
                    start_column = tile.0;
                    v.push(0);
                };
                v
            })
        })
        .collect::<Vec<Vec<u8>>>();

    let total_spots_odd = bfs(&board, 1000, start_row, start_column, 1) as i64;
    let total_spots_even = bfs(&board, 1000, start_row, start_column, 0) as i64;
    let real_steps = 26501365;
    let n = board.len();

    // we have rhombus of full boards so 4 times gaussian sum minus over counted center
    let mut quarter_board_collection = 0;
    let mut base_odd = false;
    for x in (1..(real_steps / n as i64)).rev() {
        if base_odd {
            quarter_board_collection += total_spots_odd * ((x + 1) / 2) +
                total_spots_even * (x / 2);
        } else {
            quarter_board_collection += total_spots_even * ((x + 1) / 2) +
                total_spots_odd * (x / 2);
        }
        base_odd = !base_odd;
    }
    dbg!(quarter_board_collection);
    let total_board_collection = quarter_board_collection * 4 + total_spots_odd;

    let steps_edge = (real_steps - (n as i64 / 2 + 1)) % n as i64;
    let edge_reminder = steps_edge % 2;
    let top = bfs(&board, steps_edge as i32,
                  n - 1, n / 2, edge_reminder as i32);
    let bottom = bfs(&board, steps_edge as i32,
                  0, n / 2, edge_reminder as i32);
    let left = bfs(&board, steps_edge as i32,
                  n / 2, n - 1, edge_reminder as i32);
    let right = bfs(&board, steps_edge as i32,
                  n / 2, 0, edge_reminder as i32);

    let steps_1 = (real_steps - 1) % n as i64;
    let rem_1 = edge_reminder;
    let rem_2 = (edge_reminder + 1) % 2;
    let top_left_1 = (real_steps / n as i64) *
        bfs(&board, steps_1 as i32, n - 1, n - 1, rem_1 as i32) as i64;
    let top_left_2 = (real_steps / n as i64 - 1) *
        bfs(&board, (steps_1 + n as i64) as i32, n - 1, n - 1, rem_2 as i32) as i64;

    let top_right_1 = (real_steps / n as i64) *
            bfs(&board, steps_1 as i32, n - 1, 0, rem_1 as i32) as i64;
    let top_right_2 = (real_steps / n as i64 - 1) *
            bfs(&board, (steps_1 + n as i64) as i32, n - 1, 0, rem_2 as i32) as i64;

    let bottom_left_1 = (real_steps / n as i64) *
            bfs(&board, steps_1 as i32, 0, n - 1, rem_1 as i32) as i64;
    let bottom_left_2 = (real_steps / n as i64 - 1) *
            bfs(&board, (steps_1 + n as i64) as i32, 0, n - 1, rem_2 as i32) as i64;

    let bottom_right_1 = (real_steps / n as i64) *
            bfs(&board, steps_1 as i32, 0, 0, rem_1 as i32) as i64;
    let bottom_right_2 = (real_steps / n as i64 - 1) *
            bfs(&board, (steps_1 + n as i64) as i32, 0, 0, rem_2 as i32) as i64;


    dbg!(total_board_collection, top, top_left_1, top_left_2);

    let result = total_board_collection + (top + bottom + left + right) as i64 +
        top_left_1 + top_left_2 + top_right_1 + top_right_2 +
        bottom_left_1 + bottom_left_2 + bottom_right_1 + bottom_right_2;

    println!("Result of part 2: {result}");
}

fn main() {
    let data = std::fs::read_to_string("res/d21.in").unwrap();

    part1(&data);
    part2(&data);
}
