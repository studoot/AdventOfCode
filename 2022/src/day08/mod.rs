use itertools::Itertools;
use ndarray::Array2;

fn parse(s: &str) -> Array2<u8> {
    let mut row_size = Option::None;
    let numbers = s
        .lines()
        .flat_map(|l| {
            if row_size.is_none() {
                row_size = Some(l.as_bytes().len());
            }
            l.bytes()
                .map(|digit_char| digit_char - b'0')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    Array2::from_shape_vec((row_size.unwrap(), numbers.len() / row_size.unwrap()), numbers)
        .expect("Bad vector -> array2 construction")
}

fn part1_evaluate(s: &str) -> usize {
    let arr = parse(s);

    (0..arr.nrows())
        .cartesian_product(0..arr.ncols())
        .filter(|(r, c)| {
            if *r == 0 || *r == (arr.nrows() - 1) || *c == 0 || *c == (arr.ncols() - 1) {
                return true;
            }
            let this_val = arr.get((*r, *c)).unwrap();
            (0..*c).all(|c| arr.get((*r, c)).unwrap() < this_val)
                || (c + 1..arr.ncols()).all(|c| arr.get((*r, c)).unwrap() < this_val)
                || (0..*r).all(|r| arr.get((r, *c)).unwrap() < this_val)
                || (r + 1..arr.nrows()).all(|r| arr.get((r, *c)).unwrap() < this_val)
        })
        .count()
}

fn part2_evaluate(s: &str) -> usize {
    let arr = parse(s);

    (0..arr.nrows())
        .cartesian_product(0..arr.ncols())
        .map(|(r, c)| {
            if r == 0 || r == (arr.nrows() - 1) || c == 0 || c == (arr.ncols() - 1) {
                0
            } else {
                let this_val = arr.get((r, c)).unwrap();
                let score_left = (1
                    + (0..c)
                        .rev()
                        .take_while(|c| arr.get((r, *c)).unwrap() < this_val)
                        .count())
                .min((0..c).len());
                let score_right = (1
                    + (c + 1..arr.ncols())
                        .take_while(|c| arr.get((r, *c)).unwrap() < this_val)
                        .count())
                .min((c + 1..arr.ncols()).len());
                let score_up = (1
                    + (0..r)
                        .rev()
                        .take_while(|r| arr.get((*r, c)).unwrap() < this_val)
                        .count())
                .min((0..r).len());
                let score_down = (1
                    + (r + 1..arr.nrows())
                        .take_while(|r| arr.get((*r, c)).unwrap() < this_val)
                        .count())
                .min((r + 1..arr.nrows()).len());
                score_down * score_left * score_right * score_up
            }
        })
        .max()
        .unwrap()
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = "\
30373
25512
65332
33549
35390";
#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1_evaluate(TEST_INPUT_STRING), 21);
}

#[test]
fn test_part2() {
    assert_eq!(part2_evaluate(TEST_INPUT_STRING), 8);
}

pub fn run() -> Option<(usize, bool, usize, bool)> {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string);
    let part2_answer = part2_evaluate(input_string);
    Some((part1_answer, part1_answer == 1_835, part2_answer, part2_answer == 263_670))
}
