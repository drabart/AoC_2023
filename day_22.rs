use std::cmp::max;
use std::collections::{HashSet, VecDeque};

fn part1(lines: &Vec<&str>) {
    let n = lines.len();
    // x, y, z
    let mut tower = vec![vec![vec![0;1000];10];10];

    lines.iter().zip(1..).for_each(|(&block, id)| {
        let (begin, end) = block.split_once("~").unwrap();
        let begin = begin.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let end = end.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        for x in begin[0]..=end[0] {
            for y in begin[1]..=end[1] {
                for z in begin[2]..=end[2] {
                    tower[x as usize][y as usize][z as usize] = id;
                }
            }
        }
    });

    let mut lowest = vec![vec![0;10];10];
    let mut fall = vec![0;n+1];

    for z in 1..1000 {
        for x in 0..10 {
            for y in 0..10 {
                fall[tower[x][y][z]] = max(lowest[x][y] + 1, fall[tower[x][y][z]]);
            }
        }
        for x in 0..10 {
            for y in 0..10 {
                if tower[x][y][z] == 0 {
                    continue;
                }
                let nz = fall[tower[x][y][z]];
                tower[x][y][nz] = tower[x][y][z];
                if nz != z {
                    tower[x][y][z] = 0;
                }
                lowest[x][y] = nz;
            }
        }
    }

    /*
    for z in 1..10 {
        for x in 0..10 {
            for y in 0..10 {
                print!("{}", tower[x][y][z]);
            }
            println!();
        }
        println!();
    }
     */

    let mut supported_by = vec![HashSet::<i32>::new();n+1];

    for z in 1..1000 {
        for x in 0..10 {
            for y in 0..10 {
                if tower[x][y][z] == 0 || tower[x][y][z-1] == 0 || tower[x][y][z] == tower[x][y][z-1] {
                    continue;
                }
                supported_by[tower[x][y][z]].insert(tower[x][y][z-1] as i32);
            }
        }
    }

    let mut important_blocks = HashSet::<i32>::new();

    for id in 1..=n {
        // println!("{id} {}", supported_by[id].len());
        if supported_by[id].len() == 1 {
            important_blocks.insert(*supported_by[id].iter().next().unwrap());
        }
    }

    let result = n - important_blocks.len();
    println!("Result of part 1: {result}");
}

fn part2(lines: &Vec<&str>) {
    let n = lines.len();
    // x, y, z
    let mut tower = vec![vec![vec![0;1000];10];10];

    lines.iter().zip(1..).for_each(|(&block, id)| {
        let (begin, end) = block.split_once("~").unwrap();
        let begin = begin.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let end = end.split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();

        for x in begin[0]..=end[0] {
            for y in begin[1]..=end[1] {
                for z in begin[2]..=end[2] {
                    tower[x as usize][y as usize][z as usize] = id;
                }
            }
        }
    });

    let mut lowest = vec![vec![0;10];10];
    let mut fall = vec![0;n+1];

    for z in 1..1000 {
        for x in 0..10 {
            for y in 0..10 {
                fall[tower[x][y][z]] = max(lowest[x][y] + 1, fall[tower[x][y][z]]);
            }
        }
        for x in 0..10 {
            for y in 0..10 {
                if tower[x][y][z] == 0 {
                    continue;
                }
                let nz = fall[tower[x][y][z]];
                tower[x][y][nz] = tower[x][y][z];
                if nz != z {
                    tower[x][y][z] = 0;
                }
                lowest[x][y] = nz;
            }
        }
    }

    let mut supporting = vec![HashSet::<i32>::new();n+1];
    let mut supported_by = vec![HashSet::<i32>::new();n+1];

    for z in 1..1000 {
        for x in 0..10 {
            for y in 0..10 {
                if tower[x][y][z] == 0 || tower[x][y][z-1] == 0 || tower[x][y][z] == tower[x][y][z-1] {
                    continue;
                }
                supporting[tower[x][y][z-1]].insert(tower[x][y][z] as i32);
                supported_by[tower[x][y][z]].insert(tower[x][y][z-1] as i32);
            }
        }
    }

    let mut result = 0;

    for rem_id in 1..=n {
        let mut fallen = HashSet::<i32>::new();
        let mut added = VecDeque::from([rem_id]);

        while !added.is_empty() {
            let new_block = added.pop_front().unwrap();
            fallen.insert(new_block as i32);

            for block in &supporting[new_block] {
                if supported_by[*block as usize].is_subset(&fallen) {
                    added.push_back(*block as usize);
                }
            }
        }

        result += fallen.len() - 1;
    }

    println!("Result of part 2: {result}");
}

fn main() {
    let data = std::fs::read_to_string("res/d22.in").unwrap();
    let lines = data.split("\n").filter(|&x| x != "").collect::<Vec<&str>>();

    part1(&lines);
    part2(&lines);
}
