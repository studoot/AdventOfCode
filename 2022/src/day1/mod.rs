use itertools::Itertools;

type Calories = Vec<usize>;

fn get_calories(s: &str) -> Calories {
    s.lines()
        .group_by(|s| s.is_empty())
        .into_iter()
        .filter_map(|(is_separator, g)| (!is_separator).then(|| g.map(|s| s.parse::<usize>().unwrap()).sum()))
        .collect::<Vec<_>>()
}

fn find_max_calories(c: &Calories) -> usize {
    *c.iter().max().unwrap_or(&0)
}

fn find_top_3_calories(c: &mut Calories) -> usize {
    c.sort();
    c.rchunks(3).next().unwrap().iter().sum()
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

#[test]
fn test_part1() {
    let c = get_calories(TEST_INPUT_STRING);
    assert_eq!(find_max_calories(&c), 24_000);
}

#[test]
fn test_part2() {
    let mut c = get_calories(TEST_INPUT_STRING);
    assert_eq!(find_top_3_calories(&mut c), 45_000);
}

pub fn run() -> Option<(usize, bool, usize, bool)> {
    let input_string = include_str!("./input.txt");
    let mut c = get_calories(input_string);
    let part1_answer = find_max_calories(&c);
    let part2_answer = find_top_3_calories(&mut c);
    Some((part1_answer, part1_answer == 71924, part2_answer, part2_answer == 210406))
}
