pub mod solution {
    const DATA_ADDRESS: &str = "data/day7.txt";
    const TEST_DATA_ADDRESS: &str = "data/day7test.txt";
    use crate::utils::read_lines;
    pub fn part1() {
        for filename in [DATA_ADDRESS, TEST_DATA_ADDRESS] {
            let lines = read_lines(filename).expect("Couldn't read file!");
            let mut sum = 0;
            let mut ladd = 0;
            for line in lines {
                let line = line.expect("Couldn't read line!");
                let parts = line.split(": ").collect::<Vec<&str>>();
                let result: u64 = parts[0].parse().expect("Result is not a number!");
                let elements: Vec<u64> = parts[1].split(" ").map(|x| x.trim().parse().expect("Couldn't parse number!")).collect();
                if brute_force_check(result, elements) {
                    sum += result;
                    ladd += 1;
                }
            }
            println!("Day 7 Part 1: {} lines summing up to {}", ladd, sum);
        }
    }
    fn brute_force_check(result: u64, elements: Vec<u64>) -> bool {
        if elements.len() == 1 {
            return elements[0] == result;
        }
        if result < elements[elements.len() - 1] {
            return false;
        }
        let add = brute_force_check(result - elements[elements.len() - 1], elements[0..elements.len()-1].to_vec());
        let mut mul = brute_force_check(result / elements[elements.len() - 1], elements[0..elements.len()-1].to_vec());
        mul &= result % elements[elements.len() - 1] == 0;
        add || mul
    }
    pub fn part2() {
        for filename in [DATA_ADDRESS, TEST_DATA_ADDRESS] {
            let lines = read_lines(filename).expect("Couldn't read file!");
            let mut sum = 0;
            let mut ladd = 0;
            for line in lines {
                let line = line.expect("Couldn't read line!");
                let parts = line.split(": ").collect::<Vec<&str>>();
                let result: u64 = parts[0].parse().expect("Result is not a number!");
                let elements: Vec<u64> = parts[1].split(" ").map(|x| x.trim().parse().expect("Couldn't parse number!")).collect();
                if brute_force_check_3(result, elements) {
                    sum += result;
                    ladd += 1;
                }
            }
            println!("Day 7 Part 2: {} lines summing up to {}", ladd, sum);
        }
    }
    fn brute_force_check_3(result: u64, elements: Vec<u64>) -> bool {
        if elements.len() == 1 {
            return elements[0] == result;
        }
        if result < elements[elements.len() - 1] {
            return false;
        }
        let last = *elements.last().unwrap();
        let factor = 10_f64.powf((last as f64).log10().floor() + 1_f64) as u64;
        let add = brute_force_check_3(result - last, elements[0..elements.len()-1].to_vec());
        let mut mul = result % last == 0;
        if mul {
            mul = brute_force_check_3(result / last, elements[0..elements.len()-1].to_vec());
        }
        let mut conc = result % factor == last;
        if conc {
            conc = brute_force_check_3(result / factor as u64, elements[0..elements.len()-1].to_vec());
        }
        add || mul || conc
    }
}