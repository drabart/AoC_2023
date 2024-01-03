fn handle_color(color: &str, suffix: &str) -> i32 {
    let number = color.strip_suffix(suffix);
    let number: i32 = number
        .unwrap()
        .parse()
        .unwrap();
    number
}

fn part1(lines: &Vec<&str>) {
    let sum = lines.iter().fold(0, |result, &line| {
        let (game_id, moves) = line.split_once(": ").unwrap();
        let game_id = game_id.
            strip_prefix("Game ").unwrap()
            .parse::<i32>().unwrap();

        let moves = moves.split("; ");
        let (red, blue, green) = moves.fold((0, 0, 0), |(mut r, mut b, mut g), turn| {
            let colors = turn.split(", ");
            for color in colors {
                if color.ends_with("red") {
                    r = r.max(handle_color(color, " red"));
                }
                if color.ends_with("blue") {
                    b = b.max(handle_color(color, " blue"));
                }
                if color.ends_with("green") {
                    g = g.max(handle_color(color, " green"));
                }
            }
            (r, b, g)
        });

        if red <= 12 && blue <= 14 && green <= 13 {
            result + game_id
        } else {
            result
        }
    });
    println!("Result for part 1: {sum}");
}

fn part2(lines: &Vec<&str>) {
    let sum = lines.iter().fold(0, |result, &line| {
        let (_game_id, moves) = line.split_once(": ").unwrap();

        let moves = moves.split("; ");
        let (red, blue, green) = moves.fold((0, 0, 0), |(mut r, mut b, mut g), turn| {
            let colors = turn.split(", ");
            for color in colors {
                if color.ends_with("red") {
                    r = r.max(handle_color(color, " red"));
                }
                if color.ends_with("blue") {
                    b = b.max(handle_color(color, " blue"));
                }
                if color.ends_with("green") {
                    g = g.max(handle_color(color, " green"));
                }
            }
            (r, b, g)
        });
        result + red * blue * green
    });
    println!("Result for part 2: {sum}");
}

fn main() {
    let data = std::fs::read_to_string("res/day2.in").unwrap();
    let lines = data.split("\n").filter(|&x| x != "").collect::<Vec<&str>>();

    part1(&lines);
    part2(&lines);
}
