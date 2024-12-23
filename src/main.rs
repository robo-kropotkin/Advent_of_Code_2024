mod day1;
mod day2;
mod utils;

fn main() {
    let day = 2;
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
}