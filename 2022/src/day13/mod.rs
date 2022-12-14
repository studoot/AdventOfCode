use itertools::Itertools;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Value {
    List(Vec<Value>),
    Integer(usize),
}

mod parser {
    use super::Value;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::u32;
    use nom::combinator::map;
    use nom::multi::separated_list0;
    use nom::sequence::delimited;
    use nom::IResult;
    pub(super) fn value(s: &str) -> IResult<&str, Value> {
        alt((map(u32, |v| Value::Integer(v as usize)), list))(s)
    }

    pub(super) fn list(s: &str) -> IResult<&str, Value> {
        delimited(tag("["), map(separated_list0(tag(","), value), Value::List), tag("]"))(s)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum IsLower {
    Left,
    Equal,
    Right,
}

impl From<usize> for Value {
    fn from(v: usize) -> Self {
        Self::Integer(v)
    }
}

impl From<Vec<usize>> for Value {
    fn from(v: Vec<usize>) -> Self {
        Self::List(v.into_iter().map(Self::from).collect::<Vec<_>>())
    }
}

impl Value {
    fn compare(&self, rhs: &Self) -> IsLower {
        match (self, rhs) {
            (Self::Integer(l), Self::Integer(r)) => match l.cmp(r) {
                std::cmp::Ordering::Less => IsLower::Left,
                std::cmp::Ordering::Equal => IsLower::Equal,
                std::cmp::Ordering::Greater => IsLower::Right,
            },
            (Self::Integer(l), r @ Self::List(_)) => {
                let l = Self::from(vec![*l]);
                l.compare(r)
            }
            (l @ Self::List(_), Self::Integer(r)) => {
                let r = Self::from(vec![*r]);
                l.compare(&r)
            }
            (Self::List(l), Self::List(r)) => l
                .iter()
                .zip(r.iter())
                .find_map(|(l, r)| {
                    let cmp_res = l.compare(r);
                    (cmp_res != IsLower::Equal).then_some(cmp_res)
                })
                .unwrap_or(match l.len().cmp(&r.len()) {
                    std::cmp::Ordering::Less => IsLower::Left,
                    std::cmp::Ordering::Equal => IsLower::Equal,
                    std::cmp::Ordering::Greater => IsLower::Right,
                }),
        }
    }
}

fn parse(s: &str) -> Vec<(Value, Value)> {
    s.replace("\r\n", "\n")
        .split("\n\n")
        .map(|lines| {
            lines.split_once('\n').map(|(l_s, r_s)| {
                let (_, l) = parser::list(l_s).unwrap_or_else(|e| panic!("Error {e:?} in '{l_s}"));
                let (_, r) = parser::list(r_s).unwrap_or_else(|e| panic!("Error {e:?} in '{r_s}"));
                (l, r)
            })
        })
        .collect::<Option<Vec<_>>>()
        .unwrap()
}

fn part1_evaluate(packets: &[(Value, Value)]) -> usize {
    packets
        .iter()
        .enumerate()
        .map(|(pos, (l, r))| if l.compare(r) == IsLower::Left { pos + 1 } else { 0 })
        .sum::<usize>()
}

fn part2_evaluate(packets: Vec<(Value, Value)>) -> usize {
    let recv_packets = packets
        .into_iter()
        .flat_map(|(l, r)| [l, r])
        .collect::<Vec<_>>();
    let dividers = vec![
        Value::List(vec![Value::List(vec![Value::from(2)])]),
        Value::List(vec![Value::List(vec![Value::from(6)])]),
    ];

    let all_packets = recv_packets
        .iter()
        .chain(dividers.iter())
        .sorted_by(|l, r| match l.compare(r) {
            IsLower::Left => Ordering::Less,
            IsLower::Equal => Ordering::Equal,
            IsLower::Right => Ordering::Greater,
        })
        .collect::<Vec<_>>();

    let p1 = 1 + all_packets
        .iter()
        .position(|i| i.compare(&dividers[0]) == IsLower::Equal)
        .unwrap();
    let p2 = 1 + all_packets
        .iter()
        .position(|i| i.compare(&dividers[1]) == IsLower::Equal)
        .unwrap();

    p1 * p2
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = "\
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

#[test]
fn test_compare() {
    assert_eq!(Value::from(vec![1, 1, 3, 1, 1]).compare(&Value::from(vec![1, 1, 5, 1, 1])), IsLower::Left);
    assert_eq!(Value::from(vec![7, 7, 7, 7]).compare(&Value::from(vec![7, 7, 7])), IsLower::Right);
    assert_eq!(Value::from(vec![]).compare(&Value::from(vec![3])), IsLower::Left);
}

#[test]
fn test_part1() {
    let packets = parse(TEST_INPUT_STRING);
    assert_eq!(part1_evaluate(&packets), 13);
}

#[test]
fn test_part2() {
    let packets = parse(TEST_INPUT_STRING);
    assert_eq!(part2_evaluate(packets), 140);
}

pub fn run() -> Option<(usize, bool, usize, bool)> {
    let input_string = include_str!("./input.txt");
    let packets = parse(input_string);
    let part1_answer = part1_evaluate(&packets);
    let part2_answer = part2_evaluate(packets);
    Some((part1_answer, part1_answer == 5198, part2_answer, part2_answer == 22344))
}
