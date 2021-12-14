use std::path::{Component, Path};

mod part1 {
    pub fn run(input: &str) -> i64 {
        input.chars().into_iter().fold(0, |floor, c| match c {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => floor,
        })
    }

    #[test]
    fn test_run() {
        assert_eq!(0, run("(())"));
        assert_eq!(run("()()"), 0);
        assert_eq!(run("((("), 3);
        assert_eq!(run("(()(()("), 3);
        assert_eq!(run("))((((("), 3);
        assert_eq!(run("())"), -1);
        assert_eq!(run("))("), -1);
        assert_eq!(run(")))"), -3);
        assert_eq!(run(")())())"), -3);
    }
}

mod part2 {
    pub fn run(input: &str) -> usize {
        input
            .chars()
            .into_iter()
            .scan(0, |floor, c| {
                *floor += match c {
                    '(' => 1,
                    ')' => -1,
                    _ => 0,
                };
                Some(*floor)
            })
            .enumerate()
            .find_map(|(pos, floor)| if floor < 0 { Some(pos + 1) } else { None })
            .unwrap()
    }

    #[test]
    fn test_run() {
        assert_eq!(1, run(")"));
        assert_eq!(5, run("()())"));
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
    println!("Day {} part 1 - {} - took {} milliseconds.", day_number, part1_ans, now.elapsed().as_millis());
    assert_eq!(part1_ans, 74);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 1);
}
