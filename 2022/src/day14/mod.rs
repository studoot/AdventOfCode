use super::grid::{Grid, GridCoord};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Material {
    Air,
    Rock,
    Sand,
}

impl Default for Material {
    fn default() -> Self {
        Material::Air
    }
}

fn parse_coord(s: &str) -> Option<GridCoord> {
    s.split_once(',')
        .and_then(|(x, y)| x.parse::<usize>().ok().zip(y.parse::<usize>().ok()))
        .map(GridCoord::from)
}

fn draw_line(g: &mut Grid<Material>, e0: &GridCoord, e1: &GridCoord) {
    if e0.x == e1.x {
        for y in e0.y.min(e1.y)..=e0.y.max(e1.y) {
            *g.cell_mut(GridCoord::from((e0.x, y))).unwrap() = Material::Rock;
        }
    } else if e0.y == e1.y {
        for x in e0.x.min(e1.x)..=e0.x.max(e1.x) {
            *g.cell_mut(GridCoord::from((x, e0.y))).unwrap() = Material::Rock;
        }
    }
}

fn parse(s: &str) -> (Grid<Material>, usize) {
    let mut g = Grid::new(2000, 2000);
    let mut max_y = 0;
    s.lines().for_each(|s| {
        let vertices = s
            .split(" -> ")
            .map(parse_coord)
            .collect::<Option<Vec<_>>>()
            .unwrap_or_else(|| panic!("Bad line {s}"));
        vertices[0..]
            .iter()
            .zip(vertices[1..].iter())
            .for_each(|(e0, e1)| {
                max_y = max_y.max(e0.y).max(e1.y);
                draw_line(&mut g, e0, e1);
            });
    });
    (g, max_y)
}

fn drop_sand(g: &mut Grid<Material>) -> bool {
    let mut at = GridCoord::from((500, 0));
    if *g.cell(at).unwrap() != Material::Air {
        return false;
    }

    while at.x != 0 && at.x != g.width() && at.y != g.height() - 1 {
        let next = GridCoord::from((at.x, at.y + 1));
        match g.cell(next).unwrap() {
            Material::Air => {
                at = next;
                continue;
            }
            Material::Rock | Material::Sand => {
                let next_left = GridCoord::from((at.x - 1, at.y + 1));
                if *g.cell(next_left).unwrap() == Material::Air {
                    at = next_left;
                    continue;
                }
                let next_right = GridCoord::from((at.x + 1, at.y + 1));
                if *g.cell(next_right).unwrap() == Material::Air {
                    at = next_right;
                    continue;
                }
                *g.cell_mut(at).unwrap() = Material::Sand;
                return true;
            }
        }
    }
    false
}

fn part1_evaluate(s: &str) -> usize {
    let (mut g, _) = parse(s);
    let mut grains_dropped = 0;
    while drop_sand(&mut g) {
        grains_dropped += 1;
    }
    grains_dropped
}

fn part2_evaluate(s: &str) -> usize {
    let (mut g, max_y) = parse(s);
    draw_line(&mut g, &GridCoord::from((0, max_y + 2)), &GridCoord::from((1999, max_y + 2)));
    let mut grains_dropped = 0;
    while drop_sand(&mut g) {
        grains_dropped += 1;
    }
    grains_dropped
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1_evaluate(TEST_INPUT_STRING), 24);
}

#[test]
fn test_part2() {
    assert_eq!(part2_evaluate(TEST_INPUT_STRING), 93);
}

pub fn run() -> Option<(usize, bool, usize, bool)> {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string);
    let part2_answer = part2_evaluate(input_string);
    Some((part1_answer, part1_answer == 683, part2_answer, part2_answer == 28821))
}
