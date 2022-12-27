use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct AbsX(usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Y(usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct RowX(usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Coord {
    x: RowX,
    y: Y,
}
#[cfg(test)]
fn coord(x: usize, y: usize) -> Coord {
    Coord { x: RowX(x), y: Y(y) }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Cell {
    Open,
    Wall,
}

impl TryFrom<char> for Cell {
    type Error = String;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Cell::Open),
            '#' => Ok(Cell::Wall),
            c => Err(format!("Bad character {c}!")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Row {
    offset: usize,
    cells: Vec<Cell>,
}

impl Row {
    fn to_absolute_x(&self, x: RowX) -> AbsX {
        AbsX(x.0 + self.offset)
    }
    fn to_row_x(&self, x: AbsX) -> Option<RowX> {
        x.0.checked_sub(self.offset)
            .and_then(|x| (x < self.cells.len()).then_some(x))
            .map(RowX)
    }
    fn move_to(&self, x: RowX, to_row: &Row) -> Option<RowX> {
        let x = self.to_absolute_x(x);
        to_row.to_row_x(x)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Board {
    rows: Vec<Row>,
}

impl Board {
    fn absolute_location(&self, location: Coord) -> (AbsX, Y) {
        (self.rows[location.y.0].to_absolute_x(location.x), location.y)
    }

    fn move_horizontal_part1(&self, start: Coord, vector: isize) -> Coord {
        let Coord { mut x, y } = start;
        let row = &self.rows[y.0];
        let step = vector.signum();
        let count = vector.abs();
        for _ in 0..count {
            let next_x = if x.0 == 0 && step == -1 {
                row.cells.len() - 1
            } else if x.0 == row.cells.len() - 1 && step == 1 {
                0
            } else {
                x.0.saturating_add_signed(step)
            };
            if row.cells[next_x] == Cell::Wall {
                break;
            } else {
                x.0 = next_x;
            }
        }
        Coord { x, y }
    }
    fn move_vertical_part1(&self, start: Coord, vector: isize) -> Coord {
        let Coord { mut x, mut y } = start;
        let step = vector.signum();
        let count = vector.abs();

        for _ in 0..count {
            let current_row = &self.rows[y.0];
            let new_y = y.0.checked_add_signed(step);
            let (next_x, next_y) = match new_y
                .and_then(|new_y| self.rows.get(new_y))
                .and_then(|next_row| current_row.move_to(x, next_row))
            {
                Some(new_x) => (new_x, new_y.unwrap()),
                None => {
                    let mut next_x = x;
                    let mut next_y = y.0;
                    loop {
                        let new_y = next_y.checked_add_signed(-step);
                        let new_x = new_y
                            .and_then(|new_y| self.rows.get(new_y))
                            .and_then(|next_row| current_row.move_to(x, next_row));
                        if let (Some(new_x), Some(new_y)) = (new_x, new_y) {
                            next_x = new_x;
                            next_y = new_y;
                        } else {
                            break (next_x, next_y);
                        }
                    }
                }
            };
            let next_row = &self.rows[next_y];
            if next_row.cells[next_x.0] == Cell::Wall {
                break;
            } else {
                y.0 = next_y;
                x = next_x;
            }
        }
        Coord { x, y }
    }

    fn move_one_part2(&self, start: Coord, direction: Direction) -> (Coord, Direction) {
        (Coord { x, y }, direction)
    }

    fn move_part2(&self, start: Coord, direction: Direction, count: usize) -> (Coord, Direction) {
        (0..count).fold((start, direction), |(pos, direction), _index| self.move_one_part2(pos, direction))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn turn_left(self) -> Self {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Move {
    Forward(usize),
    TurnRight,
    TurnLeft,
}

type Moves = Vec<Move>;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Player {
    location: Coord,
    direction: Direction,
}

impl Player {
    fn new() -> Self {
        Player { location: Coord { x: RowX(0), y: Y(0) }, direction: Direction::Right }
    }
    fn perform_movement_part1(&self, board: &Board, movement: Move) -> Self {
        match movement {
            Move::Forward(count) => {
                let new_location = match self.direction {
                    Direction::Right => board.move_horizontal_part1(self.location, count as isize),
                    Direction::Down => board.move_vertical_part1(self.location, count as isize),
                    Direction::Left => board.move_horizontal_part1(self.location, -(count as isize)),
                    Direction::Up => board.move_vertical_part1(self.location, -(count as isize)),
                };
                Player { location: new_location, ..*self }
            }
            Move::TurnLeft => Player { direction: self.direction.turn_left(), ..*self },
            Move::TurnRight => Player { direction: self.direction.turn_right(), ..*self },
        }
    }
    fn perform_movement_part2(&self, board: &Board, movement: Move) -> Self {
        match movement {
            Move::Forward(count) => {
                let (new_location, new_direction) = board.move_part2(self.location, self.direction, count);
                Player { location: new_location, direction: new_direction }
            }
            Move::TurnLeft => Player { direction: self.direction.turn_left(), ..*self },
            Move::TurnRight => Player { direction: self.direction.turn_right(), ..*self },
        }
    }
}

fn parse(s: &str) -> Result<(Board, Moves), String> {
    let (board, moves) = s
        .split_once("\n\n")
        .ok_or("Bad input - not in board/moves order!!!!")?;
    let board = board
        .lines()
        .map(|line| {
            let just_cells = line.trim_start();
            let offset = line.len() - just_cells.len();
            let cells = just_cells
                .trim_end()
                .chars()
                .map(Cell::try_from)
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Row { offset, cells })
        })
        .collect::<Result<Vec<_>, String>>()?;

    let moves = moves
        .trim()
        .chars()
        .group_by(|c| c.is_alphabetic())
        .into_iter()
        .flat_map(|(is_letters, g)| {
            if is_letters {
                g.into_iter()
                    .map(|c| match c {
                        'L' => Ok(Move::TurnLeft),
                        'R' => Ok(Move::TurnRight),
                        _ => Err(format!("Bad character {c}")),
                    })
                    .collect::<Result<Vec<_>, _>>()
            } else {
                g.into_iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .map_err(|_| "Bad integer".to_owned())
                    .map(|i| vec![Move::Forward(i)])
            }
        })
        .flatten()
        .collect::<Vec<_>>();
    Ok((Board { rows: board }, moves))
}

fn part1_evaluate(s: &str) -> usize {
    let Ok((board, moves)) = parse(s) else {
        panic!("Bad input!");
    };
    let mut p = Player::new();
    for movement in moves {
        p = p.perform_movement_part1(&board, movement);
    }
    let abs_loc = board.absolute_location(p.location);
    (1000 * (abs_loc.1 .0 + 1)) + (4 * (abs_loc.0 .0 + 1)) + (p.direction as usize)
}

fn part2_evaluate(s: &str) -> usize {
    0
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

#[cfg(test)]
#[test]
fn test_movement() {
    let x = parse(TEST_INPUT_STRING);
    assert!(x.is_ok());
    let board = x.unwrap().0;

    assert_eq!(board.move_horizontal_part1(coord(0, 0), 1), coord(1, 0));
    assert_eq!(board.move_horizontal_part1(coord(0, 0), 2), coord(2, 0));
    assert_eq!(board.move_horizontal_part1(coord(0, 0), 3), coord(2, 0));
    assert_eq!(board.move_horizontal_part1(coord(0, 0), -1), coord(0, 0));
    assert_eq!(board.move_horizontal_part1(coord(0, 0), -2), coord(0, 0));
    assert_eq!(board.move_horizontal_part1(coord(0, 0), -3), coord(0, 0));
    assert_eq!(board.move_horizontal_part1(coord(0, 1), 1), coord(0, 1));
    assert_eq!(board.move_horizontal_part1(coord(0, 1), 2), coord(0, 1));
    assert_eq!(board.move_horizontal_part1(coord(0, 1), 3), coord(0, 1));
    assert_eq!(board.move_horizontal_part1(coord(0, 1), -1), coord(3, 1));
    assert_eq!(board.move_horizontal_part1(coord(0, 1), -2), coord(2, 1));
    assert_eq!(board.move_horizontal_part1(coord(0, 1), -3), coord(2, 1));
    assert_eq!(board.move_horizontal_part1(coord(0, 3), 1), coord(1, 3));
    assert_eq!(board.move_horizontal_part1(coord(0, 3), 2), coord(2, 3));
    assert_eq!(board.move_horizontal_part1(coord(0, 3), 3), coord(3, 3));
    assert_eq!(board.move_horizontal_part1(coord(0, 3), 4), coord(0, 3));
    assert_eq!(board.move_horizontal_part1(coord(0, 3), -1), coord(3, 3));
    assert_eq!(board.move_horizontal_part1(coord(0, 3), -2), coord(2, 3));
    assert_eq!(board.move_horizontal_part1(coord(0, 3), -3), coord(1, 3));
    assert_eq!(board.move_horizontal_part1(coord(0, 3), -4), coord(0, 3));

    assert_eq!(board.move_vertical_part1(coord(0, 0), 1), coord(0, 1));
    assert_eq!(board.move_vertical_part1(coord(0, 0), 2), coord(0, 1));

    assert_eq!(board.move_vertical_part1(coord(0, 0), -1), coord(0, 11));
    assert_eq!(board.move_vertical_part1(coord(0, 0), -2), coord(0, 10));

    assert_eq!(board.move_vertical_part1(coord(2, 0), 3), coord(2, 3));
    assert_eq!(board.move_vertical_part1(coord(2, 0), 4), coord(10, 4));
    assert_eq!(board.move_vertical_part1(coord(2, 0), 5), coord(10, 5));
    assert_eq!(board.move_vertical_part1(coord(2, 0), 6), coord(10, 6));
    assert_eq!(board.move_vertical_part1(coord(2, 0), 7), coord(10, 6));

    assert_eq!(board.move_vertical_part1(coord(8, 7), 1), coord(0, 8));
}

#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1_evaluate(TEST_INPUT_STRING), 6032);
}

#[test]
fn test_part2() {
    assert_eq!(part2_evaluate(TEST_INPUT_STRING), 5031);
}

pub fn run() -> Option<(usize, bool, usize, bool)> {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string);
    let part2_answer = part2_evaluate(input_string);
    Some((part1_answer, part1_answer == 31568, part2_answer, part2_answer == 0))
}
