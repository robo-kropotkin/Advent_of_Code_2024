pub mod solution {
    use crate::utils::read_lines;
    pub fn count_safe_reports() {
        let data_address = "data/day2.txt";
        let lines = read_lines(&data_address).unwrap();
        let mut safe_lines = 0;
        'reports: for line in lines {
            let line = line.unwrap();
            let levels = line.split_whitespace().map(|num| num.parse::<i32>().expect("Must be numerical!"));
            let mut current = -1;
            let mut diff : Vec<i32> = Vec::new();
            for level in levels {
                if current != -1 {
                    diff.push(current - level);
                }
                current = level
            }
            if diff[0] < 0 {
                for d in diff {
                    if d < -3 || d > -1 {
                        continue 'reports;
                    }
                }
                safe_lines += 1;
            } 
            else if diff[0] > 0 {
                for d in diff {
                    if d > 3 || d < 1 {
                        continue 'reports;
                    }
                }
                safe_lines += 1;
            }
        }
        println!("{safe_lines} safe lines");
    }
    pub fn count_dampened_reports() {
        let lines = read_lines("data/day2.txt").unwrap();
        let mut safe_lines = 0;
        for line in lines {
            let mut line = line.unwrap().split(" ").map(|e| e.parse().unwrap()).collect();
            if safe_report(&line, 1) {
                safe_lines += 1;
            } else {
                line.reverse();
                safe_lines += safe_report(&line, 1) as i32;
            }
        }
        println!("{safe_lines}");
    }
    pub fn safe_report(report: &Vec<i32>, mistakes: i32) -> bool {
        for i in 0..report.len() - 1 {
            if 1 > report[i] - report[i + 1] || report[i] - report[i + 1] > 3 {
                if mistakes == 0 {
                    return false;
                }
                let mut lreport = report.to_vec();
                let mut rreport = report.to_vec();
                lreport.remove(i);
                rreport.remove(i + 1);
                if !safe_report(&lreport, mistakes - 1) && !safe_report(&rreport, mistakes - 1) {
                    return false;
                }
            }
        }
        true
    }
}