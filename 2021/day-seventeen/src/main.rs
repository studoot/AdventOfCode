use itertools::Itertools;
use std::ops::RangeInclusive;
use std::path::{Component, Path};

type Trajectory = Vec<(isize, isize)>;

fn simulate(
    mut x_speed: isize,
    mut y_speed: isize,
    target_area: &(RangeInclusive<isize>, RangeInclusive<isize>),
) -> Option<Trajectory> {
    let mut x = 0;
    let mut y = 0;
    let mut valid = false;
    let x_range = &target_area.0;
    let y_range = &target_area.1;
    let max_x_limit = x_range.start().max(x_range.end()) + 1;
    let min_y_limit = y_range.start().min(y_range.end()) - 1;
    let t = [(x, y)]
        .into_iter()
        .chain((1..).map_while(|_| {
            x += x_speed;
            y += y_speed;
            x_speed = 0.max(x_speed - 1);
            y_speed -= 1;
            valid |= x_range.contains(&x) && y_range.contains(&y);
            if y >= min_y_limit && x <= max_x_limit {
                Some((x, y))
            } else {
                None
            }
        }))
        .collect::<Vec<_>>();
    if valid {
        Some(t)
    } else {
        None
    }
}

fn valid_trajectories(target_area: &(RangeInclusive<isize>, RangeInclusive<isize>)) -> Vec<(isize, isize, Trajectory)> {
    let x_range = &target_area.0;
    let y_range = &target_area.1;
    let min_y_limit = y_range.start().min(y_range.end()) - 1;
    let max_x_limit = x_range.start().max(x_range.end()) + 1;
    (1..max_x_limit)
        .cartesian_product(min_y_limit..-min_y_limit)
        .filter_map(|(vx, vy)| simulate(vx, vy, target_area).map(|t| (vx, vy, t)))
        .collect::<Vec<_>>()
}

mod part1 {
    use std::ops::RangeInclusive;

    use super::*;

    pub fn run(target_area: (RangeInclusive<isize>, RangeInclusive<isize>)) -> isize {
        valid_trajectories(&target_area)
            .into_iter()
            .map(|(_, _, t)| t.into_iter().map(|(_, py)| py).max().unwrap())
            .max()
            .unwrap()
    }

    #[test]
    fn test_run() {
        let target_area = (20..=30, -10..=-5);
        assert_eq!(45, run(target_area))
    }
}

mod part2 {
    use super::*;

    pub fn run(target_area: (RangeInclusive<isize>, RangeInclusive<isize>)) -> usize {
        valid_trajectories(&target_area).len()
    }

    #[test]
    fn test_run() {
        let target_area = (20..=30, -10..=-5);
        assert_eq!(112, run(target_area));
    }
}

fn main() {
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
    let part1_ans = part1::run(((211..=232), (-124..=-69)));
    println!("Day {} part 1 - {} - took {} milliseconds.", day_number, part1_ans, now.elapsed().as_millis());
    assert_eq!(part1_ans, 7626);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(((211..=232), (-124..=-69)));
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 2032);
}
