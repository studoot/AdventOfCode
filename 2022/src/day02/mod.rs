fn play_matches(s: &str, play_match: fn(u8, u8) -> usize) -> usize {
    s.lines()
        .map(|line| play_match(line.as_bytes()[0], line.as_bytes()[2]))
        .sum()
}

fn part1_evaluate(s: &str) -> usize {
    play_matches(s, |their_play, my_play| {
        let score = (my_play - b'X' + 1)
            + match (their_play - b'A', my_play - b'X') {
                (theirs, mine) if theirs == mine => 3,           // Draw
                (theirs, mine) if (theirs + 1) % 3 == mine => 6, // Win
                _ => 0,                                          // Lost
            };
        score as usize
    })
}

fn part2_evaluate(s: &str) -> usize {
    play_matches(s, |their_play, desired_outcome| {
        let their_shape_score = their_play - b'A'; // Put in range 0..2 to simplify use of modulus
        let score = match desired_outcome {
            b'X' => 1 + ((their_shape_score + 2) % 3), // Lost, so played one less then them
            b'Y' => 3 + 1 + their_shape_score,         // Draw, so play the same as them
            b'Z' => 6 + 1 + (their_shape_score + 1) % 3, // Won, so played one more then them
            _ => panic!("Unexpected desired outcome {desired_outcome}"),
        };
        score as usize
    })
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = "\
A Y
B X
C Z";
#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1_evaluate(TEST_INPUT_STRING), 15);
}

#[test]
fn test_part2() {
    assert_eq!(part2_evaluate(TEST_INPUT_STRING), 12);
}

pub fn run() -> Option<(usize, bool, usize, bool)> {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string);
    let part2_answer = part2_evaluate(input_string);
    Some((part1_answer, part1_answer == 11150, part2_answer, part2_answer == 8295))
}
