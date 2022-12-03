use std::collections::HashSet;
use itertools::Itertools;

fn get_priority(c: u8) -> usize {
    (match c {
        b'a'..=b'z' => c - b'a' + 1,
        b'A'..=b'Z' => c - b'A' + 27,
        _ => panic!("Bad character {c}"),
    }) as usize
}

fn part1_evaluate(s: &str) -> usize {
    s.lines()
        .map(|line| {
            let (comp1, comp2) = line.split_at(line.len() / 2);
            let s1 = comp1.as_bytes().iter().collect::<HashSet<_>>();
            let s2 = comp2.as_bytes().iter().collect::<HashSet<_>>();
            s1.intersection(&s2)
                .map(|c| get_priority(**c))
                .sum::<usize>()
        })
        .sum()
}

fn part2_evaluate(s: &str) -> usize {
    s.lines()
        .tuples::<(_, _, _)>()
        .map(|(l1,l2,l3)| {
            let s1 = l1.as_bytes().iter().collect::<HashSet<_>>();
            let s2 = l2.as_bytes().iter().collect::<HashSet<_>>();
            let s3 = l3.as_bytes().iter().collect::<HashSet<_>>();
            let s_inter = &s1 & &s2;
            s_inter.intersection(&s3)
                .map(|c| get_priority(**c))
                .sum::<usize>()
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

pub fn run() {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string);
    let part2_answer = part2_evaluate(input_string);
    println!(
        "Day 3, part 1 = {part1_answer} [{}], part 2 = {part2_answer} [{}]",
        part1_answer == 7917,
        part2_answer == 2585
    );
}
