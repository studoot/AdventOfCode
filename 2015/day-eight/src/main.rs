use std::path::{Component, Path};

fn parse(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|s| {
            let s = s.trim();
            let code_chars = s.len();
            let s = s.strip_prefix('"').unwrap();
            let s = s.strip_suffix('"').unwrap();
            let mut i = s.chars();
            let mut char_count = 0;
            while let Some(c) = i.next() {
                if c == '\\' {
                    match i.next().unwrap() {
                        '\\' | '"' => (),
                        'x' => {
                            i.next();
                            i.next();
                        }
                        c => panic!("Bad escape sequence \\{}", c),
                    }
                }
                char_count += 1;
            }

            (code_chars, char_count)
        })
        .collect::<Vec<_>>()
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let counts = parse(input);
        counts
            .into_iter()
            .fold(0, |acc, (code_chars, string_chars)| acc + (code_chars - string_chars))
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!((2 + 5 + 10 + 6) - (/* 0+ */3 + 7 + 1), run(input_string));
    }
}

mod part2 {
    use super::*;

    fn encoded_length(s: &str) -> usize {
        s.chars()
            .map(|c| match c {
                '\\' => 2,
                '"' => 2,
                _ => 1,
            })
            .sum::<usize>()
            + 2
    }

    pub fn run(input: &str) -> usize {
        let counts = parse(input);
        let code_chars = counts
            .into_iter()
            .fold(0, |acc, (code_chars, _)| acc + code_chars);
        let encoded_chars = input
            .lines()
            .map(|s| encoded_length(s.trim()))
            .sum::<usize>();
        encoded_chars - code_chars
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(19, run(input_string));
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
    assert_eq!(part1_ans, 1350);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 2085);
}
