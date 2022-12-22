use super::grid::{Grid, GridCoord};

#[derive(Default, Debug, Copy, Clone)]
struct GridPoint {
    height: u8,
    step_count: Option<u16>,
}

impl GridPoint {
    fn new_with_steps(h: u8, steps: u16) -> Self {
        GridPoint { height: h, step_count: Some(steps) }
    }
    fn new(h: u8) -> Self {
        GridPoint { height: h, step_count: None }
    }
}

fn parse_grid(input: &str) -> (Grid<GridPoint>, GridCoord, GridCoord, Vec<GridCoord>) {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut grid = Grid::new(width, height);
    let mut start = Option::None;
    let mut end = Option::None;
    let mut lowest_points = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, col) in line.bytes().enumerate() {
            let here = GridCoord::from((x, y));
            match col {
                b'a' => {
                    *grid.cell_mut(here).unwrap() = GridPoint::new(0);
                    lowest_points.push(here);
                }
                b'b'..=b'z' => *grid.cell_mut(here).unwrap() = GridPoint::new(col - b'a'),
                b'S' => {
                    lowest_points.push(here);
                    start = Some(here);
                    *grid.cell_mut(here).unwrap() = GridPoint::new_with_steps(0, 0);
                }
                b'E' => {
                    end = Some(here);
                    *grid.cell_mut(here).unwrap() = GridPoint::new(b'z' - b'a');
                }
                c => panic!("Bad character {c} found in grid at {here:?}"),
            }
        }
    }

    match start.zip(end) {
        Some((start, end)) => (grid, start, end, lowest_points),
        _ => panic!("No start or end point(s) found in grid"),
    }
}

fn can_move(points: &Grid<GridPoint>, from_height: u8, to_coord: GridCoord) -> Option<GridCoord> {
    let to_point = points.cell(to_coord).expect("check.1");
    (from_height + 2 > to_point.height && to_point.step_count.is_none()).then_some(to_coord)
}

fn move_from(points: &mut Grid<GridPoint>, from_points: &[GridCoord]) -> Vec<GridCoord> {
    from_points
        .iter()
        .flat_map(|p| {
            let next_point = *points.cell(*p).expect("move_from.1");
            let next_step = next_point.step_count.expect("move_from.2") + 1;

            let next_points = [
                (p.x > 0).then(|| GridCoord::from((p.x - 1, p.y))),
                (p.y > 0).then(|| GridCoord::from((p.x, p.y - 1))),
                (p.x < points.width() - 1).then(|| GridCoord::from((p.x + 1, p.y))),
                (p.y < points.height() - 1).then(|| GridCoord::from((p.x, p.y + 1))),
            ]
            .into_iter()
            .filter_map(|p| p.and_then(|p| can_move(points, next_point.height, p)))
            .collect::<Vec<_>>();
            next_points
                .iter()
                .for_each(|p| points.cell_mut(*p).expect("move_from.3").step_count = Some(next_step));
            next_points
        })
        .collect::<Vec<_>>()
}

fn part1_evaluate(s: &str) -> usize {
    let (mut points, start, end, _) = parse_grid(s);

    let mut moved_to = move_from(&mut points, &[start]);
    while !moved_to.is_empty() {
        moved_to = move_from(&mut points, &moved_to);
    }

    points.cell(end).unwrap().step_count.unwrap() as usize
}

fn part2_evaluate(s: &str) -> usize {
    let (mut points, _, end, mut lowest_points) = parse_grid(s);

    lowest_points
        .iter()
        .for_each(|p| points.cell_mut(*p).expect("part2_evaluate.1").step_count = Some(0));
    while !lowest_points.is_empty() {
        lowest_points = move_from(&mut points, &lowest_points);
    }

    points.cell(end).unwrap().step_count.unwrap() as usize
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = "\
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1_evaluate(TEST_INPUT_STRING), 31);
}

#[test]
fn test_part2() {
    assert_eq!(part2_evaluate(TEST_INPUT_STRING), 29);
}

pub fn run() -> Option<(usize, bool, usize, bool)> {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string);
    let part2_answer = part2_evaluate(input_string);
    Some((part1_answer, part1_answer == 456, part2_answer, part2_answer == 454))
}
