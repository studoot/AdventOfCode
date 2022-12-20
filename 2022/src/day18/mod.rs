use itertools::Itertools;
use ndarray::Array3;

fn parse(s: &str) -> Vec<(isize, isize, isize)> {
    s.lines()
        .map(|l| {
            let (x, y, z) = l
                .split(',')
                .map(|ns| {
                    ns.parse::<isize>()
                        .unwrap_or_else(|e| panic!("Bad number in {l} - {e}"))
                })
                .collect_tuple::<(_, _, _)>()
                .unwrap_or_else(|| panic!("Bad point in {l}"));
            (x, y, z)
        })
        .collect::<Vec<_>>()
}

type CubeType = u8;
const INTERNAL: CubeType = 0;
const SOLID: CubeType = 1;
const EXTERNAL: CubeType = 2;

fn evaluate(s: &str) -> (usize, usize) {
    let mut positions = parse(s);

    let adjacencies = [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];

    let Some((x_min,x_max)) = positions.iter().map(|(x, _, _)| x).minmax().into_option() else { panic!("Bad minmax on x co-ordinates!!!")};
    let Some((y_min,y_max)) = positions.iter().map(|(_, y, _)| y).minmax().into_option() else { panic!("Bad minmax on y co-ordinates!!!")};
    let Some((z_min,z_max)) = positions.iter().map(|(_, _, z)| z).minmax().into_option() else { panic!("Bad minmax on z co-ordinates!!!")};

    // We want at least one layer of blank cubes surrounding the filled cubes,
    // so normalize (x,y,z) co-ordinates to start at (1,1,1)
    let shape =
        ((*x_max - *x_min + 1) as usize + 2, (*y_max - *y_min + 1) as usize + 2, (*z_max - *z_min + 1) as usize + 2);
    let x_offset = 1 - *x_min;
    let y_offset = 1 - *y_min;
    let z_offset = 1 - *z_min;
    let x_range = 0..shape.0 as isize;
    let y_range = 0..shape.1 as isize;
    let z_range = 0..shape.2 as isize;

    let mut cubes = Array3::<CubeType>::from_elem(shape, INTERNAL);

    for (x, y, z) in &mut positions {
        *x += x_offset;
        *y += y_offset;
        *z += z_offset;
        cubes[[*x as usize, *y as usize, *z as usize]] = SOLID;
    }

    let naive_faces_count = positions
        .iter()
        .map(|(x, y, z)| {
            let adjacent_count = adjacencies
                .iter()
                .filter(|(dx, dy, dz)| {
                    let adjacent = (x + *dx, y + *dy, z + *dz);
                    cubes[[adjacent.0 as usize, adjacent.1 as usize, adjacent.2 as usize]] == INTERNAL
                })
                .count();
            adjacent_count
        })
        .sum();

    // Now start colouring cubes from (0,0,0)
    let mut to_check = Vec::with_capacity(shape.0 * shape.1 * shape.2);
    cubes[[0, 0, 0]] = EXTERNAL;
    to_check.push((0, 0, 0));
    while !to_check.is_empty() {
        let (x, y, z) = to_check.pop().unwrap();
        adjacencies.iter().for_each(|(dx, dy, dz)| {
            let adjacent =
                (dx.saturating_add_unsigned(x), dy.saturating_add_unsigned(y), dz.saturating_add_unsigned(z));
            if x_range.contains(&adjacent.0)
                && y_range.contains(&adjacent.1)
                && z_range.contains(&adjacent.2)
                && cubes[[adjacent.0 as usize, adjacent.1 as usize, adjacent.2 as usize]] == INTERNAL
            {
                cubes[[adjacent.0 as usize, adjacent.1 as usize, adjacent.2 as usize]] = EXTERNAL;
                to_check.push((adjacent.0 as usize, adjacent.1 as usize, adjacent.2 as usize));
            }
        });
    }

    let interface_faces = cubes
        .indexed_iter()
        .filter(|(_, value)| value == &&INTERNAL)
        .map(|((x, y, z), _)| {
            adjacencies
                .iter()
                .filter(|(dx, dy, dz)| {
                    let adjacent =
                        (x.saturating_add_signed(*dx), y.saturating_add_signed(*dy), z.saturating_add_signed(*dz));
                    cubes.get(adjacent).unwrap() == &SOLID
                })
                .count()
        })
        .sum::<usize>();

    (naive_faces_count, naive_faces_count - interface_faces)
}

#[cfg(test)]
const TEST_INPUT_STRING1: &str = "\
1,1,1
2,1,1";
#[cfg(test)]
const TEST_INPUT_STRING2: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(evaluate(TEST_INPUT_STRING1).0, 10);
    assert_eq!(evaluate(TEST_INPUT_STRING2).0, 64);
}

#[test]
fn test_part2() {
    assert_eq!(evaluate(TEST_INPUT_STRING2).1, 58);
}

pub fn run() -> Option<(usize, bool, usize, bool)> {
    let input_string = include_str!("./input.txt");
    let (part1_answer, part2_answer) = evaluate(input_string);
    Some((part1_answer, part1_answer == 3374, part2_answer, part2_answer == 2010))
}
