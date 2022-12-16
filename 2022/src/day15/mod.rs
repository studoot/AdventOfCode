use std::ops::RangeInclusive;
use std::str::FromStr;

pub(crate) struct Coord {
    pub(crate) x: isize,
    pub(crate) y: isize,
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(isize, isize)> for Coord {
    fn from((x, y): (isize, isize)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Sensor {
    location: Coord,
    nearest_beacon: Coord,
}

impl Sensor {
    fn distance_to_nearest_beacon(&self) -> usize {
        self.location.x.abs_diff(self.nearest_beacon.x) + self.location.y.abs_diff(self.nearest_beacon.y)
    }

    fn scanned_points(&self, row: isize) -> (Option<RangeInclusive<isize>>, Option<isize>) {
        let maybe_beacon = (self.nearest_beacon.y == row).then_some(self.nearest_beacon.x);
        let centre_x = self.location.x;
        let vertical_distance = self.location.y.abs_diff(row);
        if vertical_distance > self.distance_to_nearest_beacon() {
            return (None, maybe_beacon);
        }
        let x_extent = (self.distance_to_nearest_beacon() - vertical_distance) as isize;
        (Some(centre_x - x_extent..=centre_x + x_extent), maybe_beacon)
    }
}

impl FromStr for Sensor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(':')
            .and_then(|(sensor, beacon)| {
                let sensor_location = sensor.split_once(", ").and_then(|(x, y)| {
                    let x = x
                        .strip_prefix("Sensor at x=")
                        .and_then(|s| s.parse::<isize>().ok())?;
                    let y = y.strip_prefix("y=").and_then(|s| s.parse::<isize>().ok())?;
                    Some((x, y))
                })?;
                let beacon_location = beacon.split_once(", ").and_then(|(x, y)| {
                    let x = x
                        .strip_prefix(" closest beacon is at x=")
                        .and_then(|s| s.parse::<isize>().ok())?;
                    let y = y.strip_prefix("y=").and_then(|s| s.parse::<isize>().ok())?;
                    Some((x, y))
                })?;
                Some(Sensor { location: sensor_location.into(), nearest_beacon: beacon_location.into() })
            })
            .ok_or_else(|| format!("Bad sensor definition in '{s}'"))
    }
}

fn parse(s: &str) -> Vec<Sensor> {
    s.lines()
        .map(Sensor::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|e| panic!("Bad parse - {e}"))
}

#[allow(clippy::reversed_empty_ranges)]
fn get_covered_ranges(
    sensors: &Vec<Sensor>,
    row: isize,
    range: &RangeInclusive<isize>,
    out: &mut Vec<RangeInclusive<isize>>,
) {
    out.clear();
    for s in sensors {
        let (covered_points, _) = s.scanned_points(row);
        if let Some(mut covered_points) = covered_points {
            covered_points = *covered_points.start().max(range.start())..=*covered_points.end().min(range.end());
            for r in out.iter_mut() {
                if covered_points.start().max(r.start()) <= covered_points.end().min(r.end()) {
                    covered_points = (*covered_points.start()).min(*r.start())..=(*covered_points.end()).max(*r.end());
                    *r = 1..=0;
                }
            }
            out.push(covered_points);
        }
    }
    out.retain(|r| !r.is_empty());
}

#[allow(clippy::reversed_empty_ranges)]
fn part1_evaluate(s: &str, row: isize) -> usize {
    let sensors = parse(s);

    let mut all_covered_points = Vec::<RangeInclusive<isize>>::new();
    let mut beacons = Vec::new();
    for s in sensors {
        let (covered_points, maybe_beacon) = s.scanned_points(row);
        if let Some(mut covered_points) = covered_points {
            for r in &mut all_covered_points {
                if covered_points.start().max(r.start()) <= covered_points.end().min(r.end()) {
                    covered_points = (*covered_points.start()).min(*r.start())..=(*covered_points.end()).max(*r.end());
                    *r = 1..=0;
                }
            }
            all_covered_points.push(covered_points);
        }
        if let Some(beacon) = maybe_beacon {
            beacons.push(beacon);
        }
    }

    for beacon in beacons {
        let mut right = Option::None;
        for r in &mut all_covered_points {
            if r.contains(&beacon) {
                let left = *r.start()..=beacon - 1;
                right = Some(beacon + 1..=*r.end());
                *r = left;
            }
        }
        if let Some(right) = right {
            all_covered_points.push(right);
        }
    }

    all_covered_points
        .into_iter()
        .map(|r| if r.is_empty() { 0 } else { r.end() - r.start() + 1 })
        .sum::<isize>() as usize
}

fn part2_evaluate(s: &str, coord_range: RangeInclusive<isize>) -> usize {
    let sensors = parse(s);

    let mut all_covered_points = Vec::<RangeInclusive<isize>>::new();
    for y in coord_range.clone() {
        get_covered_ranges(&sensors, y, &coord_range, &mut all_covered_points);
        if all_covered_points.len() == 2 {
            let missing_x = all_covered_points[0]
                .start()
                .max(all_covered_points[1].start())
                - 1;
            return ((missing_x * 4_000_000) + y) as usize;
        }
    }
    0
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = "\
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1_evaluate(TEST_INPUT_STRING, 10), 26);
}

#[test]
fn test_part2() {
    assert_eq!(part2_evaluate(TEST_INPUT_STRING, 0..=20), 56_000_011);
}

pub fn run() -> Option<(usize, bool, usize, bool)> {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string, 2000000);
    let part2_answer = part2_evaluate(input_string, 0..=4_000_000);
    Some((part1_answer, part1_answer == 6425133, part2_answer, part2_answer == 10996191429555))
}
