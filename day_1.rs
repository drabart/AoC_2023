fn part1(lines: &Vec<&str>) {
    let sum = lines.iter().fold(0, |result, &line| {
        let (first, last) = line.as_bytes().iter().fold((0, 0), |(f, l), &character| {
            if character >= b'1' && character <= b'9' {
                if f == 0 {
                    (character - b'0', character - b'0')
                } else {
                    (f, character - b'0')
                }
            } else {
                (f, l)
            }
        });
        result + (first * 10 + last) as i32
    });
    println!("Result for part 1: {sum}");
}

fn part2(lines: &Vec<&str>) {
    let digits = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let sum = lines.iter().fold(0, |result, &line| {
        let mut first = 0;
        let mut last = 0;
        for i in 0..line.len() {
            let character = line.as_bytes()[i];
            if character >= b'1' && character <= b'9' {
                if first == 0 {
                    first = character - b'0';
                }
                last = character - b'0';
            }

            let line_fragment = line.get(i..).unwrap();
            digits.iter().enumerate().for_each(|digit| {
                if line_fragment.starts_with(*(digit.1)) {
                    if first == 0 {
                        first = digit.0 as u8;
                    }
                    last = digit.0 as u8;
                }
            })
        }
        result + (first * 10 + last) as i32
    });
    println!("Result for part 2: {sum}");
}

fn main(){
    let data = std::fs::read_to_string("res/d1.in").unwrap();
    let lines = data
        .split("\n")
        .filter(|&x| x != "")
        .collect::<Vec<&str>>();

    part1(&lines);
    part2(&lines);
}