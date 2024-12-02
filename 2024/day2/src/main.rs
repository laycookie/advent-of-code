use std::{fs::File, io::Read};

fn main() {
    // Reading a file
    let mut input_file = File::open("src/input.txt").unwrap();
    let mut input = String::new();
    input_file.read_to_string(&mut input).unwrap();

    // Analyzing
    let mut safe_reports_sum = 0;

    'outer: for report in input.lines() {
        let levels = report
            .split(" ")
            .collect::<Vec<_>>()
            .iter()
            .map(|str| str.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let is_increasing = levels[0] < levels[1];

        for w in levels.windows(2) {
            if (w[0] < w[1]) != is_increasing || w[0] == w[1] || w[0].abs_diff(w[1]) > 3 {
                continue 'outer;
            } else {
                println!("{:?}, dif:{}", w, w[0].abs_diff(w[1]));
            }
        }
        safe_reports_sum += 1;
        println!("safe_reports_sum: {}, {:?}", safe_reports_sum, levels);
    }

    println!("{safe_reports_sum}");
}
