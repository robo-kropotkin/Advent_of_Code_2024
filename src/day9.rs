pub mod solution {
    use crate::utils::read_lines;
    const DATA_ADDRESS: &str = "data/day9.txt";
    const TEST_DATA_ADDRESS: &str = "data/day9test.txt";
    pub fn part1() {
        for filepath in [DATA_ADDRESS, TEST_DATA_ADDRESS] {
            let line = read_lines(filepath).expect("Couldn't read file!").next().unwrap().unwrap();
            let mut drive = line.bytes().map(|x| x - b'0');
            let mut evird = drive.clone().rev();
            let mut checksum: usize = 0;
            let mut pos = 0;
            let (mut lvalue, mut rvalue) = (drive.next().unwrap() as usize, evird.next().unwrap() as usize);
            let (mut lid, mut rid) = (0 as usize, drive.len() / 2);
            while lid < rid {
                // Add block from the left
                checksum += (2 * pos + lvalue - 1) * lid * lvalue / 2;
                lid += 1;
                pos += lvalue;
                // Read gap from the left
                lvalue = drive.next().unwrap() as usize;

                // Add block from the right
                // Block is smaller than gap
                while lvalue > rvalue {
                    checksum += (2 * pos + rvalue - 1) * rid * rvalue / 2;
                    rid -= 1;
                    pos += rvalue;
                    lvalue -= rvalue;
                    evird.next().unwrap() as usize;
                    rvalue = evird.next().unwrap() as usize;
                }

                // Block is larger than gap
                checksum += (2 * pos + lvalue - 1) * rid * lvalue / 2;
                rvalue -= lvalue;
                pos += lvalue;
                lvalue = drive.next().unwrap() as usize;
            }
            checksum += (2 * pos + rvalue - 1) * rid * rvalue / 2;
            println!("Day 9 Part 1: {}", checksum);   
        }
    }
    pub fn part2() {
        for filepath in [DATA_ADDRESS, TEST_DATA_ADDRESS] {
            let line = read_lines(filepath).expect("Couldn't read file!").next().unwrap().unwrap();
            let mut checksum = 0;
            let mut id = 0;
            let mut drive: Vec<(usize, usize)> = line.bytes().map(|x| {
                let val = (x - b'0') as usize;
                id += 1;
                (val, id / 2)
            }).collect();
            let mut i = drive.len() / 2;
            while i > 0 {
                let d = drive[2 * i];
                let n = d.0;
                let mut j = 0;
                while j < i {
                    if drive[2 * j + 1].0 >= n {
                        drive[2 * i].1 = 0;
                        drive[2 * j + 1].0 -= n;
                        drive.insert(2 * j + 1, d);
                        drive.insert(2 * j + 1, (0, 0));
                        i += 1;
                        break;
                    }
                    j += 1;
                }
                i -= 1;
            }
            let mut a = drive[0].0 + drive[1].0;
            for i in 1..drive.len() / 2 {
                checksum += arith_sum(a, 1, drive[2 * i].0) * drive[2 * i].1;
                a += drive[2 * i].0 + drive[2 * i + 1].0;
            }
            println!("Day 9 Part 2: {}", checksum);
        }
    }
    fn arith_sum(a: usize, d: usize, n: usize) -> usize {
        (2 * a + (n - 1) * d) * n / 2
    }
}