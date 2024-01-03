fn part_1(boards: &Vec<Vec<Vec<i32>>>) {
    let mut sum = 0;

    boards.iter().for_each(|board| {
        board.iter().zip(0..)
            .for_each(|row| {
                let mut is_mirror = 0;
                let mut low = row.1;
                let mut high = row.1 + 1;
                if row.1 + 1 == board.len() as i32 {
                    return;
                }
                while low >= 0 && (high as usize) < board.len() {
                    is_mirror += (0..board[0].len()).fold(0, |a, column_id| {
                        if board[low as usize][column_id] != board[high as usize][column_id] { a + 1 } else { a }
                    });
                    low -= 1;
                    high += 1;
                }
                if is_mirror == 0 {
                    // dbg!(row.1);
                    sum += 100 * (row.1 + 1);
                }
            });
    });

    // dbg!(sum);

    boards.iter().for_each(|board| {
        (0..board[0].len() as i32).for_each(|column_id| {
            let mut is_mirror = 0;
            let mut low = column_id;
            let mut high = column_id + 1;
            if column_id + 1 == board[0].len() as i32 {
                return;
            }
            while low >= 0 && (high as usize) < board[0].len() {
                is_mirror += (0..board.len()).fold(0, |a, row_id| {
                    if board[row_id][low as usize] != board[row_id][high as usize] { a + 1 } else { a }
                });
                low -= 1;
                high += 1;
            }
            if is_mirror == 0{
                sum += column_id + 1;
            }
        })
    });

    println!("Result of part 1: {sum}");
}

fn part_2(boards: &Vec<Vec<Vec<i32>>>) {
    let mut sum = 0;

    boards.iter().for_each(|board| {
        board.iter().zip(0..)
            .for_each(|row| {
                let mut is_mirror = 0;
                let mut low = row.1;
                let mut high = row.1 + 1;
                while low >= 0 && (high as usize) < board.len() {
                    is_mirror += (0..board[0].len()).fold(0, |a, column_id| {
                        if board[low as usize][column_id] != board[high as usize][column_id] { a + 1 } else { a }
                    });
                    low -= 1;
                    high += 1;
                }
                if is_mirror == 1 {
                    sum += 100 * (row.1 + 1);
                }
            });
    });

    boards.iter().for_each(|board| {
        (0..board[0].len() as i32).for_each(|column_id| {
            let mut is_mirror = 0;
            let mut low = column_id;
            let mut high = column_id + 1;
            while low >= 0 && (high as usize) < board[0].len() {
                is_mirror += (0..board.len()).fold(0, |a, row_id| {
                    if board[row_id][low as usize] != board[row_id][high as usize] { a + 1 } else { a }
                });
                low -= 1;
                high += 1;
            }
            if is_mirror == 1{
                sum += column_id + 1;
            }
        })
    });

    println!("Result of part 2: {sum}");
}

fn main() {
    let data = std::fs::read_to_string("res/d13.in").unwrap();
    let boards = data
        .split("\n\n")
        .map(|board| board
            .split("\n")
            .filter(|&line| line != "")
            .map(|line| line.as_bytes())
            .fold(Vec::new(), |mut v, line|{v.push(
                line
                    .iter()
                    .map(|&c| if c == b'.'{ 0 }
                        else{ 1 })
                    .collect::<Vec<i32>>()
            )
            ;v})
        )
        .collect::<Vec<Vec<Vec<i32>>>>();

    part_1(&boards);
    part_2(&boards);
}
