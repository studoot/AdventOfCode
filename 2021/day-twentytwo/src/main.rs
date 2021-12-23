use bitvec::prelude::*;
use itertools::Itertools;
use std::ops::{RangeInclusive, Sub};
use std::path::{Component, Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AxisRange {
    min: isize,
    max: isize,
}
impl AxisRange {
    fn new(min: isize, max: isize) -> Self {
        AxisRange { min: min.min(max), max: max.max(min) }
    }
    fn len(&self) -> usize {
        (self.max + 1 - self.min) as usize
    }
    fn contains(&self, other: &AxisRange) -> bool {
        self.min <= other.min && other.max <= self.max
    }
    fn overlaps(&self, other: &AxisRange) -> bool {
        self.min <= other.max && other.min <= self.max
    }
    fn overlap(&self, other: &AxisRange) -> Option<AxisRange> {
        if self.overlaps(other) {
            Some(AxisRange::new(self.min.max(other.min), self.max.min(other.max)))
        } else {
            None
        }
    }
    fn as_range(&self) -> RangeInclusive<isize> {
        self.min..=self.max
    }
}
#[derive(Debug, Clone)]
struct Cuboid {
    x: AxisRange,
    y: AxisRange,
    z: AxisRange,
    holes: Vec<Cuboid>,
}
impl Cuboid {
    fn points(&self) -> isize {
        ((self.x.len() * self.y.len() * self.z.len()) as isize) - self.holes.iter().fold(0, |acc, h| acc + h.points())
    }
    fn contains(&self, other: &Cuboid) -> bool {
        self.x.contains(&other.x) && self.y.contains(&other.y) && self.z.contains(&other.z)
    }
    fn overlaps(&self, other: &Cuboid) -> bool {
        self.x.overlaps(&other.x) && self.y.overlaps(&other.y) && self.z.overlaps(&other.z)
    }
    fn add_hole(&mut self, from: &Cuboid) {
        for hole in &mut self.holes {
            hole.add_hole(from);
        }
        if let Some(hole) = self.overlap(from) {
            self.holes.push(hole)
        }
    }
    fn overlap(&self, other: &Self) -> Option<Cuboid> {
        match (self.x.overlap(&other.x), self.y.overlap(&other.y), self.z.overlap(&other.z)) {
            (Some(x_overlap), Some(y_overlap), Some(z_overlap)) => {
                Some(Cuboid { x: x_overlap, y: y_overlap, z: z_overlap, holes: vec![] })
            }
            _ => None,
        }
    }
}
impl Sub for &Cuboid {
    type Output = Vec<Cuboid>;
    fn sub(self, other: Self) -> Self::Output {
        let start_volume = self.points();
        if other.contains(self) {
            Vec::new()
        } else if !self.overlaps(other) {
            vec![self.clone()]
        } else {
            fn get_coords(me: &AxisRange, other: &AxisRange) -> Vec<AxisRange> {
                let mut ps = Vec::new();
                let mut min = me.min;
                if me.min < other.min && other.min <= me.max {
                    ps.push(AxisRange { min, max: other.min - 1 });
                    min = other.min;
                };
                if me.min <= other.max && other.max < me.max {
                    ps.push(AxisRange { min, max: other.max });
                    min = other.max + 1;
                };
                ps.push(AxisRange { min, max: me.max });
                ps
            }
            let x_overlap = AxisRange { min: self.x.min.max(other.x.min), max: self.x.max.min(other.x.max) };
            let y_overlap = AxisRange { min: self.y.min.max(other.y.min), max: self.y.max.min(other.y.max) };
            let z_overlap = AxisRange { min: self.z.min.max(other.z.min), max: self.z.max.min(other.z.max) };
            let c_overlap = Cuboid { x: x_overlap, y: y_overlap, z: z_overlap, holes: Vec::new() };
            let xs = get_coords(&self.x, &other.x);
            let ys = get_coords(&self.y, &other.y);
            let zs = get_coords(&self.z, &other.z);
            let mut acc_vol = 0;
            let mut rej_vol = 0;
            let c = xs
                .clone()
                .into_iter()
                .cartesian_product(ys.clone().into_iter())
                .cartesian_product(zs.clone().into_iter())
                .filter_map(|((x, y), z)| {
                    let c = Cuboid { x, y, z, holes: Vec::new() };
                    if other.contains(&c) || c.points() <= 0 {
                        rej_vol += &c.points();
                        None
                    } else {
                        acc_vol += &c.points();
                        Some(c)
                    }
                })
                .collect::<Vec<_>>();
            let tot_acc_points = c.iter().map(|c| c.points()).sum::<isize>();
            assert_eq!(start_volume, acc_vol + rej_vol);
            if c_overlap.points() != rej_vol || start_volume - c_overlap.points() != acc_vol {
                dbg!(&self, other, &xs, &ys, &zs, c_overlap, start_volume, acc_vol, rej_vol, tot_acc_points, &c);
            }
            c
        }
    }
}
struct Reactor {
    cubes: BitVec,
}
impl Reactor {
    fn new() -> Self {
        Reactor { cubes: bitvec![0;100*100*100] }
    }
    fn side_range() -> AxisRange {
        AxisRange::new(-50, 50)
    }
    fn index(x: isize, y: isize, z: isize) -> usize {
        (((x + 50) as usize) * 100 * 100) + ((y + 50) as usize) * 100 + ((z + 50) as usize)
    }
    fn set(&mut self, x: isize, y: isize, z: isize, is_on: bool) {
        if (-50..=50).contains(&x) && (-50..=50).contains(&y) && (-50..=50).contains(&z) {
            *self.cubes.get_mut(Reactor::index(x, y, z)).unwrap() = is_on;
        }
    }
    fn count_on(&self) -> usize {
        self.cubes.count_ones()
    }
}

fn parse(input: &str) -> Vec<(bool, Cuboid)> {
    input
        .lines()
        .map(|l| {
            let (on_off, cube) = l.split_once(' ').unwrap();
            let is_on = on_off == "on";
            let ranges = cube
                .split(',')
                .map(|s| {
                    let (_axis, range) = s.split_once('=').unwrap();
                    let (min, max) = range.split_once("..").unwrap();
                    let min = min.parse::<isize>().unwrap();
                    let max = max.parse::<isize>().unwrap();
                    AxisRange::new(min, max)
                })
                .collect::<Vec<_>>();
            (is_on, Cuboid { x: ranges[0], y: ranges[1], z: ranges[2], holes: vec![] })
        })
        .collect::<Vec<_>>()
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let mut r = Reactor::new();
        let reboot_sequence = parse(input);
        for (is_on, c) in reboot_sequence {
            if let (Some(x_range), Some(y_range), Some(z_range)) = (
                c.x.overlap(&Reactor::side_range()),
                c.y.overlap(&Reactor::side_range()),
                c.z.overlap(&Reactor::side_range()),
            ) {
                for x in x_range.as_range() {
                    for y in y_range.as_range() {
                        for z in z_range.as_range() {
                            r.set(x, y, z, is_on);
                        }
                    }
                }
            }
        }
        r.count_on()
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(590784, run(input_string))
    }
}

mod part2 {
    use super::*;

    pub fn run_with_holes(input: &str) -> isize {
        let reboot_sequence = parse(input);
        let mut on_cuboids = Vec::<Cuboid>::new();
        for (is_on, c) in reboot_sequence.into_iter() {
            for existing in &mut on_cuboids {
                existing.add_hole(&c);
            }
            if is_on {
                on_cuboids.push(c);
            }
        }
        on_cuboids.into_iter().map(|c| c.points()).sum()
    }

    pub fn run_with_cubes(input: &str) -> isize {
        let reboot_sequence = parse(input);
        let mut on_cuboids = Vec::<Cuboid>::new();
        for (is_on, c) in reboot_sequence.into_iter() {
            on_cuboids = on_cuboids
                .into_iter()
                .flat_map(|existing| &existing - &c)
                .collect_vec();
            if is_on {
                on_cuboids.push(c);
            }
        }
        on_cuboids.into_iter().map(|c| c.points()).sum()
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test2.txt");
        assert_eq!(2758514936282235, run_with_holes(input_string));
        assert_eq!(2758514936282235, run_with_cubes(input_string));
    }

    #[test]
    fn test_sub() {
        let a = Cuboid {
            x: AxisRange { min: -41, max: -33 },
            y: AxisRange { min: -41, max: -28 },
            z: AxisRange { min: -36, max: 6 },
            holes: Vec::new(),
        };
        let b = Cuboid {
            x: AxisRange { min: -33, max: 15 },
            y: AxisRange { min: -32, max: 19 },
            z: AxisRange { min: -34, max: 11 },
            holes: Vec::new(),
        };

        assert!(a.overlaps(&b));
        assert!(!a.contains(&b));
        assert!(!b.contains(&a));
        assert!(a.contains(&a));

        let c = &a - &b;
        let c_volume = c.iter().fold(0, |acc, c| acc + c.points());
        assert!(dbg!(c_volume) < dbg!(a.points()));
    }

    #[test]
    fn test_simple() {
        // let input_string = "on x=0..10,y=0..10,z=0..10";
        // assert_eq!(11 * 11 * 11, run(input_string));

        // let input_string = "on x=0..10,y=0..10,z=0..10\n\
        // off x=0..5,y=0..5,z=0..5\n\
        // ";
        // assert_eq!(11 * 11 * 11 - 6 * 6 * 6, run(input_string));

        let input_string = "on x=0..10,y=0..10,z=0..10\n\
        on x=-5..5,y=-5..5,z=-5..5\n\
        ";
        assert_eq!(2446, run_with_holes(input_string));
        assert_eq!(2446, run_with_cubes(input_string));

        // let input_string = "on x=0..10,y=0..10,z=0..10\n\
        // off x=0..5,y=0..5,z=0..5\n\
        // on x=0..1,y=0..1,z=0..1\n\
        // ";
        // assert_eq!(11 * 11 * 11 - 6 * 6 * 6 + 2 * 2 * 2, run(input_string));

        let input_string = "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35";
        assert_eq!(248314, run_with_holes(input_string));
        assert_eq!(248314, run_with_cubes(input_string));

        let input_string = "on x=-1..2,y=-1..2,z=-1..2
on x=0..1,y=0..1,z=0..1
";
        assert_eq!(64, run_with_holes(input_string));
        assert_eq!(64, run_with_cubes(input_string));
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
    assert_eq!(part1_ans, 556501);

    let now = std::time::Instant::now();
    let part2_ans = part2::run_with_holes(input_string);
    println!(
        "Day {} part 2 (with holes) - {} - took {} milliseconds.",
        day_number,
        part2_ans,
        now.elapsed().as_millis()
    );
    assert_eq!(part2_ans, 1217140271559773);
    let part2_ans = part2::run_with_cubes(input_string);
    println!(
        "Day {} part 2 (with cubes) - {} - took {} milliseconds.",
        day_number,
        part2_ans,
        now.elapsed().as_millis()
    );
    assert_eq!(part2_ans, 1217140271559773);
}
