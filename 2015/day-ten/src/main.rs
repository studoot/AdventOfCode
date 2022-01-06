use std::path::{Component, Path};

fn iterate(s: &str) -> String {
    let mut count = 0;
    let mut last = None;
    let mut output = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        if last == Some(c) {
            count += 1;
        } else {
            if let Some(last_c) = last {
                output += &format!("{}{}", count, last_c);
            }
            count = 1;
            last = Some(c)
        }
    }
    if let Some(last_c) = last {
        output += &format!("{}{}", count, last_c);
    }
    output
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> usize {
        (0..40).fold(input.to_owned(), |acc, _| iterate(&acc)).len()
    }

    #[test]
    fn test_run() {
        assert_eq!(iterate("1"), "11"); // 1 becomes 11 (1 copy of digit 1).
        assert_eq!(iterate("11"), "21"); // 11 becomes 21 (2 copies of digit 1).
        assert_eq!(iterate("21"), "1211"); // 21 becomes 1211 (one 2 followed by one 1).
        assert_eq!(iterate("1211"), "111221"); // 1211 becomes 111221 (one 1, one 2, and two 1s).
        assert_eq!(iterate("111221"), "312211"); // 111221 becomes 312211 (three 1s, two 2s, and one 1).
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> usize {
        (0..50).fold(input.to_owned(), |acc, _| iterate(&acc)).len()
    }

    #[test]
    fn test_run() {}
}

fn main() {
    let input_string = "1113122113".to_owned();
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
    let part1_ans = part1::run(&input_string);
    println!("Day {} part 1 - {} - took {} milliseconds.", day_number, part1_ans, now.elapsed().as_millis());
    assert_eq!(part1_ans, 360154);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(&input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 5103798);
}
