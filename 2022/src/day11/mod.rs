use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Item {
    worry_level: usize,
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

impl Operand {
    fn value(&self, old: usize) -> usize {
        match self {
            Operand::Old => old,
            Operand::Value(v) => *v,
        }
    }
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

impl Inspection {
    fn inspect(&self, old: usize) -> usize {
        match self {
            Inspection::Add(l, r) => l.value(old) + r.value(old),
            Inspection::Multiply(l, r) => l.value(old) * r.value(old),
        }
    }
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
#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<Item>,
    inspection: Inspection,
    throw_to: ThrowTo,
    inspection_count: usize,
    worry_div_factor:usize,
    worry_mod_factor:usize,
}

impl Monkey {
    fn inspect(&mut self, item: &mut Item) {
        item.worry_level = self.inspection.inspect(item.worry_level);
        self.inspection_count += 1;
    }
    fn process_item(&mut self) -> Option<(usize, Item)> {
        self.items.pop().map(|mut item| {
            self.inspect(&mut item);
            item.worry_level = (item.worry_level / self.worry_div_factor) % self.worry_mod_factor;
            let throwing_to = if item.worry_level % self.throw_to.divisor == 0 {
                self.throw_to.if_true
            } else {
                self.throw_to.if_false
            };
            (throwing_to, item)
        })
    }
    fn process_items(&mut self, throws: &mut Vec<(usize, Item)>) {
        // let mut throws = Vec::with_capacity(self.items.len());
        while let Some(throw) = self.process_item() {
            throws.push(throw);
        }
        // throws
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
            .and_then(|s| str::parse::<usize>(s.trim_end_matches(':')).map_err(|_| Self::Err::Monkey));

        let items = line
            .next()
            .and_then(|s| s.trim().strip_prefix("Starting items:"))
            .ok_or(Self::Err::Monkey)
            .and_then(|items| {
                items
                    .split(',')
                    .map(|num| {
                        num.trim().parse::<usize>()
                            .map(|worry_level| Item { worry_level })
                            .map_err(|_| Self::Err::Monkey)
                    })
                    .collect::<Result<Vec<_>, _>>()
            });
        let operation = line
            .next()
            .and_then(|s| s.trim().strip_prefix("Operation: new = "))
            .ok_or(Self::Err::Monkey)
            .and_then(Inspection::from_str);

        let divisor = line
            .next()
            .and_then(|s| s.trim().strip_prefix("Test: divisible by "))
            .ok_or(Self::Err::Monkey)
            .and_then(|s| s.parse::<usize>().map_err(|_| Self::Err::Monkey));

        let if_true = line
            .next()
            .and_then(|s| s.trim().strip_prefix("If true: throw to monkey "))
            .ok_or(Self::Err::Monkey)
            .and_then(|s| s.parse::<usize>().map_err(|_| Self::Err::Monkey));

        let if_false = line
            .next()
            .and_then(|s| s.trim().strip_prefix("If false: throw to monkey "))
            .ok_or(Self::Err::Monkey)
            .and_then(|s| s.parse::<usize>().map_err(|_| Self::Err::Monkey));

        if let (Ok(id), Ok(items), Ok(inspection), Ok(divisor), Ok(if_true), Ok(if_false)) =
            (id, items, operation, divisor, if_true, if_false)
        {
            Ok(Self { id, items, inspection, throw_to: ThrowTo { divisor, if_true, if_false }, inspection_count: 0, worry_div_factor: 3, worry_mod_factor: usize::MAX})
        } else {
            Err(Self::Err::Monkey)
        }
    }
}

fn parse(s: &str) -> Vec<Monkey> {
    s.replace("\r\n", "\n")
        .split("\n\n")
        .map(|lines| Monkey::from_str(lines).unwrap_or_else(|_| panic!("Bad monkey in {lines}")))
        .collect::<Vec<_>>()
}

fn part1_evaluate(s: &str) -> usize {
    let mut monkeys = parse(s);

    if !monkeys.iter().enumerate().all(|(i, m)| i==m.id) {
        panic!("Monkey IDs don't match indices");
    }

    let mut throws = Vec::with_capacity(100);
    for _cycle in 0..20 {
        for i in 0..monkeys.len() {
            monkeys[i].process_items(&mut throws);
            for (throwing_to, item) in &throws {
                monkeys[*throwing_to].items.push(*item);
            }
            throws.clear();
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
    let mut monkeys = parse(s);
    let worry_mod_factor = monkeys.iter().map(|m| m.throw_to.divisor).product::<usize>();
    monkeys.iter_mut().for_each(|m| {m.worry_div_factor=1;m.worry_mod_factor=worry_mod_factor;});

    if !monkeys.iter().enumerate().all(|(i, m)| i==m.id) {
        panic!("Monkey IDs don't match indices");
    }

    let mut throws = Vec::with_capacity(100);
    for _cycle in 0..10_000 {
        for i in 0..monkeys.len() {
            monkeys[i].process_items(&mut throws);
            for (throwing_to, item) in &throws {
                monkeys[*throwing_to].items.push(*item);
            }
            throws.clear();
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
