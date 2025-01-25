pub mod solution {
    const DATA_ADDRESS: &str = "data/day11.txt";
    const TEST_ADDRESS: &str = "data/day11test.txt";

    use crate::utils::read_lines;

    pub fn part1() {
        for path in [TEST_ADDRESS, DATA_ADDRESS] {
            let mut lines = read_lines(path).expect("Couldn't read file!");
            let line = lines.next().unwrap().unwrap();
            let mut line = line.split(" ").map(|x| x.parse().expect("a")).collect::<Vec<i64>>();
            for _i in 0..25 {
                for n in 0..line.len() {
                    let val = line[n];
                    if val == 0 {
                        line[n] = 1;
                    }
                    else if (val as f64).log10().floor() as i64 % 2 == 1 {
                        let num_digits = (val as f64).log10().floor() + 1.0; 
                        let factor = 10_f64.powf(num_digits / 2.0) as i64;
                        line.push(val % factor);
                        line[n] = val / factor;
                    } else {
                        line[n] *= 2024;
                    }
                }
            }
            println!("Day 11 Part 1: {}", line.len());
        }
    }
    pub fn part2() {
        for path in [TEST_ADDRESS, DATA_ADDRESS] {
            let mut lines = read_lines(path).expect("Couldn't read file!");
            let line = lines.next().unwrap().unwrap();
            let nums = line.split(" ").map(|x| x.parse().expect("a")).collect::<Vec<usize>>();
            let mut map: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
            for num in nums {
                map.insert(num, 1);
            }
            for _i in 0..75 {
                for (key, value) in map.clone().iter() {
                    if *key == 0 {
                        if !map.contains_key(&1) {
                            map.insert(1, *value);
                        }
                        else {
                            map.insert(1, map[&1] + *value);
                        }
                    } else if (*key as f64).log10().floor() as usize % 2 == 1 {
                        let num_digits = (*key as f64).log10().floor() + 1.0; 
                        let factor = 10_f64.powf(num_digits / 2.0) as usize;
                        if !map.contains_key(&(*key % factor)) {
                            map.insert(*key % factor, *value);
                        }
                        else {
                            map.insert(*key % factor, *value + map[&(*key % factor)]);
                        }
                        if !map.contains_key(&(*key / factor)) {
                            map.insert(*key / factor, *value);
                        }
                        else {
                            map.insert(*key / factor, *value + map[&(*key / factor)]);
                        }
                    } else {
                        if !map.contains_key(&(*key * 2024)) {
                            map.insert(*key * 2024, *value);
                        }
                        else {
                            map.insert(*key * 2024, *value + map[&(*key * 2024)]);
                        }
                    }
                    map.insert(*key, map[key] - *value);
                }
            }
            let mut stones = 0;
            for val in map.values() {
                stones += val;
            }
            println!("Day 11 Part 2 {}", stones);
        }
    }
}