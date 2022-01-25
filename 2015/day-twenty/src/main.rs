use std::path::{Component, Path};

mod part1 {
    #[allow(dead_code)]
    fn present_count(house_number: usize) -> usize {
        let max_factor = (house_number as f64).sqrt().floor() as usize;
        (1..=max_factor).fold(0, |acc, num| {
            acc + if house_number % num == 0 {
                if num * num == house_number {
                    10 * num
                } else {
                    10 * (num + house_number / num)
                }
            } else {
                0
            }
        })
    }

    pub fn run() -> usize {
        let mut houses = vec![0usize; 3_500_000];
        for i in 1..houses.len() {
            let mut c = i;
            while c < houses.len() {
                houses[c] += i * 10;
                c += i;
            }
        }
        houses
            .iter()
            .enumerate()
            .find(|(_, presents)| **presents >= 33_100_000)
            .unwrap()
            .0
    }

    #[test]
    fn test_present_count() {
        assert_eq!(10, present_count(1));
        assert_eq!(30, present_count(2));
        assert_eq!(40, present_count(3));
        assert_eq!(70, present_count(4));
        assert_eq!(60, present_count(5));
        assert_eq!(120, present_count(6));
        assert_eq!(80, present_count(7));
        assert_eq!(150, present_count(8));
        assert_eq!(130, present_count(9));
    }

    #[test]
    fn test_run() {
        assert_eq!((2500..).find(|n| present_count(*n) >= 33_100_000).unwrap(), run());
    }
}

mod part2 {
    #[allow(dead_code)]
    fn present_count(house_number: usize) -> usize {
        let max_factor = (house_number as f64).sqrt().floor() as usize;
        (1..=max_factor).fold(0, |acc, num| {
            acc + if house_number % num == 0 {
                if num * num == house_number {
                    11 * (if num * 51 > house_number { num } else { 0 })
                } else {
                    let oppo = house_number / num;
                    11 * (if oppo <= 50 { num } else { 0 } + if num <= 50 { oppo } else { 0 })
                }
            } else {
                0
            }
        })
    }

    pub fn run() -> usize {
        let mut houses = vec![0usize; 3_500_000];
        for i in 1..houses.len() {
            let mut c = i;
            for _ in 1..50 {
                houses[c] += i * 11;
                c += i;
                if c >= houses.len() {
                    break;
                }
            }
        }
        houses
            .iter()
            .enumerate()
            .find(|(_, presents)| **presents >= 33_100_000)
            .unwrap()
            .0
    }

    #[test]
    fn test_run() {
        assert_eq!((2500..).find(|n| present_count(*n) >= 33_100_000).unwrap(), run());
    }
}

fn main() {
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
    let part1_ans = part1::run();
    println!("Day {} part 1 - {} - took {} milliseconds.", day_number, part1_ans, now.elapsed().as_millis());
    assert_eq!(part1_ans, 776160);

    let now = std::time::Instant::now();
    let part2_ans = part2::run();
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 786240);
}
