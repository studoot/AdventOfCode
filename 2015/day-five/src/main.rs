use std::path::{Component, Path};
mod part1 {
    fn is_nice(s: &str) -> bool {
        (s.chars()
            .filter(|c| ['a', 'e', 'i', 'o', 'u'].contains(c))
            .count()
            >= 3)
            && (s.as_bytes().windows(2).any(|a| a[0] == a[1]))
            && (s
                .as_bytes()
                .windows(2)
                .all(|a| ![[b'a', b'b'], [b'c', b'd'], [b'p', b'q'], [b'x', b'y']].contains(&[a[0], a[1]])))
    }

    pub fn run(input: &str) -> usize {
        input.lines().filter(|s| is_nice(*s)).count()
    }

    #[test]
    fn test_run() {
        assert_eq!(run("ugknbfddgicrmopn"), 1);
        assert_eq!(run("aaa"), 1);
        assert_eq!(run("jchzalrnumimnmhp"), 0);
        assert_eq!(run("haegwjzuvuyypxyu"), 0);
        assert_eq!(run("dvszwmarrgswjxmb"), 0);
    }
}

mod part2 {
    use itertools::Itertools;
    use multimap::MultiMap;

    fn is_nice(s: &str) -> bool {
        s.chars()
            .tuple_windows::<(_, _)>()
            .enumerate()
            .map(|(i, p)| (p, i))
            .collect::<MultiMap<_, _>>()
            .iter_all()
            .any(|(_k, vs)| vs.len() >= 2 && !vs.iter().tuple_windows().any(|(a, b)| (*b - *a) < 2))
            && (s.chars().tuple_windows().any(|(a, _, c)| a == c))
    }

    pub fn run(input: &str) -> usize {
        input.lines().filter(|s| is_nice(*s)).count()
    }

    #[test]
    fn test_run() {
        assert_eq!(run("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(run("xxyxx"), 1);
        assert_eq!(run("uurcxstgmygtbstg"), 0);
        assert_eq!(run("ieodomkazucvgmuy"), 0);
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
    assert_eq!(part1_ans, 258);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 53);
}
