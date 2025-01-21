pub mod solution {
    const DATA_ADDRESS: &str = "data/day8.txt";
    const TEST_DATA_ADDRESS: &str = "data/day8test.txt";
    // const WIDTH: usize = 50;
    const WIDTH32: i32 = 50;
    const HEIGHT: usize = 50;
    const HEIGHT32: i32 = 50;
    use crate::utils::read_lines;
    use std::time::Instant;
    pub fn part1() {
        let start = Instant::now();
        for filepath in [DATA_ADDRESS, TEST_DATA_ADDRESS] {
            let lines = read_lines(filepath).expect("Couldn't read file!");
            let mut unique = 0;
            let mut anode_map: [u64; HEIGHT] = [0; HEIGHT];
            let mut antennas = find_antennas(lines);
            for (byte1, i1, j1) in antennas.clone().iter() {
                for (byte2, i2, j2) in antennas[1..].iter() {
                    let (i1, j1, i2, j2) = (*i1 as i32, *j1 as i32, *i2 as i32, *j2 as i32);
                    if byte1 == byte2 {
                        let (di, dj): (i32, i32) = (i2- i1, j2 - j1);
                        let anode1 = (i1 - di, j1 - dj);
                        let anode2 = (i2 + di, j2 + dj);
                        add_anode(&mut anode_map, &mut unique, anode1);
                        add_anode(&mut anode_map, &mut unique, anode2);
                    }
                }
                antennas.remove(0);
            }
            println!("Day 8 Part 1: {} in {} micros", unique, start.elapsed().as_micros());
        }
    }
    fn add_anode(anode_map: &mut [u64; HEIGHT], unique: &mut u64, anode: (i32, i32)) {
        if anode.0 >= 0 && anode.1 >= 0 && anode.0 < HEIGHT32 && anode.1 < WIDTH32 {
            if anode_map[anode.0 as usize] & (1 << anode.1) == 0 {
                anode_map[anode.0 as usize] |= 1 << anode.1;
                *unique += 1;
            }
        }
    }
    fn find_antennas(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> Vec<(u8, usize, usize)> {
        let mut antennas: Vec<(u8, usize, usize)> = Vec::new();
        let mut i = 0;
        for line in lines {
            let mut j = 0;
            for byte in line.unwrap().bytes() {
                if byte != b'.' {
                    antennas.push((byte, i, j));
                }
                j += 1;
            }
            i += 1;
        }
        antennas
    }
    pub fn part2() {
        for filepath in [DATA_ADDRESS, TEST_DATA_ADDRESS] {
            let start = Instant::now();
            let lines = read_lines(filepath).expect("Couldn't read file!");
            let mut unique = 0;
            let mut anode_map: [u64; HEIGHT] = [0; HEIGHT];
            let mut antennas = find_antennas(lines);
            for (byte1, i1, j1) in antennas.clone().iter() {
                for (byte2, i2, j2) in antennas[1..].iter() {
                    if byte1 == byte2 {
                        let (i1, j1, i2, j2) = (*i1 as i32, *j1 as i32, *i2 as i32, *j2 as i32);
                        let (di, dj): (i32, i32) = (i2- i1, j2 - j1);
                        let mut n = 1;
                        while i1 + n * di >= 0 && j1 + n * dj >= 0 && i1 + n * di < HEIGHT32 && j1 + n * dj < WIDTH32 {
                            let anode = (i1 + n * di, j1 + n * dj);
                            add_anode(&mut anode_map, &mut unique, anode);
                            n += 1;
                        }
                        n = 0;
                        while i1 + n * di >= 0 && j1 + n * dj >= 0 && i1 + n * di < HEIGHT32 && j1 + n * dj < WIDTH32 {
                            let anode = (i1 + n * di, j1 + n * dj);
                            add_anode(&mut anode_map, &mut unique, anode);
                            n -= 1;
                        }
                    }
                }
                antennas.remove(0);
            }
            println!("Day 8 Part 1: {} in {} micros", unique, start.elapsed().as_micros());
        }
    }
}