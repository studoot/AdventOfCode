fn evaluate(s: &str, f: fn((usize, usize), (usize, usize)) -> bool) -> usize {
    s.lines()
        .filter(|line| {
            let (e1, e2) = line.split_once(',').unwrap();
            let (e1_begin, e1_end) = e1.split_once('-').unwrap();
            let (e2_begin, e2_end) = e2.split_once('-').unwrap();
            let e1 = (e1_begin.parse::<usize>().unwrap(), e1_end.parse::<usize>().unwrap());
            let e2 = (e2_begin.parse::<usize>().unwrap(), e2_end.parse::<usize>().unwrap());
            f(e1, e2)
        })
        .count()
}

fn part1_evaluate(s: &str) -> usize {
    evaluate(s, |(e1_begin, e1_end), (e2_begin, e2_end)| {
        (e1_begin <= e2_begin && e1_end >= e2_end) || (e1_begin >= e2_begin && e1_end <= e2_end)
    })
}

fn part2_evaluate(s: &str) -> usize {
    evaluate(s, |(e1_begin, e1_end), (e2_begin, e2_end)| e1_begin.max(e2_begin) <= e1_end.min(e2_end))
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1_evaluate(TEST_INPUT_STRING), 2);
}

#[test]
fn test_part2() {
    assert_eq!(part2_evaluate(TEST_INPUT_STRING), 4);
}

pub fn run() -> Option<(usize, bool, usize, bool)> {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string);
    let part2_answer = part2_evaluate(input_string);
    Some((part1_answer, part1_answer == 477, part2_answer, part2_answer == 830))
}
