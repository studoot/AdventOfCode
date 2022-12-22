#[allow(clippy::upper_case_acronyms)]
struct CPU {
    x: isize,
    cycles: isize,
}

impl CPU {
    fn new() -> Self {
        Self { x: 1, cycles: 0 }
    }
    fn addx(&mut self, delta: isize) {
        self.x += delta;
        self.cycles += 2;
    }
    fn noop(&mut self) {
        self.cycles += 1;
    }
}
fn part1_evaluate(s: &str) -> isize {
    s.lines()
        .into_iter()
        .scan(CPU::new(), |cpu, line| {
            let cycles_before = cpu.cycles;
            let x_before = cpu.x;
            match line.split_once(' ') {
                Some(("addx", delta)) => cpu.addx(
                    delta
                        .parse::<isize>()
                        .unwrap_or_else(|_| panic!("Bad operand in {line}")),
                ),
                None if line == "noop" => cpu.noop(),
                _ => panic!("Bad line {line}"),
            };
            Some((x_before, cycles_before, cpu.cycles))
        })
        .filter_map(|(x, cycles_before, cycles_after)| {
            ((cycles_after + 20) / 40 != (cycles_before + 20) / 40)
                .then_some(x * ((((cycles_after + 20) / 40) * 40) - 20))
        })
        .sum()
}

fn is_in_sprite(x: isize, sprite_pos: isize) -> bool {
    x + 1 == sprite_pos || x == sprite_pos || sprite_pos + 1 == x
}

fn end_of_line(cycles: isize) -> bool {
    (cycles % 40) == 0
}

fn render(s: &mut String, x: isize, cycles: isize) {
    let c = if is_in_sprite((cycles - 1) % 40, x) { '#' } else { '.' };
    s.push(c);
    if end_of_line(cycles) {
        s.push('\n');
    }
}

fn part2_evaluate(s: &str) -> String {
    let mut crt = String::with_capacity(240);
    let mut cpu = CPU::new();
    for line in s.lines() {
        match line.split_once(' ') {
            Some(("addx", delta)) => {
                render(&mut crt, cpu.x, cpu.cycles + 1);
                render(&mut crt, cpu.x, cpu.cycles + 2);
                cpu.addx(
                    delta
                        .parse::<isize>()
                        .unwrap_or_else(|_| panic!("Bad operand in {line}")),
                );
            }
            None if line == "noop" => {
                render(&mut crt, cpu.x, cpu.cycles + 1);
                cpu.noop();
            }
            _ => panic!("Bad line {line}"),
        };
    }
    crt
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = include_str!("./test_input.txt");
#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1_evaluate(TEST_INPUT_STRING), 13_140);
}

#[test]
fn test_part2() {
    assert_eq!(part2_evaluate(TEST_INPUT_STRING), include_str!("./test_output.txt"));
}

pub fn run() -> Option<(isize, bool, String, bool)> {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string);
    let part2_answer = part2_evaluate(input_string);
    let part2_answer_good = &part2_answer == include_str!("./output.txt");
    Some((part1_answer, part1_answer == 15_680, part2_answer, part2_answer_good))
}
