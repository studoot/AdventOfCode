use std::env;
use std::fmt::Display;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
// mod day6;
// mod day7;
// mod day8;
// mod day9;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
// mod day18;
// mod day19;
// mod day20;
// mod day21;
// mod day22;
// mod day23;
// mod day24;
// mod day25;

fn time<T: Display>(day_number: usize, f: &dyn Fn() -> Option<(T, bool, T, bool)>) {
    let now = std::time::Instant::now();
    let output = f();
    let duration = now.elapsed().as_micros();
    if let Some((part1_answer, part1_good, part2_answer, part2_good)) = output {
        println!("Day {day_number}, part 1 = {part1_answer} [{part1_good}], part 2 = {part2_answer} [{part2_good}] - took {duration} microseconds");
    } else {
        println!("Day {day_number} not yet implemented");
    }
}

fn dispatch_day(day_number: usize) {
    let oob = || None;
    match day_number {
        1 => time(day_number, &day1::run),
        2 => time(day_number, &day2::run),
        3 => time(day_number, &day3::run),
        4 => time(day_number, &day4::run),
        5 => time(day_number, &day5::run),
        // 6 => time(day_number, &day6::run),
        // 7 => time(day_number, &day7::run),
        // 8 => time(day_number, &day8::run),
        // 9 => time(day_number, &day9::run),
        // 10 => time(day_number, &day10::run),
        // 11 => time(day_number, &day11::run),
        // 12 => time(day_number, &day12::run),
        // 13 => time(day_number, &day13::run),
        // 14 => time(day_number, &day14::run),
        // 15 => time(day_number, &day15::run),
        // 16 => time(day_number, &day16::run),
        // 17 => time(day_number, &day17::run),
        // 18 => time(day_number, &day18::run),
        // 19 => time(day_number, &day19::run),
        // 20 => time(day_number, &day20::run),
        // 21 => time(day_number, &day21::run),
        // 22 => time(day_number, &day22::run),
        // 23 => time(day_number, &day23::run),
        // 24 => time(day_number, &day24::run),
        // 25 => time(day_number, &day25::run),
        _ => time::<bool>(day_number, &oob),
    }
}

fn main() {
    if let Some(day_string) = env::args().nth(1) {
        dispatch_day(day_string.parse::<usize>().unwrap());
    } else {
        for day_number in 1..=25 {
            dispatch_day(day_number);
        }
    }
}
