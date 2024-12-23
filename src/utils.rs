use std::fs::File;
use std::io::{self, BufRead};
pub fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    println!("{filename}");
    let file = File::open(filename).expect("File not found!");
    Ok(io::BufReader::new(file).lines())
}