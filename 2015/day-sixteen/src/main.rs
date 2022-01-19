use std::collections::HashMap;
use std::path::{Component, Path};

type AuntySue = HashMap<String, usize>;

fn from_string(s: &str) -> (usize, AuntySue) {
    let (prefix, attrs) = s.split_once(':').unwrap();
    let index = prefix
        .strip_prefix("Sue ")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let sue = attrs
        .trim()
        .split(", ")
        .map(|s| {
            let (name, value_string) = s.split_once(": ").unwrap();
            let value = value_string.parse::<usize>().unwrap();
            (name.to_owned(), value)
        })
        .collect::<HashMap<_, _>>();
    (index, sue)
}

fn datum() -> AuntySue {
    AuntySue::from([
        ("children".to_owned(), 3),
        ("cats".to_owned(), 7),
        ("samoyeds".to_owned(), 2),
        ("pomeranians".to_owned(), 3),
        ("akitas".to_owned(), 0),
        ("vizslas".to_owned(), 0),
        ("goldfish".to_owned(), 5),
        ("trees".to_owned(), 3),
        ("cars".to_owned(), 2),
        ("perfumes".to_owned(), 1),
    ])
}

fn contains(datum: &AuntySue, item: &AuntySue) -> bool {
    item.iter().all(|(k, v)| datum.get(k) == Some(v))
}

// In particular, the cats and trees readings indicates that there are greater
// than that many (due to the unpredictable nuclear decay of cat dander and tree
// pollen), while the pomeranians and goldfish readings indicate that there are
// fewer than that many (due to the modial interaction of magnetoreluctance).
fn contains2(datum: &AuntySue, item: &AuntySue) -> bool {
    item.iter().all(|(k, v)| match k.as_str() {
        "cats" | "trees" => !datum.contains_key(k) || v > datum.get(k).unwrap(),
        "pomeranians" | "goldfish" => !datum.contains_key(k) || v < datum.get(k).unwrap(),
        _ => datum.get(k) == Some(v),
    })
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let forensic_result = datum();
        input
            .lines()
            .map(from_string)
            .find(|(_, sue)| contains(&forensic_result, sue))
            .unwrap()
            .0
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!((1234, datum()), from_string(input_string));
        assert!(contains(&datum(), &from_string(input_string).1));
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let forensic_result = datum();
        input
            .lines()
            .map(from_string)
            .find(|(_, sue)| contains2(&forensic_result, sue))
            .unwrap()
            .0
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
    assert_eq!(part1_ans, 213);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 323);
}
