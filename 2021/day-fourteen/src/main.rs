use itertools::Itertools;
use std::collections::HashMap;
use std::hash::Hash;

type Pair = [char; 2];
type Insertion = char;

type ExpansionRules = HashMap<Pair, Insertion>;

fn expand(template: &str, polymer_rules: &ExpansionRules) -> String {
    template.chars().tuple_windows().enumerate().fold(
        String::with_capacity(template.len() * 3 / 2),
        |mut s, (i, (a, b))| {
            s.push(a);
            s.push(*polymer_rules.get(&[a, b]).unwrap());
            if i == template.len() - 2 {
                s.push(b);
            }
            s
        },
    )
}

fn polymerize(template: &str, polymer_rules: &ExpansionRules, step_count: usize) -> String {
    (0..step_count).fold(template.to_owned(), |s, _| expand(&s, polymer_rules))
}

type PairCounter = HashMap<Pair, u64>;
type CharCounter = HashMap<char, u64>;

fn add<K: Eq + Hash>(counter: &mut HashMap<K, u64>, k: K, diff: u64) {
    counter
        .entry(k)
        .and_modify(|count| *count += diff)
        .or_insert(diff);
}

type PairMappings = HashMap<Pair, (Pair, Pair)>;
fn find_pair_mappings(rules: &ExpansionRules) -> PairMappings {
    rules
        .iter()
        .map(|(&pair, &insert)| {
            let p1 = [pair[0], insert];
            let p2 = [insert, pair[1]];
            (pair, (p1, p2))
        })
        .collect::<HashMap<_, _>>()
}

fn apply_pair_mappings(pair_mappings: &PairMappings, pair_counts: &PairCounter) -> PairCounter {
    let mut new_pair_counts = PairCounter::new();
    pair_counts.iter().for_each(|(from, &count)| {
        let to = *pair_mappings.get(from).unwrap();
        add(&mut new_pair_counts, to.0, count);
        add(&mut new_pair_counts, to.1, count);
    });
    new_pair_counts
}

fn polymerize_with_counts(template: &str, polymer_rules: &ExpansionRules, step_count: usize) -> CharCounter {
    let pair_mapper = find_pair_mappings(polymer_rules);
    let mut pair_counter = PairCounter::new();
    template.chars().tuple_windows().for_each(|(a, b)| {
        add(&mut pair_counter, [a, b], 1);
    });
    pair_counter = (0..step_count).fold(pair_counter, |pair_counts, _| apply_pair_mappings(&pair_mapper, &pair_counts));
    let mut char_counts = CharCounter::new();
    [(template.chars().next().unwrap(), 1), (template.chars().last().unwrap(), 1)]
        .into_iter()
        .chain(
            pair_counter
                .into_iter()
                .flat_map(|([c1, c2], count)| [(c1, count), (c2, count)]),
        )
        .for_each(|(c, count)| add(&mut char_counts, c, count));
    char_counts
        .into_iter()
        .map(|(c, count)| (c, count / 2))
        .collect::<CharCounter>()
}

fn parse(input: &str) -> (&str, ExpansionRules) {
    let mut x = input.lines();
    let template = x.next().unwrap();
    let rules = x
        .filter_map(|l| match l.split_once(" -> ") {
            Some((pair, insertion)) if pair.len() == 2 && insertion.len() == 1 => {
                Some(([pair.chars().next().unwrap(), pair.chars().nth(1).unwrap()], insertion.chars().next().unwrap()))
            }
            _ => None,
        })
        .collect::<ExpansionRules>();
    (template, rules)
}

fn get_character_counts(s: &str) -> CharCounter {
    let mut char_counts = CharCounter::new();
    s.chars().for_each(|c| add(&mut char_counts, c, 1));
    char_counts
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> u64 {
        let (template, rules) = parse(input);
        let polymer = polymerize(template, &rules, 10);
        let char_counter = get_character_counts(&polymer);
        let (min, max) = char_counter
            .iter()
            .minmax_by_key(|(_, v)| **v)
            .into_option()
            .unwrap();
        *max.1 - *min.1
    }

    #[test]
    fn test_polymerize() {
        // This polymer grows quickly. After step 5, it has length 97; After step
        // 10, it has length 3073. After step 10, B occurs 1749 times, C occurs 298
        // times, H occurs 161 times, and N occurs 865 times; taking the quantity of
        // the most common element (B, 1749) and subtracting the quantity of the
        // least common element (H, 161) produces 1749 - 161 = 1588.

        let input_string = include_str!("../test.txt");
        let (t, rules) = parse(input_string);

        assert_eq!(t, "NNCB");
        assert_eq!(rules.len(), 16);

        assert_eq!(expand(&t, &rules), "NCNBCHB");
        assert_eq!(polymerize(&t, &rules, 2), "NBCCNBBBCBHCB");
        assert_eq!(polymerize(&t, &rules, 3), "NBBBCNCCNBBNBNBBCHBHHBCHB");
        assert_eq!(polymerize(&t, &rules, 4), "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");
        assert_eq!(polymerize(&t, &rules, 5).len(), 97);
        assert_eq!(polymerize(&t, &rules, 10).len(), 3073);
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(1588, run(input_string))
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> u64 {
        let (template, rules) = parse(input);
        let char_counter = polymerize_with_counts(template, &rules, 40);
        let (min, max) = char_counter
            .iter()
            .minmax_by_key(|(_, v)| **v)
            .into_option()
            .unwrap();
        *max.1 - *min.1
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(2188189693529, run(input_string))
    }

    #[test]
    fn test_polymerize() {
        // This polymer grows quickly. After step 5, it has length 97; After step
        // 10, it has length 3073. After step 10, B occurs 1749 times, C occurs 298
        // times, H occurs 161 times, and N occurs 865 times; taking the quantity of
        // the most common element (B, 1749) and subtracting the quantity of the
        // least common element (H, 161) produces 1749 - 161 = 1588.

        let input_string = include_str!("../test.txt");
        let (t, rules) = parse(input_string);

        assert_eq!(t, "NNCB");
        assert_eq!(rules.len(), 16);

        assert_eq!(
            polymerize_with_counts(&t, &rules, 1)
                .into_values()
                .sum::<u64>(),
            get_character_counts("NCNBCHB").into_values().sum()
        );
        assert_eq!(polymerize_with_counts(&t, &rules, 1), get_character_counts("NCNBCHB"));
        assert_eq!(polymerize_with_counts(&t, &rules, 2), get_character_counts("NBCCNBBBCBHCB"));
        assert_eq!(polymerize_with_counts(&t, &rules, 3), get_character_counts("NBBBCNCCNBBNBNBBCHBHHBCHB"));
        assert_eq!(
            polymerize_with_counts(&t, &rules, 4),
            get_character_counts("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB")
        );
        let counts_40 = polymerize_with_counts(&t, &rules, 40);
        assert_eq!(*counts_40.get(&'B').unwrap(), 2192039569602);
        assert_eq!(*counts_40.get(&'H').unwrap(), 3849876073);
    }
}

fn main() {
    let input_string = include_str!("../input.txt");
    let now = std::time::Instant::now();
    let part1_ans = part1::run(input_string);
    println!("Day 14 part 1 - {} - took {} milliseconds.", part1_ans, now.elapsed().as_millis());
    assert_eq!(part1_ans, 2975);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!(
        "Day 14 part 2 - {answer} - took {time} milliseconds.",
        answer = part2_ans,
        time = now.elapsed().as_millis()
    );
    assert_eq!(part2_ans, 3015383850689);
}
