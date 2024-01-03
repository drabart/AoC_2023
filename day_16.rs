#[derive(Copy, Clone, Debug)]
struct Beam  {
    pos: (i32, i32),
    dir: (i32, i32)
}

fn test(start_beam:Beam, board:&Vec<Vec<u8>>)->i32{
    let mut beams = vec![start_beam];
    let mut energized = vec![vec![0;board[0].len()];board.len()];

    while !beams.is_empty() {
        let mut to_remove: Vec<usize> = Vec::new();
        let mut to_add: Vec<Beam> = Vec::new();
        for beam_id in 0..beams.len() {
            let beam = &mut beams[beam_id];
            let dir_id = match beam.dir {
                (0,1) => 1,
                (0,-1) => 2,
                (1,0) => 4,
                (-1,0) => 8,
                _ => 0
            };
            // println!("{:?}: {dir_id}",beam.dir);
            let p = (beam.pos.0 as usize, beam.pos.1 as usize);
            if energized[p.0][p.1] & dir_id > 0 {
                to_remove.push(beam_id);
                continue;
            }
            energized[p.0][p.1] |= dir_id;
            match board[p.0][p.1] {
                b'.' => {},
                b'-' => {
                    // print!("hi");
                    if beam.dir.0 != 0 {
                        to_add.push(Beam {
                            pos: (beam.pos.0, beam.pos.1 + 1),
                            dir: (0, 1),
                        });
                        beam.dir = (0, -1);
                        // print!("hi2");
                    }
                },
                b'|' => {
                    if beam.dir.1 != 0 {
                        to_add.push(Beam {
                            pos: (beam.pos.0 + 1, beam.pos.1),
                            dir: (1, 0),
                        });
                        beam.dir = (-1, 0);
                    }
                },
                b'/' => {
                    beam.dir = (-beam.dir.1, -beam.dir.0)
                },
                b'\\' => {
                    // print!("hi");
                    beam.dir = (beam.dir.1, beam.dir.0)
                },
                _ => {}
            }
            // println!("{:?}", beam);
            beam.pos = (beam.pos.0 + beam.dir.0, beam.pos.1 + beam.dir.1);
            if beam.pos.0 < 0 || beam.pos.1 < 0 ||
                beam.pos.0 >= board.len() as i32 ||  beam.pos.1 >= board[0].len() as i32 {
                to_remove.push(beam_id);
            }
        }
        to_remove.iter().rev().for_each(|&x| {beams.remove(x);});
        to_add.iter().for_each(|x| {
            if x.pos.0 >= 0 && x.pos.1 >= 0 &&
                x.pos.0 < board.len() as i32 && x.pos.1 < board[0].len() as i32 {
                beams.push((*x).clone());
            }
        });
        //  println!("{:?}", beams);
    }

    let sum = energized.iter().fold(0, |s, line|{
        // println!();
        s + line.iter().fold(0, |a, &x|{
            // print!("{}",x);
            if x > 0 { a + 1 } else { a }
        })
    });

    return sum;
}

fn main() {
    let data = std::fs::read_to_string("res/d16.in").unwrap();
    let board = data
        .split("\n")
        .filter(|&line| line!="")
        .map(|line| {
            line.as_bytes().iter()
                .fold(Vec::new(), |mut v, &c| {
                    v.push(c);
                    v
                })
        })
        .collect::<Vec<Vec<u8>>>();

    // dbg!(test(Beam{pos:(0,0),dir:(0,1)},&board));

    let mut mx = 0;
    for i in 0..board.len() {
        let res = test(Beam { pos: (i as i32, 0), dir: (0, 1) }, &board);
        mx = mx.max(res);
        if i == 0 {
            println!("Result of part 1: {mx}");
        }
        let res = test(Beam { pos: (i as i32, board[0].len() as i32 - 1), dir: (0, -1) }, &board);
        mx = mx.max(res);
    }
    for i in 0..board[0].len() {
        let res = test(Beam { pos: (0, i as i32), dir: (1, 0) }, &board);
        mx = mx.max(res);
        let res = test(Beam { pos: (board.len() as i32 - 1, i as i32), dir: (-1, 0) }, &board);
        mx = mx.max(res);
    }
    println!("Result of part 2: {mx}");
}
