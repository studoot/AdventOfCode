use std::fmt::Debug;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub struct Point {
    x: u32,
    y: u32,
}
impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sx, sy) = s.split_once(',').unwrap();
        let x = u32::from_str(sx)?;
        let y = u32::from_str(sy)?;
        Ok(Point { x, y })
    }
}

#[derive(Debug)]
pub struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").unwrap();
        let start = Point::from_str(start)?;
        let end = Point::from_str(end)?;
        Ok(Line { start, end })
    }
}

impl Line {
    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }
    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
}

pub fn parse(input: &str) -> (Vec<Line>, (u32, u32)) {
    let lines = input
        .lines()
        .map(|s| Line::from_str(s).unwrap())
        .collect::<Vec<_>>();
    let (max_x, max_y) = lines.iter().fold((0, 0), |(max_x, max_y), this_line| {
        (max_x.max(this_line.start.x.max(this_line.end.x)), max_y.max(this_line.start.y.max(this_line.end.y)))
    });
    (lines, (1 + max_x, 1 + max_y))
}

#[derive(Debug)]
pub struct Map {
    area: Vec<u32>,
    width: u32,
    _height: u32,
}

impl Map {
    pub fn from_size(width: u32, height: u32) -> Self {
        Map { area: vec![0; (width * height) as usize], width, _height: height }
    }
    pub fn plot_line(&mut self, l: &Line) {
        if l.is_horizontal() {
            for x in l.start.x.min(l.end.x)..=l.start.x.max(l.end.x) {
                self.inc_point(x, l.start.y)
            }
        } else if l.is_vertical() {
            for y in l.start.y.min(l.end.y)..=l.start.y.max(l.end.y) {
                self.inc_point(l.start.x, y)
            }
        } else {
            let x_inc = l.start.x < l.end.x;
            let y_inc = l.start.y < l.end.y;
            let mut x = l.start.x;
            let mut y = l.start.y;
            loop {
                self.inc_point(x, y);
                if x == l.end.x {
                    break;
                }
                if x_inc {
                    x += 1;
                } else {
                    x -= 1;
                }
                if y_inc {
                    y += 1;
                } else {
                    y -= 1;
                }
            }
        }
    }
    pub fn inc_point(&mut self, x: u32, y: u32) {
        self.area[(x + (y * self.width)) as usize] += 1;
    }

    pub fn get_point(&self, x: u32, y: u32) -> u32 {
        self.area[(x + (y * self.width)) as usize]
    }

    pub fn dangerous_points(&self) -> Vec<Point> {
        let mut danger_points = Vec::new();
        for (i, v) in self.area.iter().enumerate() {
            if *v >= 2 {
                danger_points.push(Point { x: (i as u32) % self.width, y: (i as u32) / self.width });
            }
        }
        danger_points
    }

    pub fn plot(&self) -> String {
        let mut s = String::new();
        for y in 0..self._height {
            for x in 0..self.width {
                s.push_str(&self.get_point(x, y).to_string());
            }
            s.push('\n')
        }
        s
    }
}
mod part1 {
    use super::*;
    pub fn run(input: &str) -> usize {
        let (lines, (xsize, ysize)) = parse(input);
        let mut map = Map::from_size(xsize, ysize);

        for l in lines {
            if l.is_horizontal() || l.is_vertical() {
                map.plot_line(&l)
            }
        }
        map.dangerous_points().len()
    }
    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(5, run(input_string))
    }
}

mod part2 {
    use super::*;
    pub fn run(input: &str) -> usize {
        let (lines, (xsize, ysize)) = parse(input);
        let mut map = Map::from_size(xsize, ysize);

        for l in lines {
            map.plot_line(&l)
        }
        map.dangerous_points().len()
    }
    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(12, run(input_string))
    }
}
fn main() {
    let input_string = include_str!("../input.txt");
    println!("Day  5 part 1 - {}", part1::run(input_string));
    println!("Day  5 part 2 - {}", part2::run(input_string));
}
