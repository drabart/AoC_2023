fn do_your_thing(commands: &Vec<(u8, i64)>) -> i64 {
    let mut x = 0;
    let mut y = 0;

    let mut poly = commands.iter().fold(Vec::new(), |mut v, &a| {
        v.push((x,y));
        match a.0 {
            b'U' => y -= a.1,
            b'D' => y += a.1,
            b'R' => x += a.1,
            b'L' => x -= a.1,
            _ => {}
        }
        v
    });
    poly.push((0, 0));

    let area = poly.windows(2).fold(0, |a, window| {
        a + ((*window)[0].0 * (*window)[1].1) - ((*window)[0].1 * (*window)[1].0) +
            ((*window)[0].0 - (*window)[1].0).abs()+ ((*window)[0].1 - (*window)[1].1).abs()
    });
    return area / 2 + 1;
}

fn main() {
    let data = std::fs::read_to_string("res/d18.in").unwrap();
    let (commands, colors) = data.split("\n")
        .filter(|&x| x != "")
        .fold((Vec::new(), Vec::new()), |(mut cm, mut co), s| {
            let (a, b) = s.rsplit_once(" ").unwrap();
            cm.push(a);
            co.push(b);
            (cm, co)
        });

    let commands = commands.iter().map(|&command| {
            let x = command.split_once(" ").unwrap();
            (x.0.as_bytes()[0], x.1.parse::<i64>().unwrap())
        })
        .collect::<Vec<(u8, i64)>>();

    let res_part_1 = do_your_thing(&commands);
    println!("Result of part 1: {res_part_1}");

    let commands = colors
        .iter()
        .map(|&command| {
            let c = command
                .strip_prefix("(#").unwrap()
                .strip_suffix(")").unwrap();
            let d = i64::from_str_radix(c, 16).unwrap();
            let dir = match d%16 {
                0 => b'R',
                1 => b'D',
                2 => b'L',
                3 => b'U',
                _ => b'X'
            };
            let d = d / 16;
            (dir, d)
        })
        .collect::<Vec<(u8, i64)>>();

    let res_part_2 = do_your_thing(&commands);
    println!("Result of part 2: {res_part_2}");
}
