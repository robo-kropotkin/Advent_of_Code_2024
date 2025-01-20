pub mod solution {
    const HEIGHT: usize = 130;
    const HEIGHT32: i32 = 130;
    const WIDTH: usize = 131;
    const WIDTH32: i32 = 131;
    const DATA_ADDRESS: &str = "data/day6.txt";
    // const DATA_ADDRESS: &str = "data/day6test.txt";

    use crate::utils::read_lines;
    pub fn part1() {
        let (obstacles, mut pos) = get_obstacle_map();
        let mut visited: [[bool; 131]; 130] = [[false; 131]; 130];
        let mut numvisited = 0;
        let mut dir: i32 = 0;
        let (mut idelta, mut jdelta): (i32, i32) = dir_to_delta(dir);
        while !out_of_bounds(pos[0], pos[1]) {
            while !out_of_bounds(pos[0] + idelta, pos[1] + jdelta) &&
                obstacles[(pos[0] + idelta) as usize][(pos[1] + jdelta) as usize] {
                    dir = (dir + 1) % 4;
                    (idelta, jdelta) = dir_to_delta(dir);
            }   
            if !visited[pos[0] as usize][pos[1] as usize] {
                visited[pos[0] as usize][pos[1] as usize] = true;
                numvisited += 1;
            }
            pos[0] += idelta;
            pos[1] += jdelta;
        }
        println!("Day 6, part 1: {}", numvisited);
    }
    pub fn part2() {
        let (mut obstacles, start) = get_obstacle_map();
        let mut num_loops = 0;
        let mut dir: i32;
        let mut visited: [[u8; WIDTH]; HEIGHT];
        let (mut idelta, mut jdelta): (i32, i32);
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if (i == start[0] as usize && j == start[1] as usize) || obstacles[i][j] {
                    continue;
                }
                obstacles[i][j] = true;
                let mut pos = start;
                dir = 0;
                (idelta, jdelta) = dir_to_delta(dir);
                visited = [[0; WIDTH]; HEIGHT];
                loop {
                    while !out_of_bounds(pos[0] + idelta, pos[1] + jdelta) &&
                                            obstacles[(pos[0] + idelta) as usize][(pos[1] + jdelta) as usize] {
                        dir = (dir + 1) % 4;
                        (idelta, jdelta) = dir_to_delta(dir);
                    }
                    if visited[pos[0] as usize][pos[1] as usize] & (1 << dir) != 0 {
                        num_loops += 1;
                        break;
                    }
                    visited[pos[0] as usize][pos[1] as usize] |= 1 << dir;
                    pos[0] += idelta;
                    pos[1] += jdelta;
                    if out_of_bounds(pos[0], pos[1]) {
                        break;
                    }
                }
                obstacles[i][j] = false;
            }
        }
        println!("Day 6, part 2: {}", num_loops);
    }
    fn dir_to_delta(dir: i32) -> (i32, i32) {
        let idelta = ((dir + 1) % 2) * (dir - 1);
        let jdelta = (dir % 2) * (dir - 2) * -1;
        (idelta, jdelta)
    }
    fn get_obstacle_map() -> ([[bool; WIDTH]; HEIGHT], [i32; 2]) {
        let lines = read_lines(&DATA_ADDRESS).expect("Couldn't read file!");
        let mut obstacles: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];
        let mut pos: [i32; 2] = [0; 2];
        let mut i = 0;
        for line in lines {
            let mut j = 0;
            for byte in line.unwrap().bytes() {
                if byte == b'#' {
                    obstacles[i as usize][j as usize] = true;
                }
                if byte == b'^' {
                    pos[0] = i;
                    pos[1] = j;
                }
                j += 1;
            }
            i += 1;
        }
        (obstacles, pos)
    }
    fn out_of_bounds(i: i32, j: i32) -> bool {
        i < 0 || i >= HEIGHT32 || j < 0 || j >= WIDTH32
    }
}