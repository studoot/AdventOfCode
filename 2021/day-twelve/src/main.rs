#[derive(Clone, PartialEq, Eq, Debug)]
struct Cave {
    name: String,
    exits: Vec<usize>,
    is_big: bool,
    is_start: bool,
    is_end: bool,
}

impl Cave {
    fn new(cave_name: &str) -> Self {
        Cave {
            name: cave_name.to_owned(),
            exits: Vec::new(),
            is_big: cave_name.chars().next().unwrap().is_ascii_uppercase(),
            is_start: cave_name == "start",
            is_end: cave_name == "end",
        }
    }
}
#[derive(Debug)]
struct Caves {
    caves: Vec<Cave>,
}

type Link<'a> = (&'a str, &'a str);

type VisitPredicate = fn(&RouteBuilder, usize) -> bool;

struct RouteBuilder<'a> {
    caves: &'a Caves,
    small_caves: Vec<usize>,
    visit_count: Vec<usize>,
    visit_predicate: VisitPredicate,
}

impl<'a> RouteBuilder<'a> {
    fn new(caves: &'a Caves, visit_predicate: VisitPredicate) -> Self {
        let num_caves = caves.caves.len();
        let small_caves = (0..caves.caves.len())
            .filter(|cave| !caves.caves[*cave].is_big)
            .collect::<Vec<_>>();
        RouteBuilder { caves, small_caves, visit_count: vec![0; num_caves], visit_predicate }
    }

    fn visit(&mut self, c: usize) -> bool {
        if (self.visit_predicate)(self, c) {
            self.visit_count[c] += 1;
            true
        } else {
            false
        }
    }
    fn pop_visit(&mut self, c: usize) {
        self.visit_count[c] -= 1;
    }

    fn find_route_count(&mut self) -> usize {
        fn find_routes_inner(rb: &mut RouteBuilder, current_cave: usize) -> usize {
            rb.caves.caves[current_cave]
                .exits
                .iter()
                .map(|i| {
                    if rb.visit(*i) {
                        let routes_found = if rb.caves.caves[*i].is_end { 1 } else { find_routes_inner(rb, *i) };
                        rb.pop_visit(*i);
                        routes_found
                    } else {
                        0
                    }
                })
                .sum()
        }
        let start_cave = self.caves.caves.iter().position(|c| c.is_start).unwrap();
        self.visit(start_cave);
        find_routes_inner(self, start_cave)
    }
}

impl Caves {
    fn new() -> Self {
        Caves { caves: Vec::new() }
    }

    fn find_or_add(&mut self, cave_name: &str) -> usize {
        self.caves
            .iter()
            .position(|c| c.name == cave_name)
            .unwrap_or_else(|| {
                self.caves.push(Cave::new(cave_name));
                self.caves.len() - 1
            })
    }

    fn add_route(&mut self, (n1, n2): Link) {
        let i1 = self.find_or_add(n1);
        let i2 = self.find_or_add(n2);
        self.caves[i1].exits.push(i2);
        self.caves[i2].exits.push(i1);
    }
}

fn parse_line(line: &str) -> Link {
    line.split_once('-').unwrap()
}

fn parse(input: &str) -> Caves {
    let mut caves = Caves::new();
    input
        .lines()
        .map(parse_line)
        .for_each(|(n1, n2)| caves.add_route((n1, n2)));
    caves
}

fn get_route_count(input: &str, visit_predicate: VisitPredicate) -> usize {
    let caves = parse(input);
    RouteBuilder::new(&caves, visit_predicate).find_route_count()
}

mod part1 {
    use super::*;

    fn visit_predicate(rb: &RouteBuilder, cave: usize) -> bool {
        rb.visit_count[cave] == 0 || rb.caves.caves[cave].is_big
    }

    pub fn run(input: &str) -> usize {
        super::get_route_count(input, visit_predicate)
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(10, run(input_string))
    }
    #[test]
    fn test1_run() {
        let input_string = include_str!("../test-1.txt");
        assert_eq!(19, run(input_string))
    }
    #[test]
    fn test2_run() {
        let input_string = include_str!("../test-2.txt");
        assert_eq!(226, run(input_string))
    }
}

mod part2 {
    use super::*;

    fn visit_predicate(rb: &RouteBuilder, cave: usize) -> bool {
        if rb.caves.caves[cave].is_start || rb.caves.caves[cave].is_end {
            rb.visit_count[cave] == 0
        } else if rb.visit_count[cave] == 0 || rb.caves.caves[cave].is_big {
            true
        } else {
            rb.small_caves.iter().all(|i| rb.visit_count[*i] < 2)
        }
    }

    pub fn run(input: &str) -> usize {
        super::get_route_count(input, visit_predicate)
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(36, run(input_string))
    }
    #[test]
    fn test1_run() {
        let input_string = include_str!("../test-1.txt");
        assert_eq!(103, run(input_string))
    }
    #[test]
    fn test2_run() {
        let input_string = include_str!("../test-2.txt");
        assert_eq!(3509, run(input_string))
    }
}

fn main() {
    let input_string = include_str!("../input.txt");
    let now = std::time::Instant::now();
    let part1_ans = part1::run(input_string);
    println!("Day 12 part 1 - {} - took {} milliseconds.", part1_ans, now.elapsed().as_millis());
    assert_eq!(part1_ans, 3298);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day 12 part 2 - {} - took {} milliseconds.", part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 93572);
}
