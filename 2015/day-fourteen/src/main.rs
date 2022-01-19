use std::path::{Component, Path};

struct Reindeer {
    speed: usize,
    move_time: usize,
    rest_time: usize,
}

impl Reindeer {
    fn distance(&self, time: usize) -> usize {
        let cycle_time = self.move_time + self.rest_time;
        let whole_cycle_distance = (time / cycle_time) * (self.speed * self.move_time);
        let end_bit_time = time % cycle_time;
        let end_bit_distance = end_bit_time.min(self.move_time) * self.speed;
        whole_cycle_distance + end_bit_distance
    }
}

fn parse_line(i: &str) -> Reindeer {
    let tokens = i.split_ascii_whitespace().collect::<Vec<_>>();
    let speed = tokens[3].parse::<usize>().unwrap();
    let move_time = tokens[6].parse::<usize>().unwrap();
    let rest_time = tokens[13].parse::<usize>().unwrap();
    Reindeer { speed, move_time, rest_time }
}

fn parse(i: &str) -> Vec<Reindeer> {
    i.lines().map(parse_line).collect::<Vec<_>>()
}

mod part1 {
    use super::*;

    pub fn run(input: &str, end_time: usize) -> usize {
        let reindeer = parse(input);
        reindeer
            .into_iter()
            .map(|r| r.distance(end_time))
            .max()
            .unwrap()
    }

    #[test]
    fn test_run() {
        let input_string = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\n\
                                 Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        assert_eq!(1120, run(input_string, 1000))
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str, end_time: usize) -> usize {
        let reindeer = parse(input);
        let mut reindeer_scores = vec![0; reindeer.len()];

        for time in 1..=end_time {
            let distances = reindeer
                .iter()
                .map(|r| r.distance(time))
                .collect::<Vec<_>>();
            let max_distance = *distances.iter().max().unwrap();
            distances
                .iter()
                .enumerate()
                .filter(|(_, d)| **d == max_distance)
                .for_each(|(index, _)| reindeer_scores[index] += 1);
        }
        reindeer_scores.into_iter().max().unwrap()
    }

    #[test]
    fn test_run() {
        let input_string = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\n\
                                 Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        assert_eq!(689, run(input_string, 1000))
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
    let part1_ans = part1::run(input_string, 2503);
    println!("Day {} part 1 - {} - took {} milliseconds.", day_number, part1_ans, now.elapsed().as_millis());
    assert_eq!(part1_ans, 2660);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string, 2503);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 1256);
}
