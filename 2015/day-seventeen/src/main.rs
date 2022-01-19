use itertools::Itertools;
use std::path::{Component, Path};

mod part1 {
    use super::*;

    pub fn run(input: &[usize], volume: usize) -> usize {
        input
            .iter()
            .powerset()
            .map(|v| v.iter().fold(0usize, |acc, i| acc + **i))
            .filter(|a| *a == volume)
            .count()
    }

    #[test]
    fn test_run() {
        let input = &[20, 15, 10, 5, 5];
        assert_eq!(4, run(input, 25))
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &[usize], volume: usize) -> usize {
        let solutions = input
            .iter()
            .powerset()
            .map(|v| (v.len(), v.iter().fold(0usize, |acc, i| acc + **i)))
            .filter_map(|(count, vol)| if vol == volume { Some(count) } else { None })
            .collect::<Vec<_>>();
        let min_container_count = solutions.iter().min().unwrap();
        solutions
            .iter()
            .filter(|c| *c == min_container_count)
            .count()
    }

    #[test]
    fn test_run() {
        let input = &[20, 15, 10, 5, 5];
        assert_eq!(3, run(input, 25))
    }
}

fn main() {
    let input = [50, 44, 11, 49, 42, 46, 18, 32, 26, 40, 21, 7, 18, 43, 10, 47, 36, 24, 22, 40];
    let day_number = Path::new(file!())
        .components()
        .find_map(|bit| {
            if let Component::Normal(os_name) = bit {
                if let Some(dir_name) = os_name.to_str() {
                    return dir_name.strip_prefix("day-");
                }
            };
            None
        })
        .unwrap()
        .to_lowercase()
        .replace("-", " ")
        .replace("_", " ");
    let now = std::time::Instant::now();
    let part1_ans = part1::run(&input, 150);
    println!("Day {} part 1 - {} - took {} milliseconds.", day_number, part1_ans, now.elapsed().as_millis());
    assert_eq!(part1_ans, 654);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(&input, 150);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 57);
}
