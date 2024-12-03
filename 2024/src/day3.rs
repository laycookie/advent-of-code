use std::fs;

pub fn part1() {
    let input = fs::read_to_string("src/input_day3.txt").unwrap();

    let mut sum = 0;

    let mul = input.split("mul(");

    for pot in mul {
        let parametars = match pot.split_once(")") {
            Some((parametars, _)) => parametars,
            None => continue,
        };

        let nums = match parametars.split_once(",") {
            Some(nums) => nums,
            None => continue,
        };

        let first_num = match nums.0.parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let second_num = match nums.1.parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // let second_num = match pot.split_once(")") {
        //     Some((num, _)) => match num.parse::<i32>() {
        //         Ok(num) => num,
        //         Err(_) => continue,
        //     },
        //     None => continue,
        // };

        println!("{:?}, {:?}", first_num, second_num);
        sum += first_num * second_num;
    }
    println!("Sum: {}", sum);
}

pub fn part2() {
    let input = fs::read_to_string("src/input_day3.txt").unwrap();

    let input = input.replace("don't", "opcode1");
    let input = input.replace("do", "opcode0");
    let input = input.replace("mul", "opcode2");

    let mut sum = 0;
    let mut active = true;
    for opcode in input.split("opcode") {
        let op_num = match opcode.chars().next() {
            Some(num) => num,
            None => continue,
        };

        println!("{:?}", opcode);
        println!("{:?}", op_num);

        match op_num {
            '0' => {
                // do
                active = true
            }
            '1' => {
                // don't
                active = false
            }
            '2' => {
                if active {
                    // mul
                    let params = opcode
                        .char_indices()
                        .nth(2)
                        .map(|(idx, _)| &opcode[idx..])
                        .unwrap_or("");
                    let params = match params.split_once(")") {
                        Some((params, _)) => params,
                        None => continue,
                    };

                    let nums = match params.split_once(",") {
                        Some(nums) => nums,
                        None => continue,
                    };

                    let first_num = match nums.0.parse::<i32>() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };

                    let second_num = match nums.1.parse::<i32>() {
                        Ok(num) => num,
                        Err(_) => continue,
                    };

                    println!("{:?}, {:?}", first_num, second_num);
                    sum += first_num * second_num;
                }
            }
            _ => continue,
        };
    }

    println!("{}", sum);
}
