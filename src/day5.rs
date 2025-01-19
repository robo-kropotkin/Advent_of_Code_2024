pub mod solution {
    use crate::utils::read_lines;
    pub fn part1() {
        let data_address = "data/day5.txt";
        let mut lines = read_lines(data_address).expect("Cannot read file");
        let rules = get_rules(&mut lines);
        println!("Day 5, Part 1: {}", sum_valid(&rules, lines));
    }
    fn sum_valid(rules: &[Vec<i32>; 99], lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> i32 {
        let mut valid_sum = 0;

        for line in lines {
            let parts: Vec<i32> = line.unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let mut valid: bool = true;
            for i in 0..parts.len() - 1 {
                for j in i + 1..parts.len() {
                    if rules[parts[j] as usize].contains(&parts[i]) {
                        valid = false;
                        break;
                    }
                }
            }
            if valid {
                valid_sum += parts[parts.len() / 2];
            }
        }
        valid_sum
    }
    fn get_rules(lines: &mut std::io::Lines<std::io::BufReader<std::fs::File>>) -> [Vec<i32>; 99] {
        let mut rules: [Vec<i32>; 99] = std::array::from_fn(|_| Vec::new());
        let mut line: String = lines.next().unwrap().unwrap();
        while line != "" {
            let mut parts = line.split("|");
            let preceeding = parts.next().unwrap().parse::<i32>().unwrap() as usize;
            let succeeding = parts.next().unwrap().parse::<i32>().unwrap();
            rules[preceeding].push(succeeding);
            line = lines.next().unwrap().unwrap();
        }
        rules
    }
    pub fn part2() {
        let data_address = "data/day5.txt";
        let mut lines = read_lines(data_address).expect("Cannot read file");
        let rules = get_rules(&mut lines);

        println!("Day 5, Part 2: {}", sum_corrected(&rules, lines));
    }

    fn sum_corrected(rules: &[Vec<i32>; 99], lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> i32 {
        let mut corrected_sum = 0;
        for line in lines {
            let mut parts: Vec<i32> = line.unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            for i in 0..parts.len() {
                let mut j = i + 1;
                while j < parts.len() {
                    if rules[parts[j] as usize].contains(&parts[i]) {
                        let temp = parts[i];
                        parts[i] = parts[j];
                        parts[j] = temp;
                        j = i + 1;
                    }
                    else {
                        j += 1;
                    }
                }
            }
            corrected_sum += parts[parts.len() / 2];
        }
        corrected_sum
    }
}