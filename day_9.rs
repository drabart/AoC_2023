fn main() {
    let data = std::fs::read_to_string("res/d9.in").unwrap();
    let data = data.split("\n");

    let mut sum = 0;
    let mut sump = 0;

    for line in data.filter(|x| *x != "") {
        let mut numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();

        let mut last: Vec<i32> = Vec::new();
        let mut first: Vec<i32> = Vec::new();
        while numbers.iter().filter(|x| **x != 0).count() != 0 {
            last.push(*numbers.last().unwrap());
            first.push(*numbers.first().unwrap());
            numbers = numbers.windows(2).map(|x| x[1] - x[0]).collect();
        }
        // dbg!(&last);
        sum += last.iter().sum::<i32>();
        let prev = first.iter().rev().map(|x| *x).reduce(|acc, x| x - acc).unwrap();
        // dbg!(prev);
        sump += prev;
    }
    println!("Result of part 1: {sum}");
    println!("Result of part 2: {sump}");
}
