use std::collections::{HashMap, HashSet};
use std::path::{Component, Path};

#[derive(Debug, Eq, PartialEq)]
struct Route {
    endpoint: String,
    distance: usize,
}

#[derive(Debug, Eq, PartialEq)]
struct City {
    name: String,
    routes: Vec<Route>,
}
impl City {
    fn new(name: &str) -> Self {
        City { name: name.to_owned(), routes: Vec::new() }
    }
    fn add_route(&mut self, to: &str, dist: usize) {
        self.routes
            .push(Route { endpoint: to.to_owned(), distance: dist });
    }
}

fn parse_line(l: &str) -> (&str, &str, usize) {
    let (cities, distance) = l.split_once(" = ").unwrap();
    let (city_1, city_2) = cities.split_once(" to ").unwrap();
    (city_1, city_2, distance.parse::<usize>().unwrap())
}

fn parse(input: &str) -> HashMap<String, City> {
    let mut cities = HashMap::new();
    input.lines().for_each(|l| {
        let (a, b, dist) = parse_line(l);
        let e_a = cities.entry(a.to_owned()).or_insert_with(|| City::new(a));
        e_a.add_route(b, dist);
        let e_b = cities.entry(b.to_owned()).or_insert_with(|| City::new(b));
        e_b.add_route(a, dist);
    });
    cities
}

fn visit_min(city: &City, cities: &HashMap<String, City>, already_visited: &mut HashSet<String>) -> Option<usize> {
    already_visited.insert(city.name.clone());
    let min_distance = city
        .routes
        .iter()
        .filter_map(|r| {
            let next_city = cities.get(&r.endpoint).unwrap();
            if !already_visited.contains(&next_city.name) {
                visit_min(next_city, cities, already_visited).map(|d| d + r.distance)
            } else if already_visited.len() == cities.len() {
                Some(0)
            } else {
                None
            }
        })
        .min();
    already_visited.remove(&city.name);
    min_distance
}

fn find_shortest_route(cities: &HashMap<String, City>, already_visited: &mut HashSet<String>) -> usize {
    cities
        .iter()
        .filter_map(|(_name, city)| visit_min(city, cities, already_visited))
        .min()
        .unwrap()
}

fn visit_max(city: &City, cities: &HashMap<String, City>, already_visited: &mut HashSet<String>) -> Option<usize> {
    already_visited.insert(city.name.clone());
    let min_distance = city
        .routes
        .iter()
        .filter_map(|r| {
            let next_city = cities.get(&r.endpoint).unwrap();
            if !already_visited.contains(&next_city.name) {
                visit_max(next_city, cities, already_visited).map(|d| d + r.distance)
            } else if already_visited.len() == cities.len() {
                Some(0)
            } else {
                None
            }
        })
        .max();
    already_visited.remove(&city.name);
    min_distance
}

fn find_longest_route(cities: &HashMap<String, City>, already_visited: &mut HashSet<String>) -> usize {
    cities
        .iter()
        .filter_map(|(_name, city)| visit_max(city, cities, already_visited))
        .max()
        .unwrap()
}
mod part1 {
    use std::collections::HashSet;

    use super::*;

    pub fn run(input: &str) -> usize {
        let cities = parse(input);
        find_shortest_route(&cities, &mut HashSet::new())
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(605, run(input_string))
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let cities = parse(input);
        find_longest_route(&cities, &mut HashSet::new())
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(982, run(input_string))
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
    assert_eq!(part1_ans, 117);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 909);
}
