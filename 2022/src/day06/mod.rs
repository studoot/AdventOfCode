fn all_different(bytes: &[u8]) -> bool {
    let mut found_chars: u64 = 0;
    bytes.iter().for_each(|b| {
        found_chars |= 1u64 << (*b - b'a');
    });
    found_chars.count_ones() as usize == bytes.len()
}

fn evaluate(s: &str, window_size: usize) -> usize {
    window_size
        + s.as_bytes()
            .windows(window_size)
            .position(all_different)
            .unwrap()
}

fn part1_evaluate(s: &str) -> usize {
    evaluate(s, 4)
}

fn part2_evaluate(s: &str) -> usize {
    evaluate(s, 14)
}

#[cfg(test)]
const TEST_INPUT: [(&str, usize, usize); 5] = [
    ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
    ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
    ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
    ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
    ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
];

#[cfg(test)]
#[test]
fn test_part1() {
    TEST_INPUT
        .into_iter()
        .for_each(|(input, expectation, _)| assert_eq!(part1_evaluate(input), expectation));
}

#[test]
fn test_part2() {
    TEST_INPUT
        .into_iter()
        .for_each(|(input, _, expectation)| assert_eq!(part2_evaluate(input), expectation));
}

pub fn run() -> Option<(usize, bool, usize, bool)> {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string);
    let part2_answer = part2_evaluate(input_string);
    Some((part1_answer, part1_answer == 1356, part2_answer, part2_answer == 2564))
}
