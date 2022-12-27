use std::array::TryFromSliceError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Op {
    Plus,
    Minus,
    Mult,
    Div,
    Equal,
}

impl Op {
    fn perform(self, l: i64, r: i64) -> i64 {
        match self {
            Op::Plus => l + r,
            Op::Minus => l - r,
            Op::Mult => l * r,
            Op::Div => l / r,
            Op::Equal => (l == r) as i64,
        }
    }

    fn calculate_operand(self, result: i64, left: Option<i64>, right: Option<i64>) -> i64 {
        match (left, self, right) {
            (None, Op::Plus, Some(r)) => result - r,
            (None, Op::Minus, Some(r)) => result + r,
            (None, Op::Mult, Some(r)) => result / r,
            (None, Op::Div, Some(r)) => result * r,
            (None, Op::Equal, Some(r)) => {
                if result == 1 {
                    r
                } else {
                    panic!("Can't handle inequality")
                }
            }
            (Some(l), Op::Plus, None) => result - l,
            (Some(l), Op::Minus, None) => l - result,
            (Some(l), Op::Mult, None) => result / l,
            (Some(l), Op::Div, None) => l / result,
            (Some(l), Op::Equal, None) => {
                if result == 1 {
                    l
                } else {
                    panic!("Can't handle inequality")
                }
            }
            _ => panic!("Bad expression {self:?} with left={left:?}, right={right:?}, result={result}!!!!"),
        }
    }
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('+') => Ok(Op::Plus),
            Some('-') => Ok(Op::Minus),
            Some('*') => Ok(Op::Mult),
            Some('/') => Ok(Op::Div),
            Some('=') => Ok(Op::Equal),
            _ => Err("Bad operator".to_owned()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Value {
    Number(i64),
    Operation { left_monkey: Id, op: Op, right_monkey: Id },
}

impl Value {
    fn get_value(&self, monkeys: &Monkeys) -> i64 {
        match self {
            Value::Number(v) => *v,
            Value::Operation { left_monkey, op, right_monkey } => {
                op.perform(monkeys.get_value(*left_monkey), monkeys.get_value(*right_monkey))
            }
        }
    }
}
impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_ascii_whitespace().count() {
            1 => s
                .split_ascii_whitespace()
                .next()
                .ok_or(format!("Bad expression in {s}"))
                .and_then(|s| s.parse::<i64>().map_err(|e| e.to_string()))
                .map(Value::Number),
            3 => {
                let mut tokens = s.split_ascii_whitespace();
                let left_monkey = tokens
                    .next()
                    .ok_or(format!("Bad Id in {s}"))
                    .and_then(Id::from_str)?;
                let op = tokens
                    .next()
                    .ok_or(format!("Bad operator in {s}"))
                    .and_then(Op::from_str)?;
                let right_monkey = tokens
                    .next()
                    .ok_or(format!("Bad Id in {s}"))
                    .and_then(Id::from_str)?;
                Ok(Value::Operation { left_monkey, op, right_monkey })
            }
            _ => Err(format!("Bad expression in {s}"))?,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Id(u32);

impl FromStr for Id {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Id(u32::from_be_bytes(
            s.as_bytes()[0..4]
                .try_into()
                .map_err(|e: TryFromSliceError| e.to_string())?,
        )))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Monkey {
    id: Id,
    value: Value,
}

impl Monkey {
    fn uses(&self, monkeys: &Monkeys, monkey: Id) -> bool {
        self.id == monkey
            || match self.value {
                Value::Operation { left_monkey, op: _, right_monkey } => {
                    monkeys
                        .get_monkey(left_monkey)
                        .map_or(false, |m| m.uses(monkeys, monkey))
                        || monkeys
                            .get_monkey(right_monkey)
                            .map_or(false, |m| m.uses(monkeys, monkey))
                }
                Value::Number(_) => false,
            }
    }
    fn get_value(&self, monkeys: &Monkeys) -> i64 {
        self.value.get_value(monkeys)
    }
}

struct Monkeys {
    monkeys: HashMap<Id, Monkey>,
    cached_values: RefCell<HashMap<Id, i64>>,
}

impl Monkeys {
    fn get_monkey(&self, id: Id) -> Option<&Monkey> {
        self.monkeys.get(&id)
    }
    fn get_value(&self, id: Id) -> i64 {
        {
            if let Some(value) = self.cached_values.borrow().get(&id) {
                return *value;
            }
        }
        let Some(monkey) = self.monkeys.get(&id) else {
            panic!("Bad monkey id {id:?}");
        };
        let value = monkey.get_value(self);
        self.cached_values.borrow_mut().insert(id, value);
        value
    }
}

fn parse(s: &str) -> Monkeys {
    Monkeys {
        monkeys: s
            .lines()
            .map(|line| {
                let (id, expression) = line
                    .split_once(':')
                    .ok_or_else(|| format!("Bad line {line}"))?;
                let id = Id::from_str(id)?;
                let expression = Value::from_str(expression)?;
                Ok((id, Monkey { id, value: expression }))
            })
            .collect::<Result<HashMap<_, _>, _>>()
            .unwrap_or_else(|e: String| panic!("{e}")),
        cached_values: RefCell::new(HashMap::new()),
    }
}

fn part1_evaluate(s: &str) -> i64 {
    let monkeys = parse(s);
    let root_id = Id::from_str("root").unwrap_or_else(|e| panic!("{e}"));
    monkeys.get_value(root_id)
}

fn calculate_human_value(monkeys: &Monkeys, top_monkey: &Monkey, human_id: Id, expected_result: i64) -> i64 {
    if top_monkey.id == human_id {
        return expected_result;
    }
    let Value::Operation { left_monkey, op, right_monkey } = top_monkey.value else {
        panic!("top_monkey {top_monkey:?} isn't an expression!");
    };
    let left_monkey = monkeys.get_monkey(left_monkey).unwrap();
    let right_monkey = monkeys.get_monkey(right_monkey).unwrap();
    let (left_is_human_side, human_side, non_human_side) = if left_monkey.uses(monkeys, human_id) {
        (true, left_monkey, right_monkey)
    } else {
        (false, right_monkey, left_monkey)
    };
    let non_human_side_operand = non_human_side.get_value(monkeys);
    let next_result = if left_is_human_side {
        op.calculate_operand(expected_result, None, Some(non_human_side_operand))
    } else {
        op.calculate_operand(expected_result, Some(non_human_side_operand), None)
    };
    calculate_human_value(monkeys, human_side, human_id, next_result)
}

fn part2_evaluate(s: &str) -> i64 {
    let mut monkeys = parse(s);
    let root_id = Id::from_str("root").unwrap_or_else(|e| panic!("{e}"));
    let my_id = Id::from_str("humn").unwrap_or_else(|e| panic!("{e}"));
    {
        let root = monkeys.monkeys.get_mut(&root_id).unwrap();
        let Value::Operation { left_monkey:_, ref mut op, right_monkey:_ } = root.value else {
            panic!("root isn't an expression!")
        };
        *op = Op::Equal;
    }
    let root = monkeys.monkeys.get(&root_id).unwrap();
    calculate_human_value(&monkeys, root, my_id, 1)
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1_evaluate(TEST_INPUT_STRING), 152);
}

#[test]
fn test_part2() {
    assert_eq!(part2_evaluate(TEST_INPUT_STRING), 301);
}

pub fn run() -> Option<(i64, bool, i64, bool)> {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string);
    let part2_answer = part2_evaluate(input_string);
    Some((part1_answer, part1_answer == 291_425_799_367_130, part2_answer, part2_answer == 3_219_579_395_609))
}
