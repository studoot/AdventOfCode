#![feature(iter_zip)]
use std::iter::zip;

pub fn parse(input: &str) -> Vec<usize> {
    input.lines().flat_map(|s| s.parse::<usize>()).collect()
}

fn main() {
    let input_string = include_str!("../input.txt");
    let measurements = parse(input_string);

    {
        let count_larger = zip(&measurements, &measurements[1..])
            .map(|(prev, current)| if current > prev { 1 } else { 0 })
            .sum::<i32>();
        println!("Day  1 part 1 - depth increase count = {}", count_larger);
    }
    {
        let count_larger = zip(
            zip(&measurements, &measurements[1..]),
            zip(&measurements[2..], &measurements[3..]),
        )
        .map(|((a, b), (c, d))| {
            let prev = a + b + c;
            let current = b + c + d;

            if current > prev {
                1
            } else {
                0
            }
        })
        .sum::<i32>();
        println!(
            "Day  1 part 2 - rolling average depth increase count = {}",
            count_larger
        );
    }
}
