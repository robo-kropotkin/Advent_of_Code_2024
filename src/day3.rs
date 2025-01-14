pub mod solution {
    use::regex::Regex;
    use crate::utils::read_lines;
    pub fn part1() -> () {
        let re = Regex::new(r"(mul\(\d+\,\d+\))").unwrap();
        find_sum(re);
    }
    pub fn part2() -> () {
        let instruction_re = Regex::new(r"(mul\(\d+\,\d+\))").unwrap();
        let do_re = Regex::new(r"do\(\)").unwrap();
        let dont_re = Regex::new(r"don\'t\(\)").unwrap();
        let data_address = "data/day3.txt";
        let line_collection: Vec<String> = read_lines(&data_address).unwrap().map(|line| String::from(line.unwrap())).collect();
        let mut lines = line_collection.join("\n");
        let mut sum = 0;
        loop {
            let dont_opt = dont_re.find(&lines);
            match dont_opt {
                Some(m) => {
                    let instructions: Vec<&str> = instruction_re.find_iter(&lines[..m.start()]).map(|m| m.as_str()).collect();
                    for instruction in instructions {
                        let nums: Vec<i32> = instruction[4..instruction.len() - 1].split(",").map(|num| num.parse::<i32>().expect("Must be numerical!")).collect();
                        sum += nums[0] * nums[1];
                    }
                    lines = lines[m.end()..].to_string();
                    let do_opt = do_re.find(&lines);
                    match do_opt {
                        Some(m) => {
                            lines = lines[m.end()..].to_string();
                            continue;
                        }
                        None => {
                            break;
                        }
                    }
                }
                None => {
                    let instructions: Vec<&str> = instruction_re.find_iter(&lines).map(|m| m.as_str()).collect();
                    for instruction in instructions {
                        let nums: Vec<i32> = instruction[4..instruction.len() - 1].split(",").map(|num| num.parse::<i32>().expect("Must be numerical!")).collect();
                        sum += nums[0] * nums[1];
                    }
                    break;
                }
            }
        }
        println!("Part 2: The sum is: {}", sum);
    }
    fn find_sum(re: Regex) -> () {
        let data_address = "data/day3.txt";
        let lines = read_lines(&data_address).unwrap();
        let mut sum = 0;
        for line in lines {
            let line = line.unwrap();
            let instructions: Vec<&str> = re.find_iter(&line).map(|m| m.as_str()).collect();
            for instruction in instructions {
                let nums: Vec<i32> = instruction[4..instruction.len() - 1].split(",").map(|num| num.parse::<i32>().expect("Must be numerical!")).collect();
                sum += nums[0] * nums[1];
            }
        }
        println!("Part 1: The sum is: {}", sum);
    }
}