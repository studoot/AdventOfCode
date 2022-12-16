use std::collections::HashSet;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct Coord {
    x: u32,
    y: u32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Bad direction character"),
        }
    }
}

impl Coord {
    fn origin() -> Self {
        Self::new(u32::MAX / 2, u32::MAX / 2)
    }
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
    fn move_(&self, d: Direction) -> Self {
        match d {
            Direction::Up => Coord { y: self.y + 1, ..*self },
            Direction::Down => Coord { y: self.y - 1, ..*self },
            Direction::Left => Coord { x: self.x - 1, ..*self },
            Direction::Right => Coord { x: self.x + 1, ..*self },
        }
    }
    fn offset(&self, other: &Coord) -> [(Direction, u32); 2] {
        let x_offset = (if self.x > other.x { Direction::Left } else { Direction::Right }, self.x.abs_diff(other.x));
        let y_offset = (if self.y > other.y { Direction::Down } else { Direction::Up }, self.y.abs_diff(other.y));
        [x_offset, y_offset]
    }
    fn packed(&self) -> u64 {
        ((self.x as u64) << 32) | (self.y as u64)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct Rope<const N: usize> {
    knots: [Coord; N],
}

impl<const N: usize> Rope<N> {
    fn new(pt: Coord) -> Self {
        Self { knots: [pt; N] }
    }
    fn move_(&mut self, d: Direction) {
        self.knots[0] = self.knots[0].move_(d);
        for n in 1..N {
            let offset = self.knots[n].offset(&self.knots[n - 1]);
            self.knots[n] = if offset[0].1 >= 2 && offset[1].1 == 0 {
                self.knots[n].move_(offset[0].0)
            } else if offset[1].1 >= 2 && offset[0].1 == 0 {
                self.knots[n].move_(offset[1].0)
            } else if (offset[0].1 >= 2 && offset[1].1 >= 1) || (offset[0].1 >= 1 && offset[1].1 >= 2) {
                self.knots[n].move_(offset[0].0).move_(offset[1].0)
            } else {
                self.knots[n]
            }
        }
    }
}
fn evaluate<const N: usize>(s: &str) -> usize {
    let mut tail_points = HashSet::new();
    let mut r = Rope::<N>::new(Coord::origin());
    tail_points.insert(r.knots.last().unwrap().packed());
    s.lines().for_each(|l| {
        let (dir, count) = l
            .split_once(' ')
            .unwrap_or_else(|| panic!("Bad input line {l}"));
        let count = str::parse::<u32>(count).unwrap_or_else(|_| panic!("Bad count in line {l}"));
        let dir = Direction::from(dir.chars().next().unwrap());
        for _ in 0..count {
            r.move_(dir);
            tail_points.insert(r.knots.last().unwrap().packed());
        }
    });
    tail_points.len()
}

fn part1_evaluate(s: &str) -> usize {
    evaluate::<2>(s)
}

fn part2_evaluate(s: &str) -> usize {
    evaluate::<10>(s)
}

#[cfg(test)]
const TEST_INPUT_STRING_1: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
#[cfg(test)]
const TEST_INPUT_STRING_2: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1_evaluate(TEST_INPUT_STRING_1), 13);
}

#[test]
fn test_part2() {
    assert_eq!(part2_evaluate(TEST_INPUT_STRING_1), 1);
    assert_eq!(part2_evaluate(TEST_INPUT_STRING_2), 36);
}

pub fn run() -> Option<(usize, bool, usize, bool)> {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string);
    let part2_answer = part2_evaluate(input_string);
    Some((part1_answer, part1_answer == 6197, part2_answer, part2_answer == 2562))
}
