use std::env;
use std::fmt::Display;

use aoc2022::*;

fn time<T: Display, U: Display>(day_number: usize, f: &dyn Fn() -> Option<(T, bool, U, bool)>) -> u128 {
    let now = std::time::Instant::now();
    let output = f();
    let duration = now.elapsed().as_micros();
    if let Some((part1_answer, part1_good, part2_answer, part2_good)) = output {
        println!("Day {day_number}, part 1 = {part1_answer} [{part1_good}], part 2 = {part2_answer} [{part2_good}] - took {duration} microseconds");
        duration
    } else {
        println!("Day {day_number} not yet implemented");
        0
    }
}

fn dispatch_day(day_number: usize) -> u128 {
    let oob = || None;
    match day_number {
        1 => time(day_number, &day01::run),
        2 => time(day_number, &day02::run),
        3 => time(day_number, &day03::run),
        4 => time(day_number, &day04::run),
        5 => time(day_number, &day05::run),
        6 => time(day_number, &day06::run),
        7 => time(day_number, &day07::run),
        8 => time(day_number, &day08::run),
        9 => time(day_number, &day09::run),
        10 => time(day_number, &day10::run),
        11 => time(day_number, &day11::run),
        12 => time(day_number, &day12::run),
        13 => time(day_number, &day13::run),
        14 => time(day_number, &day14::run),
        15 => time(day_number, &day15::run),
        16 => time(day_number, &day16::run),
        17 => time(day_number, &day17::run),
        18 => time(day_number, &day18::run),
        19 => time(day_number, &day19::run),
        20 => time(day_number, &day20::run),
        21 => time(day_number, &day21::run),
        22 => time(day_number, &day22::run),
        // 23 => time(day_number, &day23::run),
        // 24 => time(day_number, &day24::run),
        // 25 => time(day_number, &day25::run),
        _ => time::<bool, bool>(day_number, &oob),
    }
}

fn main() {
    if let Some(day_string) = env::args().nth(1) {
        dispatch_day(day_string.parse::<usize>().unwrap());
    } else {
        let total_time = (1..=25).map(dispatch_day).sum::<u128>();
        println!("Took {total_time} microseconds");
    }
}
