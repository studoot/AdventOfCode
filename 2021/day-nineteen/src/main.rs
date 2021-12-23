use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::{Add, Sub};
use std::path::{Component, Path};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
struct Point(i64, i64, i64);

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Point({},{},{})", self.0, self.1, self.2))
    }
}
impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl Add for &Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl Sub for &Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl Point {
    fn reorient(&self, orientation_index: u8) -> Point {
        match orientation_index % 48 {
            0 => Point(self.0, self.1, self.2),
            1 => Point(-self.0, self.1, self.2),
            2 => Point(self.0, -self.1, self.2),
            3 => Point(-self.0, -self.1, self.2),
            4 => Point(self.0, self.1, -self.2),
            5 => Point(-self.0, self.1, -self.2),
            6 => Point(self.0, -self.1, -self.2),
            7 => Point(-self.0, -self.1, -self.2),
            8 => Point(self.0, self.2, self.1),
            9 => Point(-self.0, self.2, self.1),
            10 => Point(self.0, -self.2, self.1),
            11 => Point(-self.0, -self.2, self.1),
            12 => Point(self.0, self.2, -self.1),
            13 => Point(-self.0, self.2, -self.1),
            14 => Point(self.0, -self.2, -self.1),
            15 => Point(-self.0, -self.2, -self.1),
            16 => Point(self.1, self.0, self.2),
            17 => Point(-self.1, self.0, self.2),
            18 => Point(self.1, -self.0, self.2),
            19 => Point(-self.1, -self.0, self.2),
            20 => Point(self.1, self.0, -self.2),
            21 => Point(-self.1, self.0, -self.2),
            22 => Point(self.1, -self.0, -self.2),
            23 => Point(-self.1, -self.0, -self.2),
            24 => Point(self.1, self.2, self.0),
            25 => Point(-self.1, self.2, self.0),
            26 => Point(self.1, -self.2, self.0),
            27 => Point(-self.1, -self.2, self.0),
            28 => Point(self.1, self.2, -self.0),
            29 => Point(-self.1, self.2, -self.0),
            30 => Point(self.1, -self.2, -self.0),
            31 => Point(-self.1, -self.2, -self.0),
            32 => Point(self.2, self.1, self.0),
            33 => Point(-self.2, self.1, self.0),
            34 => Point(self.2, -self.1, self.0),
            35 => Point(-self.2, -self.1, self.0),
            36 => Point(self.2, self.1, -self.0),
            37 => Point(-self.2, self.1, -self.0),
            38 => Point(self.2, -self.1, -self.0),
            39 => Point(-self.2, -self.1, -self.0),
            40 => Point(self.2, self.0, self.1),
            41 => Point(-self.2, self.0, self.1),
            42 => Point(self.2, -self.0, self.1),
            43 => Point(-self.2, -self.0, self.1),
            44 => Point(self.2, self.0, -self.1),
            45 => Point(-self.2, self.0, -self.1),
            46 => Point(self.2, -self.0, -self.1),
            47 => Point(-self.2, -self.0, -self.1),
            _ => self.clone(),
        }
    }
    fn displace(&self, displacement: &Point) -> Point {
        self + displacement
    }
}

fn find_common_displacement(reorientation: u8, num_required: usize, v0: &[Point], v1: &[Point]) -> Option<Point> {
    let mut v0 = v0.to_vec();
    v0.sort();
    let mut v1 = v1.to_vec();
    v1.sort();

    let mut offset_counts = HashMap::<Point, usize>::new();
    v0.iter()
        .flat_map(|p0| {
            v1.iter()
                .map(|p1| p0 - &p1.reorient(reorientation))
                .collect::<Vec<_>>()
        })
        .for_each(|p| {
            offset_counts.entry(p).and_modify(|c| *c += 1).or_insert(1);
        });
    let best_displacement = offset_counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .unwrap();
    if best_displacement.1 >= num_required {
        Some(best_displacement.0)
    } else {
        None
    }
}

fn find_common_displacement_and_orientation(num_required: usize, v0: &[Point], v1: &[Point]) -> Option<(Point, u8)> {
    for o in 0..48 {
        if let Some(p) = find_common_displacement(o, num_required, v0, v1) {
            return Some((p, o));
        }
    }
    None
}

fn find_overlapping_scanners(
    num_required: usize,
    known: &[Point],
    unknown: &[&Vec<Point>],
) -> Vec<(usize, Point, Vec<Point>)> {
    let mut just_found = Vec::new();
    for (i, &s) in unknown.iter().enumerate() {
        if let Some((disp, new_orientation)) =
            &find_common_displacement_and_orientation(num_required, known, s.as_slice())
        {
            just_found.push((
                i,
                disp.clone(),
                s.iter()
                    .map(|p| p.reorient(*new_orientation).displace(disp))
                    .collect::<Vec<_>>(),
            ));
        }
    }
    just_found
}

fn locate_all_scanners(scanners: &[Vec<Point>]) -> Vec<(Point, Vec<Point>)> {
    let mut located = vec![(Point(0, 0, 0), scanners[0].to_vec())];
    let mut to_be_located = scanners.iter().collect::<Vec<_>>();
    let mut known_index = 0;
    while !to_be_located.is_empty() {
        for (index, offset, reoriented_vector) in
            find_overlapping_scanners(12, &located[known_index].1, to_be_located.as_slice())
                .into_iter()
                .rev()
        {
            located.push((offset, reoriented_vector));
            to_be_located.remove(index);
        }
        known_index += 1;
        if known_index == located.len() {
            known_index = 0;
        }
    }
    located
}

fn parse_point(input: &str) -> Option<Point> {
    let a = input
        .split(',')
        .filter_map(|s| s.parse::<i64>().ok())
        .collect::<Vec<_>>();
    if a.len() != 3 {
        None
    } else {
        Some(Point(a[0], a[1], a[2]))
    }
}

fn parse(input: &str) -> Vec<Vec<Point>> {
    let mut scanners = Vec::new();
    let mut this_scanner = Vec::new();
    for l in input.lines() {
        if l.starts_with("--- scanner ") && !this_scanner.is_empty() {
            scanners.push(this_scanner);
            this_scanner = Vec::new();
        } else if let Some(p) = parse_point(l) {
            this_scanner.push(p)
        }
    }
    if !this_scanner.is_empty() {
        scanners.push(this_scanner)
    }
    scanners
}
mod part1 {
    use std::collections::HashSet;

    use super::*;

    pub fn run(input: &str) -> usize {
        let scanners = parse(input);
        let located = locate_all_scanners(&scanners);
        located
            .into_iter()
            .flat_map(|(_, p)| p)
            .collect::<HashSet<Point>>()
            .len()
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(79, run(input_string))
    }

    #[test]
    fn test_distances() {
        let input_string = include_str!("../input.txt");
        let scanners = parse(input_string);
        let distances = scanners
            .iter()
            .map(|s| {
                s.iter()
                    .enumerate()
                    .cartesian_product(s.iter().enumerate())
                    .filter_map(|((i_a, p_a), (i_b, p_b))| {
                        if i_a < i_b {
                            let offset = p_a - p_b;
                            let distance = (offset.0 * offset.0) + (offset.1 * offset.1) + (offset.2 * offset.2);
                            Some((distance, (i_a, i_b)))
                        } else {
                            None
                        }
                    })
                    .collect::<HashMap<_, _>>()
            })
            .collect::<Vec<_>>();

        let mut similar_pairs = distances
            .iter()
            .enumerate()
            .filter_map(|d_t| {
                distances
                    .iter()
                    .enumerate()
                    .filter_map(|d_o| {
                        if d_t.0 != d_o.0 {
                            let dist_t = d_t.1.keys().collect::<HashSet<_>>();
                            let dist_o = d_o.1.keys().collect::<HashSet<_>>();
                            let common_distance_count = dist_t.intersection(&dist_o).count();
                            Some((common_distance_count, (d_t.0, d_o.0)))
                        } else {
                            None
                        }
                    })
                    .max_by_key(|(count, (_, x))| (*count * 100).saturating_sub(*x))
            })
            .collect_vec();
        similar_pairs.sort_by_cached_key(|(_, indices)| *indices);
        for (count, indices) in &similar_pairs {
            let x = find_common_displacement_and_orientation(12, &scanners[indices.0], &scanners[indices.1]);
            println!("({}, {}) - {} - {:?}", indices.0, indices.1, count, x);
        }

        let mut d_1 = distances[1].iter().collect_vec();
        d_1.sort_by_key(|(d, _)| **d);
        let mut d_36 = distances[36].iter().collect_vec();
        d_36.sort_by_key(|(d, _)| **d);
        let mut i_1 = 0;
        let mut i_36 = 0;
        loop {
            if i_1 == d_1.len() || i_36 == d_36.len() {
                break;
            }
            if d_1[i_1].0 == d_36[i_36].0 {
                let d_1 = &d_1[i_1];
                let p_1_0 = &scanners[1][(d_1.1).0];
                let p_1_1 = &scanners[1][(d_1.1).1];
                let d_36 = &d_36[i_36];
                let p_36_0 = &scanners[36][(d_36.1).0];
                let p_36_1 = &scanners[36][(d_36.1).1];
                println!("{:?}   &   {:?}", p_1_0 - p_1_1, p_36_0 - p_36_1);
                i_1 += 1;
                i_36 += 1;
            } else if d_1[i_1].0 < d_36[i_36].0 {
                i_1 += 1;
            } else if d_1[i_1].0 > d_36[i_36].0 {
                i_36 += 1;
            }
        }
    }

    #[test]
    fn test_with_rotation() {
        let input_string = include_str!("../test.txt");
        let scanners = parse(input_string);
        let (disp, o) = find_common_displacement_and_orientation(12, &scanners[0], &scanners[1]).unwrap();
        assert_eq!(disp, Point(68, -1246, -43));
        let reoriented_vector = scanners[1]
            .iter()
            .map(|p| p.reorient(o).displace(&disp))
            .collect::<Vec<_>>();
        let (disp, o) = find_common_displacement_and_orientation(12, &reoriented_vector, &scanners[4]).unwrap();
        assert_eq!(disp, Point(-20, -1133, 1061));
        let reoriented_vector = scanners[4]
            .iter()
            .map(|p| p.reorient(o).displace(&disp))
            .collect::<Vec<_>>();
        let (disp, o) = find_common_displacement_and_orientation(12, &reoriented_vector, &scanners[2]).unwrap();
        assert_eq!(disp, Point(1105, -1205, 1229));
    }

    #[test]
    fn test_simple() {
        let mut scanner0 = vec![Point(0, 2, 0), Point(4, 1, 0), Point(3, 3, 0)];
        let scanner1 = vec![Point(-5, 0, 0), Point(-1, -1, 0), Point(-2, 1, 0)];
        assert_eq!(find_common_displacement(0, 3, &scanner0, &scanner1), Some(Point(5, 2, 0)));

        scanner0.insert(2, Point(0, 0, 0));
        assert_eq!(find_common_displacement(0, 3, &scanner0, &scanner1), Some(Point(5, 2, 0)));
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> i64 {
        let scanners = parse(input);
        let located = locate_all_scanners(&scanners);
        located
            .iter()
            .enumerate()
            .cartesian_product(located.iter().enumerate())
            .filter_map(|((i0, (o0, _)), (i1, (o1, _)))| {
                if i0 < i1 {
                    Some((o0.0 - o1.0).abs() + (o0.1 - o1.1).abs() + (o0.2 - o1.2).abs())
                } else {
                    None
                }
            })
            .max()
            .unwrap()
    }
    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(3621, run(input_string))
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
    assert_eq!(part1_ans, 472);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 12092);
}
