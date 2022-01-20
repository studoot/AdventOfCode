use std::path::{Component, Path};

fn parse(input: &str) -> (String, Vec<(String, String)>) {
    let mappings = input
        .lines()
        .filter_map(|s| {
            s.split_once(" => ")
                .map(|(from, to)| (from.to_owned(), to.to_owned()))
        })
        .collect::<Vec<_>>();
    let initial = input.lines().last().unwrap().to_owned();
    (initial, mappings)
}

fn replace_mapping(s: &str, mapping: (&str, &str)) -> Vec<String> {
    let (needle, replacement) = mapping;
    s.match_indices(needle)
        .map(|(index, needle)| {
            let mut s = s.to_owned();
            s.replace_range(index..index + needle.len(), replacement);
            s
        })
        .collect::<Vec<_>>()
}

mod part1 {
    use std::collections::HashSet;

    use super::*;

    pub fn run(input: &str) -> usize {
        let (initial, mappings) = parse(input);
        mappings
            .iter()
            .flat_map(|(k, v)| replace_mapping(initial.as_str(), (k.as_str(), v.as_str())))
            .collect::<HashSet<_>>()
            .len()
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(4, run(input_string))
    }
}

mod part2 {
    use super::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    pub fn run(input: &str) -> usize {
        let (molecule, mut mappings) = parse(input);
        let mut target = molecule.clone();
        let mut rng = thread_rng();

        let mut step_count = 0;

        while target != "e" {
            let prev_target = target.clone();
            for (k, v) in &mappings {
                if target.contains(v.as_str()) {
                    target = target.replacen(v.as_str(), k.as_str(), 1);
                    step_count += 1;
                }
            }
            if prev_target == target {
                mappings.shuffle(&mut rng);
                target = molecule.clone();
                step_count = 0;
            }
        }

        step_count
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(3, run(input_string))
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
    assert_eq!(part1_ans, 576);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 207);
}
