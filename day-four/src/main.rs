use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Point {
    number: u8,
    marked: bool,
}

impl Point {
    pub fn mark_if(&mut self, n: u8) -> bool {
        if self.number == n {
            self.marked = true;
            true
        } else {
            false
        }
    }
}
impl From<u8> for Point {
    fn from(n: u8) -> Self {
        Point { number: n, marked: false }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Board([[Point; 5]; 5], bool);

impl Board {
    pub fn parse(from: &[&str; 5]) -> Board {
        let mut b: Board = Default::default();
        for (l, s) in from.iter().enumerate() {
            for (i, s) in s.split_ascii_whitespace().enumerate() {
                b.0[l][i] = Point::from(u8::from_str(s).unwrap())
            }
        }
        b
    }

    pub fn mark(&mut self, number: u8) -> bool {
        if !self.1 {
            'outer: for i in 0..5 {
                for j in 0..5 {
                    if self.0[i][j].mark_if(number) {
                        self.1 = self.winning_row(i) || self.winning_column(j);
                        break 'outer;
                    };
                }
            }
        }
        self.1
    }
    pub fn unmarked_sum(&mut self) -> u32 {
        self.0
            .iter()
            .fold(0, |acc, row| acc + row.iter().fold(0, |acc, p| acc + (if p.marked { 0 } else { p.number as u32 })))
    }

    pub fn winning_row(&self, row: usize) -> bool {
        self.0[row].iter().all(|p| p.marked)
    }
    pub fn winning_column(&self, col: usize) -> bool {
        self.0.iter().all(|row| row[col].marked)
    }

    pub fn winning_board(&self) -> bool {
        (0..5).any(|i| self.winning_column(i) || self.winning_row(i))
    }

    pub fn has_won(&self) -> bool {
        self.1
    }
}

impl From<[[u8; 5]; 5]> for Board {
    fn from(ns: [[u8; 5]; 5]) -> Self {
        let mut b: Board = Default::default();
        for (i, row) in ns.iter().enumerate() {
            for (j, n) in row.iter().enumerate() {
                b.0[i][j] = Point::from(*n);
            }
        }
        b
    }
}

#[test]
pub fn test_parse() {
    let b = Board::parse(&["22 13 17 11  0", "8  2 23  4 24", "21  9 14 16  7", "6 10  3 18  5", "1 12 20 15 19"]);
    assert_eq!(
        b,
        Board::from([
            [22u8, 13u8, 17u8, 11u8, 0u8],
            [8u8, 2u8, 23u8, 4u8, 24u8],
            [21u8, 9u8, 14u8, 16u8, 7u8],
            [6u8, 10u8, 3u8, 18u8, 5u8],
            [1u8, 12u8, 20u8, 15u8, 19u8]
        ],),
    );
}

#[test]
pub fn test_mark() {
    let mut b = Board::parse(&["22 13 17 11  0", "8  2 23  4 24", "21  9 14 16  7", "6 10  3 18  5", "1 12 20 15 19"]);
    let mut c = b.clone();
    assert_eq!(false, b.mark(13));
    assert_eq!(false, b.mark(9));
    assert_eq!(false, b.mark(12));
    assert_eq!(false, b.mark(11));
    assert_eq!(false, b.mark(33));
    assert_eq!(false, b.mark(10));
    assert_eq!(true, b.mark(2));
    assert_eq!(true, b.winning_column(1));
    assert_eq!(true, b.winning_board());
    assert_eq!(false, c.mark(10));
    assert_eq!(false, c.mark(3));
    assert_eq!(false, c.mark(5));
    assert_eq!(false, c.mark(6));
    assert_eq!(true, c.mark(18));
    assert_eq!(true, c.winning_row(3));
    assert_eq!(true, c.winning_board());
}

pub fn parse(input: &str) -> (Vec<u8>, Vec<Board>) {
    let mut ls = input.lines();
    let calls = ls.next().unwrap().split(',').map(|s| u8::from_str(s).unwrap()).collect::<Vec<_>>();
    let boards: Vec<_> = ls
        .chunks(6)
        .into_iter()
        .map(|chunk| chunk.collect_tuple().map(|(_a, b, c, d, e, f)| Board::parse(&[b, c, d, e, f])).unwrap())
        .collect::<Vec<_>>();
    (calls, boards)
}

mod part1 {
    use super::*;
    pub fn run(input: &str) -> u32 {
        let (calls, mut boards) = parse(input);
        for c in calls {
            for b in &mut boards {
                if b.mark(c) {
                    return b.unmarked_sum() * (c as u32);
                }
            }
        }
        0
    }
    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(4512, run(input_string))
    }
}

mod part2 {
    use super::*;
    pub fn run(input: &str) -> u32 {
        let (calls, mut boards) = parse(input);
        for c in calls {
            for b in &mut boards {
                if !b.has_won() {
                    b.mark(c);
                }
            }
            if boards.len() == 1 && boards[0].has_won() {
                return boards[0].unmarked_sum() * (c as u32);
            }
            boards.retain(|b| !b.has_won());
        }
        0
    }
    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(1924, run(input_string))
    }
}

fn main() {
    let input_string = include_str!("../input.txt");
    println!("Day  4 part 1 - {}", part1::run(input_string));
    println!("Day  4 part 2 - {}", part2::run(input_string));
}
