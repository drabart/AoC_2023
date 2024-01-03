fn calculate(v: &Vec<Vec<i64>>, is_row_empty: &Vec<bool>, is_column_empty: &Vec<bool>, mut row_dist: Vec<(i64, i64)>, empties: &Vec<i64>, empty_row_number: i64, expansion_constant: i64) -> i64 {
    let dist_sum: i64 = v
        .iter()
        .enumerate()
        .fold((0, 0), |(acc, empty_rows), x| {
            if is_row_empty[x.0] { (acc, empty_rows + 1) }
            else {
                (acc + x.1.iter()
                    .enumerate()
                    .fold((0, 0), |(mut result_sum, empty_columns), y| {
                        if *y.1 == 0 {
                            return if is_column_empty[y.0] {
                                (result_sum, empty_columns + 1)
                            } else {
                                (result_sum, empty_columns)
                            };
                        }
                        row_dist.iter()
                            .enumerate()
                            .for_each(|column| {
                                result_sum += column.1.1 *
                                    ((y.0 as i64 + empty_columns * expansion_constant) -
                                        (column.0 as i64 + empties[column.0] * expansion_constant)).abs();
                                result_sum += column.1.0 - column.1.1 *
                                    (v.len() as i64 + empty_row_number * expansion_constant -
                                        (x.0 as i64 + empty_rows * expansion_constant)) as i64;
                            });
                        row_dist[y.0].0 += (v.len() as i64 + empty_row_number * expansion_constant -
                            (x.0 as i64 + empty_rows * expansion_constant)) as i64;
                        row_dist[y.0].1 += 1;
                        return (result_sum, empty_columns);
                    }).0,
                 empty_rows)
            }
        }).0;

    return dist_sum;
}

fn main() {
    let data = std::fs::read_to_string("res/d11.in").unwrap();
    let lines = data.split("\n");

    let v: Vec<Vec<i64>> = lines
        .filter(|&x| x != "")
        .map(|x| x
            .as_bytes()
            .iter()
            .map(|&x| if x == b'.' { 0 } else { 1 } )
            .collect())
        .collect();

    /*
    v.iter().for_each(|x| {
        x.iter().for_each(|&y| print!("{y}"));
        println!();
    });
    println!();
     */

    let mut is_row_empty = vec![false; v.len()];
    let mut is_column_empty = vec![false; v[0].len()];

    v.iter()
        .enumerate()
        .for_each(|x| {
            if x.1.iter().sum::<i64>() == 0 { is_row_empty[x.0] = true; }
        });

    let mut column_sum: Vec<i64> = vec![0; v[0].len()];
    v.iter()
        .for_each(|x| x
        .iter()
        .enumerate()
        .for_each(|y| column_sum[y.0] += *y.1));

    column_sum.iter()
        .enumerate()
        .for_each(|x| if *x.1 == 0 { is_column_empty[x.0] = true; });

    let empty_row_numer = is_row_empty.iter().map(|&x| if x { 1 } else { 0 }).sum::<i64>();
    // let empty_column_numer = is_column_empty.iter().map(|&x| if x { 1 } else { 0 }).sum::<i64>();

    // dbg!(empty_row_numer);
    // dbg!(empty_column_numer);
    let row_dist = vec![(0, 0); v[0].len()];
    let empties = is_column_empty
        .iter()
        .fold(Vec::new(), |mut acc, &x| {
            if acc.len() == 0 {acc.push( if x { 1 } else {0} )}
            if x { acc.push(*acc.last().unwrap() + 1); } else { acc.push(*acc.last().unwrap()) }
            acc
        });

    println!("Result of part 1: {}", calculate(&v, &is_row_empty, &is_column_empty, row_dist.clone(), &empties, empty_row_numer, 2 - 1));
    println!("Result of part 2: {}", calculate(&v, &is_row_empty, &is_column_empty, row_dist.clone(), &empties, empty_row_numer, 1000000 - 1));
}
