use std::path::{Component, Path};

struct Dimensions(usize, usize, usize);

impl Dimensions {
    fn required_paper(&self) -> usize {
        let area = (self.0 * self.1, self.0 * self.2, self.1 * self.2);
        let extra = area.0.min(area.1.min(area.2));
        (area.0 + area.1 + area.2) * 2 + extra
    }
    fn volume(&self) -> usize {
        self.0 * self.1 * self.2
    }
    fn smallest_perimeter(&self) -> usize {
        let mut ordered_sides = [self.0, self.1, self.2];
        ordered_sides.sort_unstable();
        (ordered_sides[0] + ordered_sides[1]) * 2
    }
    fn ribbon_length(&self) -> usize {
        self.smallest_perimeter() + self.volume()
    }
}
fn parse(s: &str) -> Dimensions {
    let (a, rest) = s.split_once('x').unwrap();
    let (b, c) = rest.split_once('x').unwrap();
    Dimensions(a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap(), c.parse::<usize>().unwrap())
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> usize {
        input.lines().map(|s| parse(s).required_paper()).sum()
    }

    #[test]
    fn test_run() {
        assert_eq!(58, run("2x3x4"));
        assert_eq!(43, run("1x1x10"));
        assert_eq!(58 + 43, run("2x3x4\n1x1x10"));
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> usize {
        input.lines().map(|s| parse(s).ribbon_length()).sum()
    }

    #[test]
    fn test_run() {
        assert_eq!(34, run("2x3x4"));
        assert_eq!(14, run("1x1x10"));
        assert_eq!(34 + 14, run("2x3x4\n1x1x10"));
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
    assert_eq!(part1_ans, 1606483);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 3842356);
}
