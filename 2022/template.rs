fn part1_evaluate(s: &str) -> usize {
}

fn part2_evaluate(s: &str) -> usize {
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = "\
";
#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1_evaluate(TEST_INPUT_STRING), 0);
}

#[test]
fn test_part2() {
    assert_eq!(part2_evaluate(TEST_INPUT_STRING), 0);
}

pub fn run() -> Option<(usize, bool, usize, bool)> {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string);
    let part2_answer = part2_evaluate(input_string);
    Some((part1_answer, part1_answer == 0, part2_answer, part2_answer == 0))
}
