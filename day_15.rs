fn hash(s: &str) -> i32 {
    s.as_bytes().iter().fold(0,|acc, &x| ((acc + x as i32) * 17) % 256 )
}

fn main() {
    let data = std::fs::read_to_string("res/d15.in").unwrap();
    let words = data.split(",");

    println!("Result of part 1: {}", words.clone().fold(0, |acc, x| {
        acc + hash(x)
    }));

    let mut boxes: Vec<Vec<(&str, i32)>> = vec![vec![];256];
    words.for_each(|op| {
        if op.contains("-") {
            let label = op.strip_suffix("-").unwrap();
            let box_id = hash(label) as usize;
            let label_id = boxes[box_id].iter().position(|x| x.0 == label);
            if label_id.is_some() {
                boxes[box_id].remove(label_id.unwrap());
            }
        } else {
            let (label, lens) = op.split_once("=").unwrap();
            let lens = lens.parse().unwrap();
            let box_id = hash(label) as usize;
            let label_id = boxes[box_id].iter().position(|x| x.0== label);
            if label_id.is_some() {
                boxes[box_id][label_id.unwrap()] = (label, lens);
            } else {
                boxes[box_id].push((label, lens));
            }
        }
    });

    let sum = boxes.iter().zip(1..).fold(0, |acc, bo| {
        acc + bo.0.iter().zip(1..).fold(0, |box_sum, lens| {
            box_sum + lens.0.1 * lens.1
        }) * bo.1
    });

    println!("Result of part 2: {sum}");
}
