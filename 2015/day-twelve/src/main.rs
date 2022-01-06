use std::path::{Component, Path};

use itertools::Itertools;
use json::JsonValue;

fn sum_nums(i: &str) -> isize {
    i.chars()
        .group_by(|c| *c == '-' || (*c >= '0' && *c <= '9'))
        .into_iter()
        .map(|(key, group)| {
            if key {
                let (sign, sum) = group.into_iter().fold((1, 0), |(sign, sum), c| {
                    if c == '-' {
                        (-1, 0)
                    } else {
                        (sign, sum * 10 + (c as isize) - (b'0' as isize))
                    }
                });
                sign * sum
            } else {
                0
            }
        })
        .sum()
}

fn get_json_sum(jv: &JsonValue) -> isize {
    use json::JsonValue::*;
    match jv {
        Null | String(_) | Short(_) | Boolean(_) => 0,
        Number(n) => isize::try_from(*n).unwrap_or_else(|_err| panic!("Bad JSON number {}", &n)),
        Object(o) => {
            if o.iter().any(|(_key, jv)| jv == "red") {
                0
            } else {
                o.iter().map(|(_key, jv)| get_json_sum(jv)).sum()
            }
        }
        Array(a) => a.iter().map(get_json_sum).sum(),
    }
}

fn sum_nums2(i: &str) -> isize {
    get_json_sum(&json::parse(i).unwrap())
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> isize {
        sum_nums(input)
    }

    #[test]
    fn test_run() {
        assert_eq!(sum_nums("[1,2,3]"), 6);
        assert_eq!(sum_nums(r#"{"a":2,"b":4}"#), 6);
        assert_eq!(sum_nums("[[[3]]]"), 3);
        assert_eq!(sum_nums(r#"{"a":{"b":4},"c":-1}"#), 3);
        assert_eq!(sum_nums(r#"{"a":[-1,1]}"#), 0);
        assert_eq!(sum_nums(r#"[-1,{"a":1}]"#), 0);
        assert_eq!(sum_nums(r#"[]"#), 0);
        assert_eq!(sum_nums(r#"{}"#), 0);
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> isize {
        sum_nums2(input)
    }
    #[test]
    fn test_run() {
        assert_eq!(sum_nums2("[1,2,3]"), 6);
        assert_eq!(sum_nums2(r#"{"a":2,"b":4}"#), 6);
        assert_eq!(sum_nums2("[[[3]]]"), 3);
        assert_eq!(sum_nums2(r#"{"a":{"b":4},"c":-1}"#), 3);
        assert_eq!(sum_nums2(r#"{"a":[-1,1]}"#), 0);
        assert_eq!(sum_nums2(r#"[-1,{"a":1}]"#), 0);
        assert_eq!(sum_nums2(r#"[]"#), 0);
        assert_eq!(sum_nums2(r#"{}"#), 0);

        assert_eq!(sum_nums2(r#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(sum_nums2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
        assert_eq!(sum_nums2(r#"[1,"red",5]"#), 6);
    }
}

fn main() {
    let input_string = include_str!("../input.txt");
    let day_number = Path::new(file!())
        .components()
        .find_map(|bit| {
            if let Component::Normal(os_name) = bit {
                if let Some(dir_name) = os_name.to_str() {
                    return dir_name.strip_prefix("day-");
                }
            };
            None
        })
        .unwrap()
        .to_lowercase()
        .replace("-", " ")
        .replace("_", " ");
    let now = std::time::Instant::now();
    let part1_ans = part1::run(input_string);
    println!("Day {} part 1 - {} - took {} milliseconds.", day_number, part1_ans, now.elapsed().as_millis());
    assert_eq!(part1_ans, 156366);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 96852);
}
