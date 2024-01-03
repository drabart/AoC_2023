fn rek(spring: &String, len: usize, tests: &Vec<usize>, wym: usize, id_tests: usize, max_sum: usize, sum: usize, tab: &mut Vec<Vec<Vec<i64>>>) -> i64 {
    // println!("s: {} len: {} test: {:?} wym: {} id: {}",spring, len, tests, wym,  id_tests);

    if spring.len() == len {
        return if wym <= 1 && id_tests == (*tests).len() { 1 } else { 0 };
    }

    if max_sum < sum {
        return 0;
    }

    if tab[len][wym][id_tests] != -1 {
        return tab[len][wym][id_tests];
    }

    let mut dot = 0;
    let mut has = 0;

    if wym == 0 {
        if *spring.as_bytes().get(len).unwrap() == b'?'{
            dot = rek(spring, len+1, tests, 0, id_tests, max_sum-1, sum, tab);
            if id_tests != tests.len() {
                has = rek(spring, len + 1, tests, (*tests)[id_tests], id_tests + 1, max_sum-1, sum-1, tab);
            }
        }
        if *spring.as_bytes().get(len).unwrap() == b'#' {
            if id_tests != tests.len() {
                has = rek(spring, len + 1, tests, (*tests)[id_tests], id_tests + 1, max_sum-1, sum-1, tab);
            }
        }
        if *spring.as_bytes().get(len).unwrap() == b'.' {
            dot = rek(spring, len+1, tests, 0, id_tests, max_sum, sum, tab);
        }
    } else if wym == 1 { // required dot after hashes
        if *spring.as_bytes().get(len).unwrap() == b'?' {
            dot = rek(spring, len+1, tests, wym-1, id_tests, max_sum-1, sum, tab);
        } else if *spring.as_bytes().get(len).unwrap() == b'.' {
            dot = rek(spring, len+1, tests, wym-1, id_tests, max_sum, sum, tab);
        }
    } else { // take necessary hashes
        if *spring.as_bytes().get(len).unwrap() == b'?' || *spring.as_bytes().get(len).unwrap() == b'#' {
            has = rek(spring, len+1, tests, wym-1, id_tests,max_sum-1,sum-1, tab);
        }
    }

    tab[len][wym][id_tests] = dot + has;
    return dot + has;
}

// basic ans = 7633

fn part_2(lines: &Vec<&str>) {
    let mut sum = 0;

    for line in lines {
        let (spring, test) = line.split_once(" ").unwrap();
        let mut s = String::new();
        s.push_str(spring);
        s.push('?');
        s.push_str(spring);
        s.push('?');
        s.push_str(spring);
        s.push('?');
        s.push_str(spring);
        s.push('?');
        s.push_str(spring);
        // println!("{s}");
        let mut t =  String::new();
        t.push_str(test);
        t.push(',');
        t.push_str(test);
        t.push(',');
        t.push_str(test);
        t.push(',');
        t.push_str(test);
        t.push(',');
        t.push_str(test);
        let tests = t
            .split(",")
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<usize>>();

        let mut tab: Vec<Vec<Vec<i64>>> = vec![vec![vec![-1;40];20];120];

        let combinations: i64 = rek(&s, 0, &tests, 0, 0,
                                    s.as_bytes().iter()
                                        .fold(0, |acc, &a| if a == b'.' {acc} else {acc+1}),
                                    tests.iter().sum::<usize>(), &mut tab);

        sum += combinations;
    }

    println!("Result of part 2: {sum}");
}

fn part_1(lines: &Vec<&str>) {
    let mut sum = 0;

    for line in lines {
        let (s, t) = line.split_once(" ").unwrap();
        let s = s.parse::<String>().unwrap();
        let tests = t
            .split(",")
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<usize>>();

        let mut tab: Vec<Vec<Vec<i64>>> = vec![vec![vec![-1;40];20];120];

        let combinations: i64 = rek(&s, 0, &tests, 0, 0,
                                    s.as_bytes().iter()
                                        .fold(0, |acc, &a| if a == b'.' {acc} else {acc+1}),
                                    tests.iter().sum::<usize>(), &mut tab);

        sum += combinations;
    }

    println!("Result of part 1: {sum}");
}

fn main() {
    let data = std::fs::read_to_string("res/d12.in").unwrap();
    let lines = data.split("\n").filter(|&x|  x != "").collect::<Vec<&str>>();

    part_1(&lines);
    part_2(&lines);
}
