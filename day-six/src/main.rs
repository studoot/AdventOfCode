use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Fishes([u64; 9]);

impl Fishes {
    fn a_day_passes(&mut self) {
        let fish_to_breed = self.0[0];
        self.0.rotate_left(1);
        self.0[6] += fish_to_breed;
    }

    fn parse(input: &str) -> Self {
        let mut fishes = [0; 9];
        input.lines().next().unwrap().split(',').for_each(|s| fishes[usize::from_str(s).unwrap()] += 1);
        Fishes(fishes)
    }

    fn how_many(&self) -> u64 {
        self.0.iter().sum()
    }
}

fn simulate_days(days: usize, mut fishes: Fishes) -> u64 {
    for _day in 0..days {
        fishes.a_day_passes();
    }
    fishes.how_many()
}

mod part1 {
    use super::*;
    pub fn run(input: &str) -> u64 {
        let fishes = Fishes::parse(input);
        simulate_days(80, fishes)
    }

    #[test]
    fn test_day_by_day() {
        let input_string = include_str!("../test.txt");
        let mut fishes = Fishes::parse(input_string);
        fishes.a_day_passes();
        let fishes_after_1_day = Fishes::parse("2,3,2,0,1");
        assert_eq!(fishes, fishes_after_1_day);
        fishes.a_day_passes();
        let fishes_after_2_days = Fishes::parse("1,2,1,6,0,8");
        assert_eq!(fishes, fishes_after_2_days);
        fishes.a_day_passes();
        let fishes_after_3_days = Fishes::parse("0,1,0,5,6,7,8");
        assert_eq!(fishes, fishes_after_3_days, "Day 3");
        fishes.a_day_passes();
        let fishes_after_4_days = Fishes::parse("6,0,6,4,5,6,7,8,8");
        assert_eq!(fishes, fishes_after_4_days, "Day 4");
        fishes.a_day_passes();
        let fishes_after_5_days = Fishes::parse("5,6,5,3,4,5,6,7,7,8");
        assert_eq!(fishes, fishes_after_5_days, "Day 5");
        fishes.a_day_passes();
        let fishes_after_6_days = Fishes::parse("4,5,4,2,3,4,5,6,6,7");
        assert_eq!(fishes, fishes_after_6_days, "Day 6");
        fishes.a_day_passes();
        let fishes_after_7_days = Fishes::parse("3,4,3,1,2,3,4,5,5,6");
        assert_eq!(fishes, fishes_after_7_days, "Day 7");
        fishes.a_day_passes();
        let fishes_after_8_days = Fishes::parse("2,3,2,0,1,2,3,4,4,5");
        assert_eq!(fishes, fishes_after_8_days, "Day 8");
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(5934, run(input_string))
    }
}

mod part2 {
    use super::*;
    pub fn run(input: &str) -> u64 {
        let fishes = Fishes::parse(input);
        simulate_days(256, fishes)
    }
    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(26984457539, run(input_string))
    }
}
fn main() {
    let input_string = include_str!("../input.txt");
    let part1_ans=part1::run(input_string);
    println!("Day  6 part 1 - {}", part1_ans);
    assert_eq!(part1_ans, 360268);
    let part2_ans=part2::run(input_string);
    println!("Day  6 part 2 - {}", part2_ans);
    assert_eq!(part2_ans, 1632146183902);
}
