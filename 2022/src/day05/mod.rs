use std::string::String;
use std::vec::Vec;

type Stack = Vec<char>;
type Stacks = Vec<Stack>;

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    count: usize,
}
type Moves = Vec<Move>;

fn parse_stacks(s: &str) -> Stacks {
    let mut line_iter = s.lines().rev();
    let stack_count = line_iter.next().unwrap().split_whitespace().count();
    let mut stacks = vec![Stack::new(); stack_count];
    for line in line_iter {
        for (stack_index, stack) in stacks.iter_mut().enumerate() {
            let string_index = (stack_index * 4) + 1;
            let _crate = line.chars().nth(string_index).unwrap();
            if !_crate.is_whitespace() {
                stack.push(_crate);
            }
        }
    }
    stacks
}

fn parse_move(s: &str) -> Move {
    let bits = s.split_whitespace().collect::<Vec<_>>();
    assert!(bits.len() == 6, "Wibble - {s} => {bits:?}");
    Move { count: bits[1].parse().unwrap(), from: bits[3].parse().unwrap(), to: bits[5].parse().unwrap() }
}

fn parse_moves(s: &str) -> Moves {
    s.lines().map(parse_move).collect::<_>()
}

fn perform_move_part1(stacks: &mut Stacks, m: &Move) {
    for _ in 0..m.count {
        if let Some(char_to_move) = stacks[m.from - 1].pop() {
            stacks[m.to - 1].push(char_to_move);
        } else {
            panic!("Empty stack!!!!");
        }
    }
}

fn perform_move_part2(stacks: &mut Stacks, m: &Move) {
    let split_pos = stacks[m.from - 1].len() - m.count;
    let mut to_move = stacks[m.from - 1].split_off(split_pos);
    stacks[m.to - 1].append(&mut to_move);
}

fn parse(s: &str) -> (Stacks, Moves) {
    if let Some((stacks, moves)) = s.replace("\r\n", "\n").split_once("\n\n") {
        (parse_stacks(stacks), parse_moves(moves))
    } else {
        (Vec::new(), Vec::new())
    }
}

fn evaluate(s: &str, perform_move: fn(stacks: &mut Stacks, m: &Move)) -> String {
    let (mut stacks, moves) = parse(s);
    for m in moves {
        perform_move(&mut stacks, &m);
    }
    stacks
        .into_iter()
        .map(|s| *s.last().unwrap())
        .collect::<String>()
}

fn part1_evaluate(s: &str) -> String {
    evaluate(s, perform_move_part1)
}

fn part2_evaluate(s: &str) -> String {
    evaluate(s, perform_move_part2)
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = "\
\x20   [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1_evaluate(TEST_INPUT_STRING), "CMZ");
}

#[test]
fn test_part2() {
    assert_eq!(part2_evaluate(TEST_INPUT_STRING), "MCD");
}

pub fn run() -> Option<(String, bool, String, bool)> {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string);
    let part1_good = part1_answer == "SHMSDGZVC";
    let part2_answer = part2_evaluate(input_string);
    let part2_good = part2_answer == "VRZGHDFBQ";
    Some((part1_answer, part1_good, part2_answer, part2_good))
}
