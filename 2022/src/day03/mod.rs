use itertools::Itertools;

fn get_priority(c: u8) -> usize {
    (match c {
        b'a'..=b'z' => c - b'a' + 1,
        b'A'..=b'Z' => c - b'A' + 27,
        _ => panic!("Bad character {c}"),
    }) as usize
}

fn get_priority_set(s: &str) -> u64 {
    let mut set: u64 = 0;
    s.as_bytes().iter().for_each(|b| {
        set |= 1u64 << get_priority(*b);
    });
    set
}

fn part1_evaluate(s: &str) -> usize {
    s.lines()
        .map(|line| {
            let (comp1, comp2) = line.split_at(line.len() / 2);
            let s1 = get_priority_set(comp1);
            let s2 = get_priority_set(comp2);
            let s_inter = s1 & s2;
            assert_eq!(s_inter.count_ones(), 1);
            s_inter.trailing_zeros() as usize
        })
        .sum()
}

fn part2_evaluate(s: &str) -> usize {
    s.lines()
        .tuples::<(_, _, _)>()
        .map(|(l1, l2, l3)| {
            let s1 = get_priority_set(l1);
            let s2 = get_priority_set(l2);
            let s3 = get_priority_set(l3);
            let s_inter = s1 & s2 & s3;
            assert_eq!(s_inter.count_ones(), 1);
            s_inter.trailing_zeros() as usize
        })
        .sum()
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1_evaluate(TEST_INPUT_STRING), 157);
}

#[test]
fn test_part2() {
    assert_eq!(part2_evaluate(TEST_INPUT_STRING), 70);
}

pub fn run() -> Option<(usize, bool, usize, bool)> {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string);
    let part2_answer = part2_evaluate(input_string);
    Some((part1_answer, part1_answer == 7917, part2_answer, part2_answer == 2585))
}
