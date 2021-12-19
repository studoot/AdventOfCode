use itertools::Itertools;
use std::fmt::Display;
use std::ops::AddAssign;
use std::path::{Component, Path};

#[derive(PartialEq, Eq, Debug, Clone)]
enum Digit {
    Lit(usize),
    Number(Box<Number>),
}
impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Digit::Lit(v) => f.write_fmt(format_args!("{}", *v)),
            Digit::Number(n) => f.write_fmt(format_args!("{}", *n)),
        }
    }
}
impl Digit {
    fn magnitude(&self) -> usize {
        match &self {
            Digit::Lit(a) => *a,
            Digit::Number(b_num) => b_num.magnitude(),
        }
    }
    fn split(&mut self) -> bool {
        match &self {
            Digit::Lit(a) if *a >= 10 => {
                *self = Digit::Number(Box::new(Number(Digit::Lit(*a / 2), Digit::Lit((*a + 1) / 2))));
                true
            }
            _ => false,
        }
    }
}
#[derive(PartialEq, Eq, Debug, Clone)]
struct Number(Digit, Digit);
impl AddAssign for Number {
    fn add_assign(&mut self, rhs: Number) {
        *self = Number(Digit::Number(Box::new(self.clone())), Digit::Number(Box::new(rhs)));
        self.reduce()
    }
}
impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{},{}]", &self.0, &self.1))
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum ReduceAction {
    DidSplit,
    DidExplode(Option<usize>, Option<usize>),
    Nothing,
}
impl Number {
    fn magnitude(&self) -> usize {
        &self.0.magnitude() * 3 + &self.1.magnitude() * 2
    }
    fn add_to_right(&mut self, v: usize) {
        match &mut self.1 {
            Digit::Lit(a) => self.1 = Digit::Lit(*a + v),
            Digit::Number(b_num) => b_num.add_to_right(v),
        }
    }
    fn add_to_left(&mut self, v: usize) {
        match &mut self.0 {
            Digit::Lit(a) => self.0 = Digit::Lit(*a + v),
            Digit::Number(b_num) => b_num.add_to_left(v),
        }
    }
    fn add_to_next_right(&mut self, v: usize) {
        match &mut self.1 {
            Digit::Lit(a) => self.1 = Digit::Lit(*a + v),
            Digit::Number(b_num) => b_num.add_to_left(v),
        }
    }
    fn add_to_next_left(&mut self, v: usize) {
        match &mut self.0 {
            Digit::Lit(a) => self.0 = Digit::Lit(*a + v),
            Digit::Number(b_num) => b_num.add_to_right(v),
        }
    }
    fn check_for_explode(&mut self, nesting_level: u8) -> ReduceAction {
        if nesting_level == 4 {
            // println!("Explode {}", &self);
            return match (&self.0, &self.1) {
                (&Digit::Lit(left), &Digit::Lit(right)) => ReduceAction::DidExplode(Some(left), Some(right)),
                _ => unreachable!(),
            };
        }
        if let Digit::Number(n) = &mut self.0 {
            if let ReduceAction::DidExplode(l, r) = n.check_for_explode(nesting_level + 1) {
                if nesting_level == 3 {
                    self.0 = Digit::Lit(0);
                }
                if let Some(r) = r {
                    self.add_to_next_right(r);
                    return ReduceAction::DidExplode(l, None);
                }
                return ReduceAction::DidExplode(l, r);
            }
        }
        if let Digit::Number(n) = &mut self.1 {
            if let ReduceAction::DidExplode(l, r) = n.check_for_explode(nesting_level + 1) {
                if nesting_level == 3 {
                    self.1 = Digit::Lit(0);
                }
                if let Some(l) = l {
                    self.add_to_next_left(l);
                    return ReduceAction::DidExplode(None, r);
                }
                return ReduceAction::DidExplode(l, r);
            }
        }
        ReduceAction::Nothing
    }
    fn check_for_split(&mut self) -> ReduceAction {
        match &mut self.0 {
            d @ Digit::Lit(_) => {
                if d.split() {
                    // println!("Split");
                    return ReduceAction::DidSplit;
                }
            }
            Digit::Number(n) => {
                if n.check_for_split() == ReduceAction::DidSplit {
                    return ReduceAction::DidSplit;
                }
            }
        }
        match &mut self.1 {
            d @ Digit::Lit(_) => {
                if d.split() {
                    // println!("Split");
                    return ReduceAction::DidSplit;
                }
            }
            Digit::Number(n) => {
                if n.check_for_split() == ReduceAction::DidSplit {
                    return ReduceAction::DidSplit;
                }
            }
        }
        ReduceAction::Nothing
    }

    fn reduce(&mut self) {
        loop {
            // println!("{}", &self);
            if self.check_for_explode(0) == ReduceAction::Nothing && self.check_for_split() == ReduceAction::Nothing {
                // println!("======================================================");
                break;
            }
        }
    }
}

fn parse_digit(s: &str) -> (Digit, &str) {
    if s.starts_with('[') {
        let (n, rest) = parse_number_with_remainder(s);
        (Digit::Number(Box::new(n)), rest)
    } else {
        let (d, rest) = s.split_at(
            s.find(|c| !char::is_digit(c, 10))
                .unwrap_or_else(|| panic!("expected {} @ {}", "digit", s)),
        );
        (Digit::Lit(d.parse::<usize>().unwrap()), rest)
    }
}
fn parse_number_with_remainder(line: &str) -> (Number, &str) {
    let number = line
        .strip_prefix('[')
        .unwrap_or_else(|| panic!("expected {} @ {}", '[', line));
    let (d1, rest) = parse_digit(number);
    let rest = rest
        .strip_prefix(',')
        .unwrap_or_else(|| panic!("expected {} @ {}", ',', rest));
    let (d2, rest) = parse_digit(rest);
    let rest = rest
        .strip_prefix(']')
        .unwrap_or_else(|| panic!("expected {} @ {}", ']', rest));
    (Number(d1, d2), rest)
}
fn parse_number(line: &str) -> Number {
    parse_number_with_remainder(line).0
}

fn parse(input: &str) -> Vec<Number> {
    input.lines().map(parse_number).collect::<Vec<_>>()
}

mod part1 {
    use super::*;

    pub(crate) fn run(input: &str) -> Number {
        let nums = parse(input);
        let mut i = nums.into_iter();
        let mut a = i.next().unwrap();
        // println!("run:a={}", a);
        for n in i {
            // println!("run:a + {}", n);
            a += n;
            // println!(" = {}", a);
        }
        a
    }

    #[test]
    fn test_run1() {
        let input_string = include_str!("../test1.txt");
        assert_eq!(parse_number("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"), run(input_string))
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(4140, run(input_string).magnitude())
    }

    #[test]
    fn test_run2() {
        let mut a = parse_number("[1,1]");
        a += parse_number("[2,2]");
        a += parse_number("[3,3]");
        a += parse_number("[4,4]");
        assert_eq!(a, parse_number("[[[[1,1],[2,2]],[3,3]],[4,4]]"));
        a += parse_number("[5,5]");
        assert_eq!(a, parse_number("[[[[3,0],[5,3]],[4,4]],[5,5]]"));
        a += parse_number("[6,6]");
        assert_eq!(a, parse_number("[[[[5,0],[7,4]],[5,5]],[6,6]]"));
    }

    #[test]
    fn test_reduce() {
        let mut n = parse_number("[[[[[9,8],1],2],3],4]");
        n.reduce();
        assert_eq!(n, parse_number("[[[[0,9],2],3],4]"));
        let mut n = parse_number("[7,[6,[5,[4,[3,2]]]]]");
        n.reduce();
        assert_eq!(n, parse_number("[7,[6,[5,[7,0]]]]"));
        let mut n = parse_number("[[6,[5,[4,[3,2]]]],1]");
        n.reduce();
        assert_eq!(n, parse_number("[[6,[5,[7,0]]],3]"));
        let mut n = parse_number("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        n.reduce();
        assert_eq!(n, parse_number("[[3,[2,[8,0]]],[9,[5,[7,0]]]]"));
        let mut n = parse_number("[[[[4,0],[5,0]],[[[4,5],0],0]],0]");
        n.reduce();
        assert_eq!(n, parse_number("[[[[4,0],[5,4]],[[0,5],0]],0]"));
    }

    #[test]
    fn test_add() {
        let mut added = parse_number("[[[[4,3],4],4],[7,[[8,4],9]]]");
        added += parse_number("[1,1]");
        assert_eq!(added, parse_number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));

        let mut added = parse_number("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
        added += parse_number("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");
        assert_eq!(added, parse_number("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"));
    }
    #[test]
    fn test_magnitude() {
        assert_eq!(parse_number("[[1,2],[[3,4],5]]").magnitude(), 143);
        assert_eq!(parse_number("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(), 1384);
        assert_eq!(parse_number("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(), 445);
        assert_eq!(parse_number("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(), 791);
        assert_eq!(parse_number("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(), 1137);
        assert_eq!(parse_number("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(), 3488);
        assert_eq!(parse_number("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]").magnitude(), 4140);
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse_number_with_remainder("[3,4]"), (Number(Digit::Lit(3), Digit::Lit(4)), ""));
        assert_eq!(
            parse_number_with_remainder("[3,[4,5]]"),
            (Number(Digit::Lit(3), Digit::Number(Box::new(Number(Digit::Lit(4), Digit::Lit(5))))), "")
        );
    }
}

mod part2 {
    use super::*;

    pub(crate) fn run(input: &str) -> usize {
        let nums = parse(input);
        nums.iter()
            .cartesian_product(nums.iter())
            .map(|(a, b)| {
                let mut sa = a.clone();
                let sb = b.clone();
                sa += sb;
                sa.magnitude()
            })
            .max()
            .unwrap()
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(3993, run(input_string))
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
    let part1_ans = part1::run(input_string).magnitude();
    println!("Day {} part 1 - {} - took {} milliseconds.", day_number, part1_ans, now.elapsed().as_millis());
    assert_eq!(part1_ans, 4088);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 4536);
}
