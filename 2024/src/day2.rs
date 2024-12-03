use std::fs::{self};

pub fn part1(input: &str) -> i32 {
    let mut safe_reports_sum = 0;

    'outer: for report in input.lines() {
        let levels = report
            .split_whitespace()
            .map(|str| str.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let is_increasing = levels[0] < levels[1];
        for w in levels.windows(2) {
            if (w[0] < w[1]) != is_increasing || w[0] == w[1] || w[0].abs_diff(w[1]) > 3 {
                continue 'outer;
            }
        }
        safe_reports_sum += 1;
    }
    safe_reports_sum
}

pub fn part2() {
    let input = fs::read_to_string("src/input_day2.txt").unwrap();

    let mut safe_reports_sum = 0;

    'outer: for report in input.lines() {
        let levels = report
            .split_whitespace()
            .map(|str| str.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let is_increasing = levels[0] < levels[1];

        let mut problem_dampener = false;
        for w in levels.windows(2) {
            if (w[0] < w[1]) != is_increasing || w[0] == w[1] || w[0].abs_diff(w[1]) > 3 {
                if problem_dampener {
                    continue 'outer;
                } else {
                    problem_dampener = true;
                }
            }
        }
        safe_reports_sum += 1;
    }

    println!("{}", safe_reports_sum);
}
