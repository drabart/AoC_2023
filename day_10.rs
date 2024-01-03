fn get_char_on_position(lines: &String, position: &(usize, usize)) -> char {
    *lines.split("\n").nth(position.0).unwrap().as_bytes().get(position.1).unwrap() as char
}

fn next_position(data: &String, previous_position: &(usize, usize), current_position: &(usize, usize), board: &mut Vec<Vec<i32>>) -> (usize, usize) {
    let mut lines = data.split("\n");
    let number_of_rows = lines.clone().count();
    let number_of_columns = lines.nth(0).unwrap().as_bytes().len();

    let mut new_position = *current_position;
    let current_char = get_char_on_position(data, &current_position);

    // up
    if "S|JL".contains(current_char) && current_position.0 > 0 {
        let new_test_position = (current_position.0 - 1, current_position.1);
        if new_test_position != *previous_position {
            let char_at_position = get_char_on_position(data, &new_test_position);
            if "S|7F".contains(char_at_position) {
                board[current_position.0][current_position.1] = 1;
                new_position = new_test_position;
            }
        }
    }
    // down
    if "S|7F".contains(current_char) && current_position.0 < number_of_rows {
        let new_test_position = (current_position.0 + 1, current_position.1);
        if new_test_position != *previous_position {
            let char_at_position = get_char_on_position(data, &new_test_position);
            if "S|JL".contains(char_at_position) {
                board[current_position.0][current_position.1] = 2;
                new_position = new_test_position;
            }
        }
    }
    // left
    if "S-J7".contains(current_char) && current_position.1 > 0 {
        let new_test_position = (current_position.0, current_position.1 - 1);
        if new_test_position != *previous_position {
            let char_at_position = get_char_on_position(data, &new_test_position);
            if "S-LF".contains(char_at_position) {
                board[current_position.0][current_position.1] = 3;
                new_position = new_test_position;
            }
        }
    }
    // right
    if "S-LF".contains(current_char) && current_position.1 < number_of_columns {
        let new_test_position = (current_position.0, current_position.1 + 1);
        if new_test_position != *previous_position {
            let char_at_position = get_char_on_position(data, &new_test_position);
            if "S-J7".contains(char_at_position) {
                board[current_position.0][current_position.1] = 4;
                new_position = new_test_position;
            }
        }
    }

    return new_position;
}

fn dfs(board: &mut Vec<Vec<i32>>, position: (usize, usize), size: &(usize, usize)) {
    let number_of_rows = size.0;
    let number_of_columns = size.1;

    board[position.0][position.1] = 5;

    if position.0 > 0 && board[position.0 - 1][position.1] == 0{
        dfs(board, (position.0 - 1, position.1), size);
    }
    if position.0 + 1 < number_of_rows && board[position.0 + 1][position.1] == 0{
        dfs(board, (position.0 + 1, position.1), size);
    }
    if position.1 > 0 && board[position.0][position.1 - 1] == 0{
        dfs(board, (position.0, position.1 - 1), size);
    }
    if position.1 + 1 < number_of_columns && board[position.0][position.1 + 1] == 0{
        dfs(board, (position.0, position.1 + 1), size);
    }
}

/*
fn print_vec(v: &Vec<Vec<i32>>) {
    v.iter().for_each(|x| {
        x.iter().for_each(|&y| print!("{y}"));
        println!();
    });
}
 */

fn extrapolate(main_pipe: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let v: Vec<i32> = vec![0; main_pipe[0].len() * 2 - 1];
    let mut extrapolated_board: Vec<Vec<i32>> = vec![v.clone(); main_pipe.len() * 2 - 1];

    main_pipe
        .iter()
        .enumerate()
        .for_each(|x| x.1
            .iter()
            .enumerate()
            .for_each(|y| {
                if *y.1 != 0 {
                    extrapolated_board[x.0 * 2][y.0 * 2] = 1;
                }
                if *y.1 == 1 {
                    extrapolated_board[x.0 * 2 - 1][y.0 * 2] = 1;
                }
                if *y.1 == 2 {
                    extrapolated_board[x.0 * 2 + 1][y.0 * 2] = 1;
                }
                if *y.1 == 3 {
                    extrapolated_board[x.0 * 2][y.0 * 2 - 1] = 1;
                }
                if *y.1 == 4 {
                    extrapolated_board[x.0 * 2][y.0 * 2 + 1] = 1;
                }
            }));

    // print_vec(&extrapolated_board);

    return extrapolated_board;
}

fn main() {
    let data = std::fs::read_to_string("res/d10.in").unwrap();
    let lines = data.split("\n");

    let (row, column): (usize, usize) = lines
        .clone()
        .filter(|&line| line != "")
        .enumerate()
        .map(|x|
            (x.0,
             x.1
                 .as_bytes()
                 .iter()
                 // .inspect(|&&x| println!("{0}", x as char))
                 .position(|&character| character == b'S')
                 .unwrap_or(usize::MAX)))
        // .inspect(|x| println!("{x:?}"))
        .filter(|&x| x.1 != usize::MAX)
        .next()
        .unwrap();

    let mut main_pipe: Vec<Vec<i32>> = Vec::new();
    let mut v: Vec<i32> = Vec::new();
    lines.clone().nth(0).unwrap().as_bytes().iter().for_each(|_| v.push(0));
    lines.clone().filter(|&x| x != "").for_each(|_| main_pipe.push(v.clone()));

    // dbg!(row, column);
    let mut position = (row, column);
    let mut previous_position = position.clone();
    let mut dist = 0;
    loop {
        let new_position = next_position(&data, &previous_position, &position, &mut main_pipe);
        previous_position = position;
        position = new_position;
        let current_char = get_char_on_position(&data, &position);
        // dbg!(position, current_char);

        dist += 1;
        if current_char == 'S' {
            break;
        }
    }
    println!("Result of part 1: {}", dist / 2);
    // dbg!(dist/2);

    // print_vec(&main_pipe);

    let mut main_pipe = extrapolate(&main_pipe);

    let size = (main_pipe.len(), main_pipe[0].len());

    for i in 0..size.0 {
        if main_pipe[i][0] == 0 {
            dfs(&mut main_pipe, (i, 0), &size);
        }
        if main_pipe[i][size.1-1] == 0 {
            dfs(&mut main_pipe, (i, size.1-1), &size);
        }
    }
    for i in 0..size.1 {
        if main_pipe[0][i] == 0 {
            dfs(&mut main_pipe, (0, i), &size);
        }
        if main_pipe[size.0-1][i] == 0 {
            dfs(&mut main_pipe, (size.0-1, i), &size);
        }
    }

    let nest_size = main_pipe
        .iter()
        .enumerate()
        .filter(|&x| x.0 % 2 == 0 )
        .map(|x| x.1)
        .map(|x| x
            .iter()
            .enumerate()
            .filter(|&x| x.0 % 2 == 0)
            .map(|x| x.1)
            .fold(0, |acc, &x| if x == 0 { acc + 1 } else { acc }))
        .sum::<i32>();

    println!("Result of part 2: {nest_size}");
    // dbg!(nest_size);
}
