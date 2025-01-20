mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod utils;

fn main() {
    let day = 6;
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
    if day == 4 {
        use crate::day4::solution;
        solution::part1();
        solution::part2();
    }
    if day == 5 {
        use crate::day5::solution;
        solution::part1();
        solution::part2();
    }
    if day == 6 {
        use crate::day6::solution;
        solution::part1();
        solution::part2();
    }

}