use std::fs;

fn correct_spot(row: i32, column: i32, data: &Vec<&str>) -> bool {
    if row < 0 || column < 0 {
        return false;
    }
    if row >= data.len() as i32 {
        return false;
    }
    if column >= data[row as usize].len() as i32 {
        return false;
    }
    if ".0123456789".contains(*data[row as usize].as_bytes().get(column as usize).unwrap() as char)
    {
        return false;
    }
    return true;
}

fn check_if_near_symbol(
    row: i32,
    column: i32,
    data: &Vec<&str>,
    symbol: &mut Vec<Vec<i32>>,
) -> bool {
    let mut symbol_near = false;
    let dir = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]];
    for d in dir {
        if correct_spot(row + d[0], column + d[1], data) {
            symbol[(row + d[0]) as usize][(column + d[1]) as usize] += 1;
            symbol_near = true;
        }
    }
    return symbol_near;
}

fn correct_spot_gear(row: i32, column: i32, data: &Vec<&str>) -> bool {
    if row < 0 || column < 0 {
        return false;
    }
    if row >= data.len() as i32 {
        return false;
    }
    if column >= data[row as usize].len() as i32 {
        return false;
    }
    if !"0123456789".contains(*data[row as usize].as_bytes().get(column as usize).unwrap() as char)
    {
        return false;
    }
    return true;
}

fn check_if_near_2_numbers(
    row: i32,
    column: i32,
    data: &Vec<&str>,
    numbers: &Vec<Vec<i32>>,
) -> i32 {
    let dir = [[-1, -1], [-1, 0], [-1, 1], [0, -1], [0, 1], [1, -1], [1, 0], [1, 1]];
    let mut res = [0; 8];
    let mut value = 1;
    let mut k = 0;
    for i in 0..dir.len() {
        if correct_spot_gear(row + dir[i][0], column + dir[i][1], data) {
            k += 1;
            res[i] = numbers[(row + dir[i][0]) as usize][(column + dir[i][1]) as usize];
            if (i == 1 || i == 2 || i == 6 || i == 7) && res[i-1] > 0 { res[i-1] = 0; k -= 1; }
        }
    }
    if k != 2 { return 0; }
    for i in 0..res.len() {
        if res[i] == 0 { continue; }
        let mut c: usize = (column + dir[i][1] + 1) as usize;
        while c < numbers[row as usize].len() {
            if numbers[(row + dir[i][0]) as usize][c] == 0 { break; }
            res[i] = numbers[(row + dir[i][0]) as usize][c];
            c += 1;
        }
        value *= res[i];
    }
    return value;
}

fn part1(lines: &Vec<&str>) {
    let mut symbol_use: Vec<Vec<i32>> = Vec::new();
    let mut v: Vec<i32> = Vec::new();
    for _ in 0..lines[0].len() {v.push(0)}
    for _ in 0..lines.len() {symbol_use.push(v.clone())};
    let mut number_to = symbol_use.clone();

    let mut sum = 0;

    for row in 0..lines.len() {
        let mut x = 0;
        let mut has_symbol = false;
        for column in 0..lines[row].len() {
            if "0123456789".contains(lines[row].chars().nth(column).unwrap()) {
                x = x * 10 + *lines[row].as_bytes().get(column).unwrap() as i32 - 48;
                number_to[row][column] = x;
                if !has_symbol {
                    has_symbol =
                        check_if_near_symbol(row as i32, column as i32, &lines, &mut symbol_use);
                }
            } else if x > 0 && has_symbol {
                sum += x;
                x = 0;
                has_symbol = false;
            } else {
                x = 0;
                has_symbol = false;
            }
        }
        if x > 0 && has_symbol {
            sum += x;
        }
    }
    println!("Result of part 1: {sum}");
}

fn part2(lines: &Vec<&str>) {
    let mut symbol_use: Vec<Vec<i32>> = Vec::new();
    let mut v: Vec<i32> = Vec::new();
    for _ in 0..lines[0].len() {v.push(0)}
    for _ in 0..lines.len() {symbol_use.push(v.clone())};
    let mut number_to = symbol_use.clone();

    for row in 0..lines.len() {
        let mut x = 0;
        for column in 0..lines[row].len() {
            if "0123456789".contains(lines[row].chars().nth(column).unwrap()) {
                x = x * 10 + *lines[row].as_bytes().get(column).unwrap() as i32 - 48;
                number_to[row][column] = x
            } else {
                x = 0;
            }
        }
    }

    let mut sum = 0;

    for row in 0..lines.len() {
        for column in 0..lines[row].len() {
            if lines[row].chars().nth(column).unwrap() != '*' {
                continue;
            }
            let a =  check_if_near_2_numbers(row as i32, column as i32, &lines, &number_to);
            sum += a;
        }
    }
    println!("Result of part 2: {sum}");
}

fn main() {
    let data = fs::read_to_string("res/day3.in").unwrap();
    let lines: Vec<&str> = data.split("\n").collect();

    part1(&lines);
    part2(&lines);
}
