pub mod solution {
    use std::io::BufReader;
    use std::fs::File;
    pub fn part1() {
        use crate::utils::read_lines;

        let data_address = "data/day4.txt";
        let lines = read_lines(data_address);
        match lines {
            Ok(lines) => {
                print_xcount(lines);
            }
            Err(e) => {
                println!("Error reading lines! {}", e);
            }
        }

    }
    pub fn part2() {
        use crate::utils::read_lines;

        let data_address = "data/day4.txt";
        let lines = read_lines(data_address);
        match lines {
            Ok(lines) => {
                print_mascount(lines);
            }
            Err(e) => {
                println!("Error reading lines! {}", e);
            }
        }
    }
    fn print_xcount(lines: std::io::Lines<BufReader<File>>) {
        let mut xcount = 0;
        let mat : Vec<Vec<char>> = lines.map(|line| line.unwrap().chars().collect()).collect();
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 'X' {
                    for delta in [(0, 1), (1, 0), (1, 1), (-1, 0), (0, -1), (-1, -1), (1, -1), (-1, 1)].iter() {
                        let (di, dj) = *delta;
                        xcount += rec_check(&mat, 0, di, dj, i as i32, j as i32);
                    }
                }
            }
        }
        println!("{}", xcount);
    }
    fn rec_check(mat: &Vec<Vec<char>>, n: i32, di: i32, dj: i32, i: i32, j: i32) -> i32 {
        if i + di < 0 || i + di >= mat.len() as i32 || j + dj < 0 || j + dj >= mat[0].len() as i32 {
            return 0;
        }
        match n {
            0 => {
                if mat[(i + di) as usize][(j + dj) as usize] == 'M' {
                    return rec_check(mat, 1, di, dj, i + di, j + dj);
                }
                else {
                    return 0;
                }
            }
            1 => {
                if mat[(i + di) as usize][(j + dj) as usize] == 'A' {
                    return rec_check(mat, 2, di, dj, i + di, j + dj);
                }
                else {
                    return 0;
                }
            }
            2 => {
                if mat[(i + di) as usize][(j + dj) as usize] == 'S' {
                    return 1;
                }
                else {
                    return 0;
                }
            }
            d => {
                println!("Error, n = {}", d);
                return 0;
            }
        }
    }
    fn print_mascount(lines: std::io::Lines<BufReader<File>>) {
        let ms = 'M' as i32 + 'S' as i32;
        let mut mascount = 0;
        let mat : Vec<Vec<char>> = lines.map(|line| line.unwrap().chars().collect()).collect();
        for i in 1..mat.len() - 1 {
            for j in 1..mat[0].len() - 1 {
                if mat[i][j] == 'A' {
                    if mat[i-1][j-1] as i32 + mat[i+1][j+1] as i32 == ms && mat[i-1][j+1] as i32 + mat[i+1][j-1] as i32 == ms{
                        mascount += 1;
                    }
                }
            }
        }
        println!("{}", mascount);
    }
}