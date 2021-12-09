use std::cmp::Reverse;

#[derive(Debug, Default)]
struct Area {
    heights: Vec<u8>,
    width: usize,
    height: usize,
}

struct LowPointIter<'a> {
    i: Box<dyn Iterator<Item = (usize, usize)> + 'a>,
    _a: &'a Area,
}

impl<'a> Area {
    fn new_from_str(input: &str) -> Self {
        let mut width = Option::None;
        let data = input
            .lines()
            .map(|l| {
                if *width.get_or_insert(l.len()) != l.len() {
                    panic!("Non-uniform widths!");
                };
                l.chars().map(|c| (c as u8) - b'0').collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();
        let height = data.len() / width.unwrap();
        Area { heights: data, width: width.unwrap(), height }
    }

    fn index_from_coords(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }

    fn coords_from_index(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
    }

    fn height_at(&self, x: usize, y: usize) -> u8 {
        self.heights[x + (y * self.width)]
    }

    fn low_points(&'a self) -> LowPointIter<'a> {
        LowPointIter {
            i: Box::new(self.heights.iter().enumerate().filter_map(|(i, h)| {
                let p @ (x, y) = self.coords_from_index(i);
                if (x == 0 || self.height_at(x - 1, y) > *h)
                    && (y == 0 || self.height_at(x, y - 1) > *h)
                    && (x == self.width - 1 || self.height_at(x + 1, y) > *h)
                    && (y == self.height - 1 || self.height_at(x, y + 1) > *h)
                {
                    Some(p)
                } else {
                    None
                }
            })),
            _a: self,
        }
    }

    fn flood(&self, basin_points: &mut Vec<bool>, x: usize, y: usize) {
        if self.height_at(x, y) < 9 {
            basin_points[self.index_from_coords(x, y)] = true;
            if x > 0 && !basin_points[self.index_from_coords(x - 1, y)] {
                self.flood(basin_points, x - 1, y);
            };
            if y > 0 && !basin_points[self.index_from_coords(x, y - 1)] {
                self.flood(basin_points, x, y - 1);
            };
            if x < self.width - 1 && !basin_points[self.index_from_coords(x + 1, y)] {
                self.flood(basin_points, x + 1, y);
            };
            if y < self.height - 1 && !basin_points[self.index_from_coords(x, y + 1)] {
                self.flood(basin_points, x, y + 1);
            };
        }
    }

    fn find_basin(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut basin_points = vec![false; self.heights.len()];
        self.flood(&mut basin_points, x, y);
        basin_points
            .into_iter()
            .enumerate()
            .filter_map(|(i, is_in_basin)| if is_in_basin { Some(self.coords_from_index(i)) } else { None })
            .collect::<Vec<_>>()
    }
}

impl<'a> Iterator for LowPointIter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.i.next()
    }
}

fn parse(input: &str) -> Area {
    Area::new_from_str(input)
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let area = parse(input);
        area.low_points()
            .map(|(x, y)| 1 + area.height_at(x, y) as usize)
            .sum()
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(15, run(input_string))
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let area = parse(input);
        let mut basins = area
            .low_points()
            .map(|(x, y)| area.find_basin(x, y))
            .collect::<Vec<_>>();
        basins.sort_by_key(|b| Reverse(b.len()));
        basins.dedup();
        basins.into_iter().take(3).map(|b| b.len()).product()
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(1134, run(input_string))
    }
}
fn main() {
    let input_string = include_str!("../input.txt");
    let part1_ans = part1::run(input_string);
    println!("Day  9 part 1 - {}", part1_ans);
    assert_eq!(part1_ans, 504);
    let part2_ans = part2::run(input_string);
    println!("Day  9 part 2 - {}", part2_ans);
    assert_eq!(part2_ans, 1558722);
}
