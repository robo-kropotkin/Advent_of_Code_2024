pub mod solution {
    use crate::utils::read_lines;
    pub fn part1() {
        let data_address = "data/day6.txt";
        let lines = read_lines(&data_address).expect("Couldn't read file!");
        let mut obstacles: [[bool; 131]; 130] = [[false;131]; 130];
        let mut visited: [[bool; 131]; 130] = [[false; 131]; 130];
        let mut i = 0;
        let mut pos: [i32; 2] = [0; 2];
        let mut dir: i32 = 0;
        let width = 131;
        let height = 130;
        for line in lines {
            let mut j = 0;
            for byte in line.unwrap().bytes() {
                if byte == b'#' {
                    obstacles[i][j] = true;
                }
                if byte == b'^' {
                    pos[0] = i as i32;
                    pos[1] = j as i32;
                }
                j += 1;
            }
            i += 1;
        }
        let mut numvisited = 0;
        let mut idelta = -1;
        let mut jdelta = 0;
        while !out_of_bounds(width, height, pos[0], pos[1]) {
            while !out_of_bounds(width, height, pos[0] + idelta, pos[1] + jdelta) &&
                obstacles[(pos[0] + idelta) as usize][(pos[1] + jdelta) as usize] {
                    dir = (dir + 1) % 4;
                    idelta = ((dir + 1) % 2) * (dir - 1);
                    jdelta = (dir % 2) * (dir - 2) * -1;
            }   
            if !visited[pos[0] as usize][pos[1] as usize] {
                visited[pos[0] as usize][pos[1] as usize] = true;
                numvisited += 1;
            }
            pos[0] += idelta;
            pos[1] += jdelta;
            println!("{},{}", pos[0], pos[1])
        }
        println!("Day 6, part 1: {}", numvisited);
    }
    fn out_of_bounds(width: i32, height: i32, i: i32, j: i32) -> bool {
        i < 0 || i >= height || j < 0 || j >= width
    }
}