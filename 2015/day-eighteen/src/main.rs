use itertools::Itertools;
use std::path::{Component, Path};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Area {
    values: Vec<bool>,
    width: usize,
    height: usize,
}

impl Area {
    fn new(width: usize, height: usize) -> Area {
        Area { values: vec![false; width * height], width, height }
    }
    fn index(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }
    fn count_set(&self) -> usize {
        self.values.iter().filter(|b| **b).count()
    }
    fn count_set_neighbours(&self, x: usize, y: usize) -> usize {
        let x = x as isize;
        let y = y as isize;
        [x - 1, x, x + 1]
            .iter()
            .cartesian_product([y - 1, y, y + 1].iter())
            .filter(|(p_x, p_y)| {
                **p_x >= 0
                    && **p_x < self.width as isize
                    && **p_y >= 0
                    && **p_y < self.height as isize
                    && (**p_x, **p_y) != (x, y)
                    && self.get_at(**p_x as usize, **p_y as usize)
            })
            .count()
    }
    fn get_at(&self, x: usize, y: usize) -> bool {
        self.values[self.index(x as usize, y as usize)]
    }
    fn set_at(&mut self, x: usize, y: usize, v: bool) {
        let i = self.index(x, y);
        self.values[i] = v;
    }

    fn parse(input: &str) -> Self {
        let height = input.lines().count();
        let width = input.lines().next().unwrap().chars().count();
        let values = input
            .lines()
            .flat_map(|row| row.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Area { values, width, height }
    }

    fn iterate(&self) -> Self {
        let mut result = Area::new(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                let neighbour_count = self.count_set_neighbours(x, y);
                // dbg!(x, y, neighbour_count);
                result.set_at(x, y, (self.get_at(x, y) && neighbour_count == 2) || neighbour_count == 3);
            }
        }
        result
    }

    fn iterate2(&self) -> Self {
        let mut result = Area::new(self.width, self.height);
        for x in 0..self.width {
            for y in 0..self.height {
                let neighbour_count = self.count_set_neighbours(x, y);
                result.set_at(x, y, (self.get_at(x, y) && neighbour_count == 2) || neighbour_count == 3);
            }
        }
        result.set_at(0, 0, true);
        result.set_at(0, self.height - 1, true);
        result.set_at(self.width - 1, 0, true);
        result.set_at(self.width - 1, self.height - 1, true);
        result
    }
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> usize {
        (0..100)
            .fold(Area::parse(input), |lights, _| lights.iterate())
            .count_set()
    }

    #[test]
    fn test_run() {
        let input_string = ".#.#.#\n\
                                 ...##.\n\
                                 #....#\n\
                                 ..#...\n\
                                 #.#..#\n\
                                 ####..";
        let lights = Area::parse(input_string);
        let after1 = Area::parse(
            "..##..\n\
                   ..##.#\n\
                   ...##.\n\
                   ......\n\
                   #.....\n\
                   #.##..",
        );

        let after2 = Area::parse(
            "..###.\n\
                   ......\n\
                   ..###.\n\
                   ......\n\
                   .#....\n\
                   .#....",
        );

        let after3 = Area::parse(
            "...#..\n\
                   ......\n\
                   ...#..\n\
                   ..##..\n\
                   ......\n\
                   ......",
        );

        let after4 = Area::parse(
            "......\n\
                   ......\n\
                   ..##..\n\
                   ..##..\n\
                   ......\n\
                   ......",
        );
        let lights = lights.iterate();
        assert_eq!(after1, lights);
        assert_eq!(11, lights.count_set());
        let lights = lights.iterate();
        assert_eq!(after2, lights);
        assert_eq!(8, lights.count_set());
        let lights = lights.iterate();
        assert_eq!(after3, lights);
        assert_eq!(4, lights.count_set());
        let lights = lights.iterate();
        assert_eq!(after4, lights);
        assert_eq!(4, lights.count_set());
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> usize {
        (0..100)
            .fold(Area::parse(input), |lights, _| lights.iterate2())
            .count_set()
    }

    #[test]
    fn test_run() {
        let lights = Area::parse(
            "##.#.#\n\
                                      ...##.\n\
                                      #....#\n\
                                      ..#...\n\
                                      #.#..#\n\
                                      ####.#",
        );
        let lights = lights.iterate2();
        let lights = lights.iterate2();
        let lights = lights.iterate2();
        let lights = lights.iterate2();
        let lights = lights.iterate2();
        assert_eq!(17, lights.count_set());
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
    assert_eq!(part1_ans, 768);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 781);
}
