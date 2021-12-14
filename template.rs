use std::path::{Component, Path};

mod part1 {
    use super::*;

    pub fn run(input: &str) -> usize {
        0
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(1, run(input_string))
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> usize {
        0
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(1, run(input_string))
    }
}

fn main() {
    let input_string = include_str!("../input.txt");
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
    let part1_ans = part1::run(input_string);
    println!(
        "Day {} part 1 - {} - took {} milliseconds.",
        day_number,
        part1_ans,
        now.elapsed().as_millis()
    );
    assert_eq!(part1_ans, 1);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!(
        "Day {} part 2 - {} - took {} milliseconds.",
        day_number,
        part2_ans,
        now.elapsed().as_millis()
    );
    assert_eq!(part2_ans, 1);
}
