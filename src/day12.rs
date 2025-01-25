
pub mod solution {
    const DATA_ADDRESS: &str = "data/day12.txt";
    const TEST_ADDRESS: &str = "data/day12test.txt";
    use crate::utils::read_lines;

    pub fn part1() {
        for path in [DATA_ADDRESS, TEST_ADDRESS] {
            let _lines = read_lines(path).expect("Couldn't read file!");
            println!("Day 10 Part 1");
        }
    }
    pub fn part2() {
        for path in [DATA_ADDRESS, TEST_ADDRESS] {
            let _lines = read_lines(path).expect("Couldn't read file!");
            println!("Day 10 Part 2");
        }
    }
}