use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Item {
    worry_level: usize,
    owner: usize,
}

#[derive(Debug)]
enum ParseErrors {
    Operand,
    Inspection,
    Monkey,
}

#[derive(Debug)]
enum Operand {
    Old,
    Value(usize),
}

impl FromStr for Operand {
    type Err = ParseErrors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "old" {
            Ok(Operand::Old)
        } else {
            s.parse::<usize>()
                .map_err(|_| Self::Err::Operand)
                .map(Operand::Value)
        }
    }
}

#[derive(Debug)]
enum Inspection {
    Add(Operand, Operand),
    Multiply(Operand, Operand),
}

impl FromStr for Inspection {
    type Err = ParseErrors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let left = tokens
            .next()
            .ok_or(Self::Err::Inspection)
            .and_then(Operand::from_str);
        let op = tokens.next().ok_or(Self::Err::Inspection);
        let right = tokens
            .next()
            .ok_or(Self::Err::Inspection)
            .and_then(Operand::from_str);
        if let (Ok(left), Ok(op), Ok(right)) = (left, op, right) {
            match op {
                "+" => Ok(Self::Add(left, right)),
                "*" => Ok(Self::Multiply(left, right)),
                _ => Err(Self::Err::Inspection),
            }
        } else {
            Err(Self::Err::Inspection)
        }
    }
}

#[derive(Debug)]
struct ThrowTo {
    divisor: usize,
    if_true: usize,
    if_false: usize,
}

impl ThrowTo {
    fn throw(&self, item: &Item) -> usize {
        if item.worry_level % self.divisor == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

struct Monkey {
    id: usize,
    initial_items: Vec<Item>,
    inspection: Box<dyn Fn(usize) -> usize>,
    throw_to: ThrowTo,
    inspection_count: usize,
    // worry_div_factor: usize,
    // worry_mod_factor: usize,
    deworrier: Box<dyn Fn(usize) -> usize>,
}

impl Monkey {
    fn process_item(&mut self, item: &mut Item) {
        self.inspection_count += 1;
        item.worry_level = (self.deworrier)((self.inspection)(item.worry_level));
        item.owner = self.throw_to.throw(item);
    }
    fn process_items(&mut self, items: &mut Vec<Item>) {
        for item in items {
            if item.owner == self.id {
                self.process_item(item);
            }
        }
    }
}

impl FromStr for Monkey {
    type Err = ParseErrors;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line = s.lines();
        let id = line
            .next()
            .and_then(|s| s.trim().strip_prefix("Monkey "))
            .ok_or(Self::Err::Monkey)
            .and_then(|s| str::parse::<usize>(s.trim_end_matches(':')).map_err(|_| Self::Err::Monkey))?;

        let initial_items = line
            .next()
            .and_then(|s| s.trim().strip_prefix("Starting items:"))
            .ok_or(Self::Err::Monkey)
            .and_then(|items| {
                items
                    .split(',')
                    .map(|num| {
                        num.trim()
                            .parse::<usize>()
                            .map(|worry_level| Item { worry_level, owner: id })
                            .map_err(|_| Self::Err::Monkey)
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?;
        let inspection = line
            .next()
            .and_then(|s| s.trim().strip_prefix("Operation: new = "))
            .ok_or(Self::Err::Monkey)
            .and_then(Inspection::from_str)?;

        let inspection: Box<dyn Fn(usize) -> usize> = match &inspection {
            Inspection::Add(l, r) => match (l, r) {
                (&Operand::Old, &Operand::Old) => Box::new(|old| 2 * old),
                (&Operand::Old, &Operand::Value(v)) => Box::new(move |old| old + v),
                (&Operand::Value(v), &Operand::Old) => Box::new(move |old| v + old),
                (&Operand::Value(v1), &Operand::Value(v2)) => Box::new(move |_old| v1 + v2),
            },
            Inspection::Multiply(l, r) => match (l, r) {
                (&Operand::Old, &Operand::Old) => Box::new(|old| old * old),
                (&Operand::Old, &Operand::Value(v)) => Box::new(move |old| old * v),
                (&Operand::Value(v), &Operand::Old) => Box::new(move |old| v * old),
                (&Operand::Value(v1), &Operand::Value(v2)) => Box::new(move |_old| v1 * v2),
            },
        };

        let divisor = line
            .next()
            .and_then(|s| s.trim().strip_prefix("Test: divisible by "))
            .ok_or(Self::Err::Monkey)
            .and_then(|s| s.parse::<usize>().map_err(|_| Self::Err::Monkey))?;

        let if_true = line
            .next()
            .and_then(|s| s.trim().strip_prefix("If true: throw to monkey "))
            .ok_or(Self::Err::Monkey)
            .and_then(|s| s.parse::<usize>().map_err(|_| Self::Err::Monkey))?;

        let if_false = line
            .next()
            .and_then(|s| s.trim().strip_prefix("If false: throw to monkey "))
            .ok_or(Self::Err::Monkey)
            .and_then(|s| s.parse::<usize>().map_err(|_| Self::Err::Monkey))?;

        Ok(Self {
            id,
            initial_items,
            inspection,
            throw_to: ThrowTo { divisor, if_true, if_false },
            inspection_count: 0,
            deworrier: Box::new(|old| old / 3),
        })
    }
}

fn parse(s: &str) -> (Vec<Monkey>, Vec<Item>) {
    let monkeys = s
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|lines| Monkey::from_str(lines).unwrap_or_else(|_| panic!("Bad monkey in {lines}")))
        .collect::<Vec<_>>();
    let items = monkeys
        .iter()
        .flat_map(|m| m.initial_items.clone())
        .collect::<Vec<_>>();
    (monkeys, items)
}

fn part1_evaluate(s: &str) -> usize {
    let (mut monkeys, mut items) = parse(s);

    for _ in 0..20 {
        for m in &mut monkeys {
            m.process_items(&mut items);
        }
    }
    monkeys
        .into_iter()
        .map(|m| m.inspection_count)
        .sorted()
        .rev()
        .take(2)
        .product::<usize>()
}

fn part2_evaluate(s: &str) -> usize {
    let (mut monkeys, mut items) = parse(s);
    let worry_mod_factor = monkeys
        .iter()
        .map(|m| m.throw_to.divisor)
        .product::<usize>();
    monkeys.iter_mut().for_each(|m| {
        m.deworrier = Box::new(move |old| old % worry_mod_factor);
    });

    for _ in 0..10_000 {
        for m in &mut monkeys {
            m.process_items(&mut items);
        }
    }

    monkeys
        .into_iter()
        .map(|m| m.inspection_count)
        .sorted()
        .rev()
        .take(2)
        .product::<usize>()
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1_evaluate(TEST_INPUT_STRING), 10605);
}

#[test]
fn test_part2() {
    assert_eq!(part2_evaluate(TEST_INPUT_STRING), 2713310158);
}

pub fn run() -> Option<(usize, bool, usize, bool)> {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string);
    let part2_answer = part2_evaluate(input_string);
    Some((part1_answer, part1_answer == 78678, part2_answer, part2_answer == 15333249714))
}
