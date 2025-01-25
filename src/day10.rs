pub mod solution {
    const DATA_ADDRESS: &str = "data/day10.txt";
    const TEST_ADDRESS: &str = "data/day10test.txt";

    use crate::utils::read_lines;
    pub fn part1() {
        for path in [TEST_ADDRESS, DATA_ADDRESS] {
            let lines = read_lines(path).expect("Couldn't read file!");
            let mut grid: Vec<Vec<u8>> = Vec::new();
            let mut i = 0;
            for line in lines {
                grid.push(Vec::new());
                for byte in line.unwrap().bytes() {
                    grid[i].push(byte - b'0');
                }
                i += 1;
            }
            let mut score = 0;
            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    if grid[i][j] == 0 {
                        score += calc_score(&grid, 0, &mut Vec::new(), (i, j));
                    }
                }
            }
            println!("Day 10 Part 1: {}", score);
        }
    }
    fn calc_score(grid: &Vec<Vec<u8>>, h: u8, mut peaks: &mut Vec<(usize, usize)>, start: (usize, usize)) -> i32 {
        let mut score = 0;
        if grid[start.0][start.1] == h {
            if h == 9 {
                if !peaks.contains(&start) {
                    peaks.push(start);
                    return 1;
                }
                else {
                    return 0;
                }
            }
            if start.0 > 0 {
                score += calc_score(grid, h + 1, &mut peaks, (start.0 - 1, start.1));
            }
            if start.0 < grid.len() - 1 {
                score += calc_score(grid, h + 1, &mut peaks, (start.0 + 1, start.1));
            }
            if start.1 > 0 {
                score += calc_score(grid, h + 1, &mut peaks, (start.0, start.1 - 1));
            }
            if start.1 < grid[start.0].len() - 1 {
                score += calc_score(grid, h + 1, &mut peaks, (start.0, start.1 + 1));
            }
            return score;
        }
        0
    }
    pub fn part2() {
        for path in [TEST_ADDRESS, DATA_ADDRESS] {
            let lines = read_lines(path).expect("Couldn't read file!");
            let mut grid: Vec<Vec<u8>> = Vec::new();
            let mut i = 0;
            for line in lines {
                grid.push(Vec::new());
                for byte in line.unwrap().bytes() {
                    grid[i].push(byte - b'0');
                }
                i += 1;
            }
            let mut score = 0;
            for i in 0..grid.len() {
                for j in 0..grid[i].len() {
                    if grid[i][j] == 0 {
                        score += calc_rating(&grid, 0, (i, j));
                    }
                }
            }
            println!("Day 10 Part 2: {}", score);
        }
    }
    fn calc_rating(grid: &Vec<Vec<u8>>, h: u8, start: (usize, usize)) -> i32 {
        let mut score = 0;
        if grid[start.0][start.1] == h {
            if h == 9 {
                return 1;
            }
            if start.0 > 0 {
                score += calc_rating(grid, h + 1, (start.0 - 1, start.1));
            }
            if start.0 < grid.len() - 1 {
                score += calc_rating(grid, h + 1, (start.0 + 1, start.1));
            }
            if start.1 > 0 {
                score += calc_rating(grid, h + 1, (start.0, start.1 - 1));
            }
            if start.1 < grid[start.0].len() - 1 {
                score += calc_rating(grid, h + 1, (start.0, start.1 + 1));
            }
            return score;
        }
        0
    }
}