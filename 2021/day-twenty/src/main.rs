use std::path::{Component, Path};

#[derive(Default, Clone, PartialEq, Eq)]
struct Area {
    values: Vec<bool>,
    width: usize,
    height: usize,
}

impl Area {
    fn new(width: usize, height: usize) -> Area {
        Area { values: vec![false; width * height], width, height }
    }
    fn index(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }

    fn get_at(&self, x: isize, y: isize, background: bool) -> bool {
        if (0..self.width as isize).contains(&x) && (0..self.height as isize).contains(&y) {
            self.values[self.index(x as usize, y as usize)]
        } else {
            background
        }
    }
    fn set_at(&mut self, x: usize, y: usize, v: bool) {
        let i = self.index(x, y);
        self.values[i] = v;
    }

    fn parse(input: &[&str]) -> Self {
        let width = input[0].chars().count();
        let height = input.len();
        let data = input
            .iter()
            .map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
            .flatten()
            .collect::<Vec<_>>();
        Self { values: data, width, height }
    }
}

fn parse(input: &str) -> (Vec<bool>, Area) {
    let mut l = input.lines();
    let algorithm = l
        .next()
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>();
    l.next();
    (algorithm, Area::parse(l.collect::<Vec<_>>().as_slice()))
}

fn enhance_pixel(x: isize, y: isize, image: &Area, algo: &[bool], background: bool) -> bool {
    let mut index = 0;
    for y in [y - 1, y, y + 1] {
        for x in [x - 1, x, x + 1] {
            let p = image.get_at(x, y, background);
            index = (index << 1) + p as usize;
        }
    }
    algo[index]
}

fn enhance(image: Area, algo: &[bool], background: bool) -> Area {
    let mut new_image = Area::new(image.width + 2, image.height + 2);

    for y in -1..(image.height + 1) as isize {
        for x in -1..(image.width + 1) as isize {
            let new_pixel = enhance_pixel(x, y, &image, algo, background);
            new_image.set_at((x + 1) as usize, (y + 1) as usize, new_pixel)
        }
    }
    new_image
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let (algo, image) = parse(input);
        let enhanced = enhance(image, &algo, false);
        let enhanced = enhance(enhanced, &algo, algo[0]);
        enhanced.values.into_iter().filter(|p| *p).count()
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(35, run(input_string))
    }
    #[test]
    fn test_run2() {
        let input_string = include_str!("../test2.txt");
        assert_eq!(5326, run(input_string))
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let (algo, image) = parse(input);
        let mut enhanced = image;
        enhanced = enhance(enhanced, &algo, false);
        let odd_background = algo[0];
        let even_background = algo[0] & algo[511];
        for _ in 0..24 {
            enhanced = enhance(enhanced, &algo, odd_background);
            enhanced = enhance(enhanced, &algo, even_background);
        }
        enhanced = enhance(enhanced, &algo, odd_background);
        enhanced.values.into_iter().filter(|p| *p).count()
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(3351, run(input_string))
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
    assert_eq!(part1_ans, 5354);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 18269);
}
