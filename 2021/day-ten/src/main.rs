#[derive(Debug, Eq, PartialEq)]
enum LineResult {
    Illegal(char),
    Incomplete(String),
    Complete,
}

impl LineResult {
    fn score(&self) -> usize {
        match self {
            Self::Complete => 0,
            Self::Incomplete(s) => s.chars().fold(0, |acc, c| {
                (acc * 5)
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => 0,
                    }
            }),
            Self::Illegal(c) => match c {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            },
        }
    }
}

fn parse_line(line: &str) -> LineResult {
    let mut expecting = String::new();
    line.chars()
        .map(|c| {
            match c {
                '(' => expecting.push(')'),
                '[' => expecting.push(']'),
                '{' => expecting.push('}'),
                '<' => expecting.push('>'),
                ')' | ']' | '}' | '>' => {
                    if expecting.chars().last().unwrap_or(' ') == c {
                        expecting.pop();
                    } else {
                        return LineResult::Illegal(c);
                    }
                }
                _ => (),
            }
            LineResult::Complete
        })
        .find(|l| matches!(l, LineResult::Illegal(_)))
        .unwrap_or_else(|| {
            if expecting.is_empty() {
                LineResult::Complete
            } else {
                LineResult::Incomplete(expecting.chars().rev().collect())
            }
        })
}

fn parse(input: &str) -> Vec<LineResult> {
    input.lines().map(parse_line).collect::<Vec<_>>()
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> usize {
        parse(input)
            .into_iter()
            .filter_map(|l| if let LineResult::Illegal(_) = l { Some(l.score()) } else { None })
            .sum()
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(26397, run(input_string))
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let mut line_completion_scores = parse(input)
            .into_iter()
            .filter_map(|l| if let LineResult::Incomplete(_) = l { Some(l.score()) } else { None })
            .collect::<Vec<_>>();
        line_completion_scores.sort_unstable();
        line_completion_scores[line_completion_scores.len() / 2]
    }

    #[test]
    fn test_completions() {
        assert_eq!(LineResult::Incomplete("}}]])})]".to_owned()).score(), 288957);
        assert_eq!(LineResult::Incomplete(")}>]})".to_owned()).score(), 5566);
        assert_eq!(LineResult::Incomplete("}}>}>))))".to_owned()).score(), 1480781);
        assert_eq!(LineResult::Incomplete("]]}}]}]}>".to_owned()).score(), 995444);
        assert_eq!(LineResult::Incomplete("])}>".to_owned()).score(), 294);
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(288957, run(input_string))
    }
}
fn main() {
    let input_string = include_str!("../input.txt");
    let part1_ans = part1::run(input_string);
    println!("Day 10 part 1 - {}", part1_ans);
    assert_eq!(part1_ans, 364389);
    let part2_ans = part2::run(input_string);
    println!("Day 10 part 2 - {}", part2_ans);
    assert_eq!(part2_ans, 2870201088);
}
