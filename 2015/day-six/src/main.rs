use std::path::{Component, Path};

struct Area {
    values: Vec<usize>,
    width: usize,
    _height: usize,
}

impl Area {
    fn new(width: usize, height: usize) -> Area {
        Area { values: vec![0; width * height], width, _height: height }
    }
    fn index(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }

    fn get_at(&self, x: usize, y: usize) -> usize {
        self.values[self.index(x, y)]
    }
    fn set_at(&mut self, x: usize, y: usize, v: usize) {
        let i = self.index(x, y);
        self.values[i] = v;
    }
}

impl std::fmt::Debug for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Area {\n")?;
        for v in self.values.chunks(self.width).map(|row| {
            row.iter()
                .map(|u| std::char::from_digit(*u as u32, 10).unwrap())
                .collect::<String>()
        }) {
            f.write_fmt(format_args!("\t\t{}\n", v))?;
        }
        f.write_str("}")
    }
}

fn parse_coords(s: &str) -> ((usize, usize), (usize, usize)) {
    let (a, b) = s.split_once(" through ").unwrap();
    let (a_x, a_y) = a.split_once(',').unwrap();
    let (b_x, b_y) = b.split_once(',').unwrap();
    (
        (a_x.parse::<usize>().unwrap(), a_y.parse::<usize>().unwrap()),
        (b_x.parse::<usize>().unwrap(), b_y.parse::<usize>().unwrap()),
    )
}

mod part1 {
    use super::*;

    #[allow(clippy::needless_range_loop)]
    fn parse(l: &str, a: &mut [[bool; 1000]; 1000]) {
        if let Some(coords) = l.strip_prefix("turn on ") {
            let ((x0, y0), (x1, y1)) = parse_coords(coords);
            for x in x0..=x1 {
                for y in y0..=y1 {
                    a[x][y] = true;
                }
            }
        } else if let Some(coords) = l.strip_prefix("turn off ") {
            let ((x0, y0), (x1, y1)) = parse_coords(coords);
            for x in x0..=x1 {
                for y in y0..=y1 {
                    a[x][y] = false;
                }
            }
        } else if let Some(coords) = l.strip_prefix("toggle ") {
            let ((x0, y0), (x1, y1)) = parse_coords(coords);
            for x in x0..=x1 {
                for y in y0..=y1 {
                    a[x][y] = !a[x][y];
                }
            }
        }
    }

    pub fn run(input: &str) -> usize {
        let mut a = Box::new([[false; 1000]; 1000]);
        input.lines().for_each(|l| parse(l, &mut a));
        a.iter().map(|a| a.iter().filter(|v| **v).count()).sum()
    }

    #[test]
    fn test_run() {
        assert_eq!(run("turn on 0,0 through 999,999"), 1_000_000);
        assert_eq!(run("toggle 0,0 through 999,0"), 1_000);
        assert_eq!(run("turn off 499,499 through 500,500"), 0);
        assert_eq!(
            run("turn on 0,0 through 999,999\n\
                 toggle 0,0 through 999,0"),
            1_000_000 - 1_000
        );
        assert_eq!(
            run("turn on 0,0 through 999,999\n\
                 turn off 499,499 through 500,500\n\
                 toggle 0,0 through 999,0"),
            1_000_000 - 1_000 - 4
        );
    }
}

mod part2 {
    use super::*;

    fn parse(l: &str, a: &mut Area) {
        if let Some(coords) = l.strip_prefix("turn on ") {
            let ((x0, y0), (x1, y1)) = parse_coords(coords);
            for x in x0..=x1 {
                for y in y0..=y1 {
                    a.set_at(x, y, a.get_at(x, y) + 1);
                }
            }
        } else if let Some(coords) = l.strip_prefix("turn off ") {
            let ((x0, y0), (x1, y1)) = parse_coords(coords);
            for x in x0..=x1 {
                for y in y0..=y1 {
                    a.set_at(x, y, a.get_at(x, y).saturating_sub(1));
                }
            }
        } else if let Some(coords) = l.strip_prefix("toggle ") {
            let ((x0, y0), (x1, y1)) = parse_coords(coords);
            for x in x0..=x1 {
                for y in y0..=y1 {
                    a.set_at(x, y, a.get_at(x, y) + 2);
                }
            }
        }
    }

    pub fn run(input: &str) -> usize {
        let mut a = Area::new(1000, 1000);
        input.lines().for_each(|l| parse(l, &mut a));
        a.values.into_iter().sum()
    }

    #[test]
    fn test_run() {
        assert_eq!(run("turn on 0,0 through 0,0"), 1);
        assert_eq!(run("toggle 0,0 through 999,999"), 2000000);
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
    assert_eq!(part1_ans, 543903);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 14687245);
}
