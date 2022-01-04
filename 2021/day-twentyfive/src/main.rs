use itertools::Itertools;
use std::collections::HashSet;
use std::path::{Component, Path};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: u8,
    y: u8,
}
#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    East,
    South,
}
#[derive(Debug, PartialEq, Eq)]
struct Cucumbers {
    east_cucumbers: HashSet<Pos>,
    south_cucumbers: HashSet<Pos>,
    width: u8,
    height: u8,
}

impl Cucumbers {
    fn iterate(&mut self) -> bool {
        let mut some_moved = false;
        let new_east_cucumbers = self
            .east_cucumbers
            .iter()
            .map(|cucumber| {
                let new_pos = Pos { x: (cucumber.x + 1) % self.width, y: cucumber.y };
                if self.east_cucumbers.contains(&new_pos) || self.south_cucumbers.contains(&new_pos) {
                    *cucumber
                } else {
                    some_moved = true;
                    new_pos
                }
            })
            .collect::<HashSet<_>>();
        let new_south_cucumbers = self
            .south_cucumbers
            .iter()
            .map(|cucumber| {
                let new_pos = Pos { x: cucumber.x, y: (cucumber.y + 1) % self.height };
                if new_east_cucumbers.contains(&new_pos) || self.south_cucumbers.contains(&new_pos) {
                    *cucumber
                } else {
                    some_moved = true;
                    new_pos
                }
            })
            .collect::<HashSet<_>>();

        self.east_cucumbers = new_east_cucumbers;
        self.south_cucumbers = new_south_cucumbers;

        !some_moved
    }

    #[allow(dead_code, unstable_name_collisions)]
    fn print(&self) -> String {
        let mut lines = vec![vec!['.'; self.width as usize]; self.height as usize];
        for c in &self.east_cucumbers {
            lines[c.y as usize][c.x as usize] = '>';
        }
        for c in &self.south_cucumbers {
            lines[c.y as usize][c.x as usize] = 'v';
        }
        lines
            .into_iter()
            .map(|l| l.into_iter().collect::<String>())
            .intersperse("\n".to_owned())
            .collect::<String>()
    }
}

fn parse(input: &str) -> Cucumbers {
    let all_cucumbers = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.trim()
                .chars()
                .enumerate()
                .filter_map(|(col, c)| match c {
                    'v' | '>' => Some((
                        Pos { x: col as u8, y: row as u8 },
                        if c == '>' { Direction::East } else { Direction::South },
                    )),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let east_cucumbers = all_cucumbers
        .iter()
        .filter_map(|(pos, dirn)| if *dirn == Direction::East { Some(*pos) } else { None })
        .collect::<HashSet<_>>();
    let south_cucumbers = all_cucumbers
        .iter()
        .filter_map(|(pos, dirn)| if *dirn == Direction::South { Some(*pos) } else { None })
        .collect::<HashSet<_>>();
    Cucumbers {
        east_cucumbers,
        south_cucumbers,
        width: input.lines().next().unwrap().chars().count() as u8,
        height: input.lines().count() as u8,
    }
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let mut cucumbers = parse(input);
        let mut iters = 0;
        loop {
            iters += 1;
            if cucumbers.iterate() {
                return iters;
            }
        }
    }

    #[test]
    fn test_run() {
        let input_string = "v...>>.vv>
                                 .vv>>.vv..
                                 >>.>v>...v
                                 >>v>>.>.v.
                                 v>v.vv.v..
                                 >.>>..v...
                                 .vv..>.>v.
                                 v.v..>>v.v
                                 ....v..v.>";
        assert_eq!(58, run(input_string))
    }

    #[test]
    fn test_simple1() {
        let mut cs = parse("...>>>>>...");

        assert_eq!(cs.iterate(), false);
        assert_eq!(cs.print(), "...>>>>.>..");
        assert_eq!(cs.iterate(), false);
        assert_eq!(cs.print(), "...>>>.>.>.");
    }

    #[test]
    fn test_simple2() {
        let mut cs = parse(
            "..........
                   .>v....v..
                   .......>..
                   ..........",
        );
        assert_eq!(cs.iterate(), false);
        assert_eq!(
            cs.print(),
            "..........\n\
             .>........\n\
             ..v....v>.\n\
             .........."
        );
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
    assert_eq!(part1_ans, 329);
}
