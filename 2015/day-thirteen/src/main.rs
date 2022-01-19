use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::{Component, Path};

struct HappinessChange {
    person: String,
    change: isize,
    neighbour: String,
}

lazy_static! {
    static ref HAPPINESS_MATCHER:Regex = Regex::new(r#"^(?P<person>[[:alpha:]]+) would (?P<direction>gain|lose) (?P<amount>\d+) happiness units by sitting next to (?P<neighbour>[[:alpha:]]+)\.$"#).unwrap();
}

fn parse(input: &str) -> Vec<HappinessChange> {
    input
        .lines()
        .map(|l| {
            let caps = HAPPINESS_MATCHER.captures(l).unwrap();
            let person = caps.name("person").unwrap().as_str().to_owned();
            let direction = if caps.name("direction").unwrap().as_str() == "gain" { 1 } else { -1 };
            let change = caps
                .name("amount")
                .unwrap()
                .as_str()
                .parse::<isize>()
                .unwrap()
                * direction;
            let neighbour = caps.name("neighbour").unwrap().as_str().to_owned();
            HappinessChange { person, change, neighbour }
        })
        .collect::<Vec<_>>()
}

fn determine_happiness(seating_plan: Vec<&String>, happiness_records: &HashMap<(&String, &String), isize>) -> isize {
    seating_plan
        .into_iter()
        .circular_tuple_windows::<(_, _, _)>()
        .map(|(n1, p, n2)| {
            let c1 = happiness_records.get(&(p, n1)).unwrap_or(&0);
            let c2 = happiness_records.get(&(p, n2)).unwrap_or(&0);
            c1 + c2
        })
        .sum()
}

mod part1 {
    use std::collections::{HashMap, HashSet};

    use super::*;

    pub fn run(input: &str) -> isize {
        let records = parse(input);
        let people = records
            .iter()
            .map(|c| c.person.clone())
            .collect::<HashSet<_>>();
        let happiness_records = records
            .iter()
            .map(|c| ((&c.person, &c.neighbour), c.change))
            .collect::<HashMap<_, _>>();
        people
            .iter()
            .permutations(people.len())
            .map(|seating_plan| determine_happiness(seating_plan, &happiness_records))
            .max()
            .unwrap()
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(330, run(input_string))
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> isize {
        let records = parse(input);
        let mut people = records
            .iter()
            .map(|c| c.person.clone())
            .collect::<HashSet<_>>();
        people.insert("Stu".to_owned());
        let happiness_records = records
            .iter()
            .map(|c| ((&c.person, &c.neighbour), c.change))
            .collect::<HashMap<_, _>>();
        people
            .iter()
            .permutations(people.len())
            .map(|seating_plan| determine_happiness(seating_plan, &happiness_records))
            .max()
            .unwrap()
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
    assert_eq!(part1_ans, 709);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 668);
}
