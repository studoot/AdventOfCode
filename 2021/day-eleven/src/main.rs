use itertools::Itertools;

#[derive(Default, Clone, PartialEq, Eq)]
struct Area {
    values: Vec<u8>,
    width: usize,
    height: usize,
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

impl Area {
    fn index(&self, x: usize, y: usize) -> usize {
        x + (y * self.width)
    }

    fn coords(&self, i: usize) -> (usize, usize) {
        (i % self.width, i / self.width)
    }

    fn parse(input: &[&str]) -> Self {
        let width = input[0].chars().count();
        let height = input.len();
        let data = input
            .iter()
            .map(|l| l.chars().map(|c| (c as u8) - b'0').collect::<Vec<_>>())
            .flatten()
            .collect::<Vec<_>>();
        Self { values: data, width, height }
    }

    fn inc(&mut self, x: usize, y: usize) {
        let index = self.index(x, y);
        self.values[index] += 1;
        if self.values[index] == 10 {
            for this_x in [x.wrapping_sub(1), x, x + 1] {
                for this_y in [y.wrapping_sub(1), y, y + 1] {
                    if this_x < self.width && this_y < self.height {
                        self.inc(this_x, this_y);
                    }
                }
            }
        }
    }

    fn step(&mut self) -> usize {
        (0..self.values.len()).for_each(|i| {
            let (x, y) = self.coords(i);
            self.inc(x, y);
        });
        self.values
            .iter_mut()
            .map(|v| {
                if *v > 9 {
                    *v = 0;
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}

fn parse_steps(input: &str) -> Vec<Area> {
    input
        .lines()
        .group_by(|s| s.is_empty())
        .into_iter()
        .filter_map(
            |(group_flag, lines)| if group_flag { None } else { Some(Area::parse(&lines.into_iter().collect_vec())) },
        )
        .collect_vec()
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let mut octopii = parse_steps(input).swap_remove(0);
        (0..100).map(|_i| octopii.step()).sum()
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(1656, run(input_string))
    }

    #[test]
    fn test_steps() {
        let input_string = include_str!("../test-steps.txt");
        let mut step = parse_steps(input_string);
        assert_eq!(step.len(), 20);
        step[0].step();
        assert_eq!(step[0], step[1]);
        step[0].step();
        assert_eq!(step[0], step[2]);
        step[0].step();
        assert_eq!(step[0], step[3]);
        step[0].step();
        assert_eq!(step[0], step[4]);
        step[0].step();
        assert_eq!(step[0], step[5]);
        step[0].step();
        assert_eq!(step[0], step[6]);
        step[0].step();
        assert_eq!(step[0], step[7]);
        step[0].step();
        assert_eq!(step[0], step[8]);
        step[0].step();
        assert_eq!(step[0], step[9]);
        step[0].step();
        assert_eq!(step[0], step[10]);
        (0..10).for_each(|_| {
            step[0].step();
        });
        assert_eq!(step[0], step[11]);
        (0..10).for_each(|_| {
            step[0].step();
        });
        assert_eq!(step[0], step[12]);
        (0..10).for_each(|_| {
            step[0].step();
        });
        assert_eq!(step[0], step[13]);
        (0..10).for_each(|_| {
            step[0].step();
        });
        assert_eq!(step[0], step[14]);
        (0..10).for_each(|_| {
            step[0].step();
        });
        assert_eq!(step[0], step[15]);
        (0..10).for_each(|_| {
            step[0].step();
        });
        assert_eq!(step[0], step[16]);
        (0..10).for_each(|_| {
            step[0].step();
        });
        assert_eq!(step[0], step[17]);
        (0..10).for_each(|_| {
            step[0].step();
        });
        assert_eq!(step[0], step[18]);
        (0..10).for_each(|_| {
            step[0].step();
        });
        assert_eq!(step[0], step[19]);
    }

    #[test]
    fn test_small() {
        let input_string = include_str!("../small-test-steps.txt");
        let mut step = parse_steps(input_string);
        assert_eq!(step[0].step(), 9);
        assert_eq!(step[0], step[1]);
        assert_eq!(step[0].step(), 0);
        assert_eq!(step[0], step[2]);
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let mut octopii = parse_steps(input).swap_remove(0);
        (1..)
            .find(|_| octopii.step() == (octopii.width * octopii.height))
            .unwrap()
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(195, run(input_string))
    }
}
fn main() {
    let input_string = include_str!("../input.txt");
    let part1_ans = part1::run(input_string);
    println!("Day 11 part 1 - {}", part1_ans);
    assert_eq!(part1_ans, 1562);
    let part2_ans = part2::run(input_string);
    println!("Day 11 part 2 - {}", part2_ans);
    assert_eq!(part2_ans, 268);
}
