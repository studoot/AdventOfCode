use std::path::{Component, Path};

mod dijkstra;
use crate::dijkstra::*;

#[derive(Default, Clone, PartialEq, Eq)]
struct Area {
    values: Vec<u8>,
    width: usize,
    height: usize,
}

impl Area {
    fn new(width: usize, height: usize) -> Area {
        Area { values: vec![0; width * height], width, height }
    }
    fn index(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }

    fn coords(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
    }

    fn get_at(&self, x: usize, y: usize) -> u8 {
        self.values[self.index(x, y)]
    }
    fn set_at(&mut self, x: usize, y: usize, v: u8) {
        let i = self.index(x, y);
        self.values[i] = v;
    }

    fn parse(input: &[&str]) -> Self {
        let width = input[0].chars().count();
        let height = input.len();
        let data = input
            .iter()
            .map(|l| l.chars().map(|c| (c as u8) - b'0').collect::<Vec<_>>())
            .flatten()
            .collect::<Vec<_>>();
        Self { values: data, width, height }
    }
}

impl std::fmt::Debug for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Area {\n")?;
        for v in self.values.chunks(self.width).map(|row| {
            row.iter()
                .map(|u| std::char::from_digit(*u as u32, 10).unwrap())
                .collect::<String>()
        }) {
            f.write_fmt(format_args!("\t\t{}\n", v))?;
        }
        f.write_str("}")
    }
}

fn find_min_route(a: &Area) -> usize {
    // First prepare the vertex/edge list
    let node_list = (0..a.values.len())
        .map(|i| {
            let mut adjacent_nodes = Vec::new();
            let c = a.coords(i);
            if c.0 < a.width - 1 {
                adjacent_nodes.push(Edge { node: i + 1, cost: a.values[i + 1] as usize });
            }
            if c.1 < a.height - 1 {
                adjacent_nodes.push(Edge { node: i + a.width, cost: a.values[i + a.width] as usize });
            }
            if c.0 > 0 {
                adjacent_nodes.push(Edge { node: i - 1, cost: a.values[i - 1] as usize });
            }
            if c.1 > 0 {
                adjacent_nodes.push(Edge { node: i - a.width, cost: a.values[i - a.width] as usize });
            }
            adjacent_nodes
        })
        .collect::<Vec<_>>();

    shortest_path(&node_list, 0, a.values.len() - 1).unwrap()
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let a = Area::parse(&input.lines().collect::<Vec<_>>());
        find_min_route(&a)
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(40, run(input_string))
    }

    #[test]
    fn test_run2() {
        let input_string = "19999\n\
        19111\n\
        11191";
        assert_eq!(8, run(input_string))
    }
}

mod part2 {
    use super::*;

    fn copy_area(from: &Area, to: &mut Area, row: usize, col: usize) {
        let value_offset = (row + col) as u8;
        let x_offset = col * from.width;
        let y_offset = row * from.height;
        for x in 0..from.width {
            for y in 0..from.height {
                to.set_at(x + x_offset, y + y_offset, value_mod(from.get_at(x, y) + value_offset));
            }
        }
    }

    fn value_mod(v: u8) -> u8 {
        ((v - 1) % 9) + 1
    }

    pub fn run(input: &str) -> usize {
        let a = Area::parse(&input.lines().collect::<Vec<_>>());
        let mut full_area = Area::new(a.width * 5, a.height * 5);
        for row in 0..5 {
            for col in 0..5 {
                copy_area(&a, &mut full_area, row, col);
            }
        }
        find_min_route(&full_area)
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(315, run(input_string))
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
    assert_eq!(part1_ans, 613);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 2899);
}
