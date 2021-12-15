use std::collections::HashSet;
use std::path::{Component, Path};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Point(i64, i64);

impl Point {
    fn next_pos(&self, c: char) -> Point {
        match c {
            '^' => Point(self.0, self.1 + 1),
            'v' => Point(self.0, self.1 - 1),
            '<' => Point(self.0 - 1, self.1),
            '>' => Point(self.0 + 1, self.1),
            _ => *self,
        }
    }
}
mod part1 {
    use super::*;

    pub fn run(input: &str) -> usize {
        input
            .chars()
            .scan(Point(0, 0), |pos, c| {
                *pos = pos.next_pos(c);
                Some(*pos)
            })
            .chain([Point(0, 0)].into_iter())
            .collect::<HashSet<_>>()
            .len()
    }

    #[test]
    fn test_run() {
        assert_eq!(2, run(">"));
        assert_eq!(4, run("^>v<"));
        assert_eq!(2, run("^v^v^v^v^v"));
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> usize {
        input
            .chars()
            .scan((Point(0, 0), Point(0, 0), false), |(p1, p2, is_first), c| {
                *is_first = !*is_first;
                if *is_first {
                    *p1 = p1.next_pos(c);
                    Some(*p1)
                } else {
                    *p2 = p2.next_pos(c);
                    Some(*p2)
                }
            })
            .chain([Point(0, 0)].into_iter())
            .collect::<HashSet<_>>()
            .len()
    }

    #[test]
    fn test_run() {
        assert_eq!(3, run("^v"));
        assert_eq!(3, run("^>v<"));
        assert_eq!(11, run("^v^v^v^v^v"));
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
    assert_eq!(part1_ans, 2081);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 2341);
}
