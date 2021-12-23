use std::collections::{hash_map::Entry, HashMap};
use std::path::{Component, Path};

#[derive(Debug, Eq, PartialEq, Clone)]
struct EnvNode {
    circuit: Node,
    value: Option<u16>,
}
impl EnvNode {
    fn reset_value(&mut self) {
        self.value = None;
    }
    fn override_value(&mut self, v: u16) {
        self.value = Some(v);
    }
    fn evaluate(&mut self, env: &mut Env) -> u16 {
        if self.value.is_none() {
            self.value = Some(self.circuit.evaluate(env));
        }
        self.value.unwrap()
    }
}
type Env = HashMap<String, EnvNode>;

fn reset_env_node_values(env: &mut Env) {
    env.values_mut().for_each(|n| n.reset_value());
}

fn override_env_node_value(env: &mut Env, name: &str, v: u16) {
    if let Some(node) = env.get_mut(name) {
        node.override_value(v);
    }
}

fn evaluate_env_node(env: &mut Env, name: &str) -> u16 {
    let e = env.entry(name.to_owned());
    if let Entry::Occupied(o) = e {
        let mut x = o.get().clone();
        let v = x.evaluate(env);
        env.entry(name.to_owned()).and_modify(|n| {
            *n = x;
        });
        v
    } else {
        panic!("Value for wire {} is unobtainable", &name);
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Argument {
    Lit(u16),
    Wire(String),
}
impl Argument {
    fn value(&self, env: &mut Env) -> u16 {
        match *self {
            Argument::Lit(v) => v,
            Argument::Wire(ref name) => evaluate_env_node(env, name),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Node {
    Value(Argument),
    And(Argument, Argument),
    Or(Argument, Argument),
    Not(Argument),
    LShift(Argument, Argument),
    RShift(Argument, Argument),
}
impl Node {
    fn evaluate(&self, env: &mut Env) -> u16 {
        match self {
            Node::Value(arg) => arg.value(env),
            Node::Not(arg) => !arg.value(env),
            Node::And(left, right) => left.value(env) & right.value(env),
            Node::Or(left, right) => left.value(env) | right.value(env),
            Node::LShift(left, right) => left.value(env) << right.value(env),
            Node::RShift(left, right) => left.value(env) >> right.value(env),
        }
    }
}

fn parse_arg(a: &str) -> Argument {
    if let Ok(v) = a.parse::<u16>() {
        Argument::Lit(v)
    } else {
        Argument::Wire(a.to_owned())
    }
}
fn parse_node(n: &str) -> Node {
    if !n.contains(char::is_whitespace) {
        Node::Value(parse_arg(n))
    } else if let Some(arg) = n.strip_prefix("NOT ") {
        Node::Not(parse_arg(arg))
    } else {
        let mut arg_strings = n.split_whitespace();
        let left = parse_arg(
            arg_strings
                .next()
                .unwrap_or_else(|| panic!("Bad node description '{}'", &n)),
        );
        let op = arg_strings
            .next()
            .unwrap_or_else(|| panic!("Bad node description '{}'", &n));
        let right = parse_arg(
            arg_strings
                .next()
                .unwrap_or_else(|| panic!("Bad node description '{}'", &n)),
        );
        match op {
            "AND" => Node::And(left, right),
            "OR" => Node::Or(left, right),
            "LSHIFT" => Node::LShift(left, right),
            "RSHIFT" => Node::RShift(left, right),
            _ => panic!("Bad node description '{}'", &n),
        }
    }
}

fn parse(input: &str) -> Env {
    input
        .lines()
        .map(|l| {
            let (expr, dest) = l.split_once(" -> ").unwrap();
            (dest.to_owned(), EnvNode { circuit: parse_node(expr), value: None })
        })
        .collect::<Env>()
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> u16 {
        let mut circuit = parse(input);
        evaluate_env_node(&mut circuit, &"a".to_owned())
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        let mut circuit = parse(input_string);

        assert_eq!(evaluate_env_node(&mut circuit, "d"), 72);
        assert_eq!(evaluate_env_node(&mut circuit, "e"), 507);
        assert_eq!(evaluate_env_node(&mut circuit, "f"), 492);
        assert_eq!(evaluate_env_node(&mut circuit, "g"), 114);
        assert_eq!(evaluate_env_node(&mut circuit, "h"), 65412);
        assert_eq!(evaluate_env_node(&mut circuit, "i"), 65079);
        assert_eq!(evaluate_env_node(&mut circuit, "x"), 123);
        assert_eq!(evaluate_env_node(&mut circuit, "y"), 456);
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> u16 {
        let mut circuit = parse(input);
        let a_value = evaluate_env_node(&mut circuit, "a");
        reset_env_node_values(&mut circuit);
        override_env_node_value(&mut circuit, "b", a_value);
        evaluate_env_node(&mut circuit, "a")
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
    assert_eq!(part1_ans, 16076);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 2797);
}
