mod day1;
mod day2;
mod day3;
mod utils;

fn main() {
    let day = 3;
    if day == 1 {
        use crate::day1::solution;
        solution::total_distance();
        solution::similarity_score();
    }
    if day == 2 {
        use crate::day2::solution;
        solution::count_safe_reports();
        solution::count_dampened_reports();
    }
    if day == 3 {
        use crate::day3::solution;
        solution::part1();
        solution::part2();
    }
}