use once_cell::sync::OnceCell;
struct Shape {
    shape: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

const SPACE_WIDTH: usize = 7;

impl Shape {
    fn shapes() -> &'static [Shape; 5] {
        static INSTANCE: OnceCell<[Shape; 5]> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            let dash: Shape = Shape { shape: vec![(0, 0), (1, 0), (2, 0), (3, 0)], width: 4, height: 1 };
            let cross: Shape = Shape { shape: vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], width: 3, height: 3 };
            let l: Shape = Shape { shape: vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], width: 3, height: 3 };
            let pipe: Shape = Shape { shape: vec![(0, 0), (0, 1), (0, 2), (0, 3)], width: 1, height: 4 };
            let block: Shape = Shape { shape: vec![(0, 0), (1, 0), (0, 1), (1, 1)], width: 2, height: 2 };

            [dash, cross, l, pipe, block]
        })
    }
}

struct Sim {
    space: Vec<[bool; SPACE_WIDTH]>,
    shape_index: usize,
    gas: String,
    gas_index: usize,
}

impl Sim {
    fn new(g: &str) -> Self {
        Self { space: Vec::with_capacity(50_000_000), shape_index: 0, gas: g.to_owned(), gas_index: 0 }
    }
    fn can_move_rock_to(&self, x: usize, y: usize, shape: &Shape) -> bool {
        if y > self.space.len() {
            true
        } else {
            shape.shape.iter().all(|(offset_x, offset_y)| {
                (y + offset_y) >= self.space.len() || !self.space[y + offset_y][x + offset_x]
            })
        }
    }
    fn place_rock(&mut self, x: usize, y: usize, shape: &Shape) {
        let new_max_y = self.space.len().max(y + shape.height);
        self.space.reserve(shape.height);
        while self.space.len() < new_max_y {
            self.space.push([false; 7]);
        }
        shape
            .shape
            .iter()
            .for_each(|(offset_x, offset_y)| self.space[y + *offset_y][x + *offset_x] = true)
    }
    fn drop_rock(&mut self) {
        let shape = &Shape::shapes()[self.shape_index];
        let mut x = 2usize;
        let mut y = self.space.len() + 3;
        let max_x = SPACE_WIDTH - Shape::shapes()[self.shape_index].width;
        loop {
            let gas_move = self.gas.as_bytes()[self.gas_index];
            let new_x = if gas_move == b'<' { x.saturating_sub(1) } else { (x + 1).min(max_x) };
            self.gas_index = (self.gas_index + 1) % self.gas.len();
            if self.can_move_rock_to(new_x, y, shape) {
                x = new_x;
            }
            let new_y = y.saturating_sub(1);
            if y > 0 && self.can_move_rock_to(x, new_y, shape) {
                y = new_y;
            } else {
                self.place_rock(x, y, shape);
                break;
            }
        }
        self.shape_index = (self.shape_index + 1) % Shape::shapes().len();
    }
    #[allow(dead_code)]
    fn draw(&self) {
        for y in (self.space.len().saturating_sub(10)..self.space.len()).rev() {
            let line = self.space[y]
                .iter()
                .map(|b| if *b { '#' } else { '.' })
                .collect::<String>();
            println!("{line}");
            if y == 0 {
                println!("=======");
            }
        }
    }
}

impl PartialEq for Sim {
    fn eq(&self, other: &Self) -> bool {
        self.gas_index == other.gas_index
            && self.shape_index == other.shape_index
            && self.space[self.space.len() - 10..] == other.space[other.space.len() - 10..]
    }
}

fn part1_evaluate(s: &str) -> usize {
    // evaluate(s, 2022)
    let mut sim = Sim::new(s.trim());
    for _ in 0..2022 {
        sim.drop_rock();
    }
    sim.space.len()
}

fn part2_evaluate(s: &str) -> usize {
    let rock_count = 1_000_000_000_000usize;

    let mut sim1 = Sim::new(s.trim());
    let mut sim2 = Sim::new(s.trim());
    let mut first_match = None;
    let mut second_match = None;
    let mut height_at_first_match = 0;
    let mut height_at_second_match = 0;
    for i in 0..1_000_000 {
        sim1.drop_rock();
        sim2.drop_rock();
        sim2.drop_rock();
        if sim1 == sim2 {
            if first_match.is_none() {
                first_match = Some(i);
                height_at_first_match = sim1.space.len();
            } else if second_match.is_none() {
                second_match = Some(i);
                height_at_second_match = sim1.space.len();
            } else {
                break;
            }
        }
    }

    let cycle_length = second_match.unwrap() - first_match.unwrap();
    let cycle_count = (rock_count - first_match.unwrap()) / cycle_length;
    let remnant_after = (rock_count - first_match.unwrap()) % cycle_length;
    let rocks_per_cycle = height_at_second_match - height_at_first_match;

    sim1 = Sim::new(s.trim());
    for _ in 0..first_match.unwrap() + remnant_after {
        sim1.drop_rock();
    }
    sim1.space.len() + (cycle_count * rocks_per_cycle)
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1_evaluate(TEST_INPUT_STRING), 3068);
}

#[test]
fn test_part2() {
    assert_eq!(part2_evaluate(TEST_INPUT_STRING), 1_514_285_714_288);
}

pub fn run() -> Option<(usize, bool, usize, bool)> {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string);
    let part2_answer = part2_evaluate(input_string);
    Some((part1_answer, part1_answer == 3171, part2_answer, part2_answer == 1586627906921))
}
