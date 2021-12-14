#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Point(usize, usize);
type Points = Vec<Point>;

impl Point {
    fn apply_fold(&mut self, fold: &Fold) {
        match *fold {
            Fold::X(fold_x) => self.0 = if self.0 > fold_x { 2 * fold_x - self.0 } else { self.0 },
            Fold::Y(fold_y) => self.1 = if self.1 > fold_y { 2 * fold_y - self.1 } else { self.1 },
        }
    }
}

fn apply_folds_to_points(points: &mut Points, folds: &[Fold]) {
    folds
        .iter()
        .for_each(|fold| points.iter_mut().for_each(|p| p.apply_fold(fold)));
    points.sort_unstable();
    points.dedup();
}

#[derive(Debug, Eq, PartialEq)]
enum Fold {
    X(usize),
    Y(usize),
}
type Folds = Vec<Fold>;

fn parse(input: &str) -> (Points, Folds) {
    let mut points = Points::new();
    let mut folds = Folds::new();
    input.lines().for_each(|l| {
        if let Some(fold_x) = l.strip_prefix("fold along x=") {
            folds.push(Fold::X(fold_x.parse().unwrap()))
        } else if let Some(fold_y) = l.strip_prefix("fold along y=") {
            folds.push(Fold::Y(fold_y.parse().unwrap()))
        } else if let Some((x, y)) = l.split_once(',') {
            points.push(Point(x.parse().unwrap(), y.parse().unwrap()))
        }
    });
    (points, folds)
}

fn plot_points(points: &[Point]) -> String {
    let (width, height) = points
        .iter()
        .fold((0, 0), |(w, h), &Point(px, py)| (w.max(px + 1), h.max(py + 1)));
    let mut plot_area = vec![" ".repeat(width); height];
    for &Point(x, y) in points {
        let (replace_pos, c) = plot_area[y].char_indices().nth(x).unwrap();
        plot_area[y].replace_range(replace_pos..replace_pos + c.len_utf8(), "▮")
    }
    plot_area.join("\n")
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let (mut points, folds) = parse(input);
        apply_folds_to_points(&mut points, &folds[0..1]);
        points.len()
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(17, run(input_string))
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> String {
        let (mut points, folds) = parse(input);
        apply_folds_to_points(&mut points, &folds);
        plot_points(&points)
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(
            "▮▮▮▮▮\n\
             ▮   ▮\n\
             ▮   ▮\n\
             ▮   ▮\n\
             ▮▮▮▮▮",
            run(input_string)
        )
    }
}

fn main() {
    let input_string = include_str!("../input.txt");
    let now = std::time::Instant::now();
    let part1_ans = part1::run(input_string);
    println!("Day 13 part 1 - {} - took {} milliseconds.", part1_ans, now.elapsed().as_millis());
    assert_eq!(part1_ans, 751);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day 13 part 2 took {time} milliseconds.\n{answer}", answer = part2_ans, time = now.elapsed().as_millis());
    assert_eq!(
        part2_ans,
        "▮▮▮   ▮▮  ▮  ▮ ▮▮▮  ▮  ▮ ▮    ▮  ▮ ▮   \n\
         ▮  ▮ ▮  ▮ ▮  ▮ ▮  ▮ ▮ ▮  ▮    ▮ ▮  ▮   \n\
         ▮  ▮ ▮    ▮▮▮▮ ▮  ▮ ▮▮   ▮    ▮▮   ▮   \n\
         ▮▮▮  ▮ ▮▮ ▮  ▮ ▮▮▮  ▮ ▮  ▮    ▮ ▮  ▮   \n\
         ▮    ▮  ▮ ▮  ▮ ▮ ▮  ▮ ▮  ▮    ▮ ▮  ▮   \n\
         ▮     ▮▮▮ ▮  ▮ ▮  ▮ ▮  ▮ ▮▮▮▮ ▮  ▮ ▮▮▮▮"
    );
}
