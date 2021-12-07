use std::cmp::{max, min};
use std::iter::Iterator;
use std::str::FromStr;

fn parse(input: &str) -> Vec<usize> {
    let mut v = input.lines().next().unwrap().split(',').map(|s| usize::from_str(s).unwrap()).collect::<Vec<_>>();
    v.sort();
    v
}

fn fuel_required(to_pos: usize, crabs: &Vec<usize>, fuel_for_distance:fn(usize)->usize) -> usize {
    crabs.iter().fold(0, |acc, crab| {
        let distance = max(*crab, to_pos) - min(*crab, to_pos);
        acc + fuel_for_distance(distance)
    })
}

fn fuel_required_for_all_crabs(crabs: &Vec<usize>, fuel_for_distance:fn(usize)->usize) -> usize {
    (crabs[0]..=crabs[crabs.len() - 1]).map(|pos| fuel_required(pos, &crabs, fuel_for_distance)).min().expect("No positions?")
}

mod part1 {
    use super::*;
    fn fuel_required_for_distance(distance: usize) -> usize {
        distance
    }

    pub fn run(input: &str) -> usize {
        fuel_required_for_all_crabs(&parse(input), fuel_required_for_distance)
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(37, run(input_string))
    }
}

mod part2 {
    use super::*;

    fn fuel_required_for_distance(distance: usize) -> usize {
        // "...crab submarine engines don't burn fuel at a constant rate. Instead,
        // each change of 1 step in horizontal position costs 1 more unit of
        // fuel than the last: the first step costs 1, the second step costs 2,
        // the third step costs 3, and so on."
        //
        // This makes the amount of fuel required for a distance `d` equal to
        // `(d * (d+1))/2` (sum of integers up to & including `d`).
        (distance * (distance + 1)) / 2
    }
    pub fn run(input: &str) -> usize {
        fuel_required_for_all_crabs(&parse(input), fuel_required_for_distance)
    }
    #[test]
    fn test_fuel_required() {
        let input_string = include_str!("../test.txt");
        let crabs = parse(input_string);
        assert_eq!(206, fuel_required(2, &crabs, fuel_required_for_distance));
    }
    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(168, run(input_string))
    }
}
fn main() {
    let input_string = include_str!("../input.txt");
    let part1_ans = part1::run(input_string);
    println!("Day  7 part 1 - {}", part1_ans);
    assert_eq!(part1_ans, 348996);
    let part2_ans = part2::run(input_string);
    println!("Day  7 part 2 - {}", part2_ans);
    assert_eq!(part2_ans, 98231647);
}
