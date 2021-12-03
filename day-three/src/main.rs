mod part1 {
    fn parse_line(line: &str, excess_ones: &mut Vec<i32>) {
        if line.len() > excess_ones.len() {
            excess_ones.resize(line.len(), 0);
        }
        for (i, c) in line.chars().enumerate() {
            let index_to_update = line.len() - i - 1;
            if c == '1' {
                excess_ones[index_to_update] += 1;
            } else {
                excess_ones[index_to_update] -= 1;
            }
        }
    }

    pub fn parse(input: &str) -> Vec<i32> {
        let mut excess_ones = Vec::<i32>::new();
        input.lines().for_each(|s| parse_line(s, &mut excess_ones));
        excess_ones
    }

    pub fn run(input: &str) -> u32 {
        let excess_ones = parse(input);

        let (gamma, epsilon) = excess_ones.iter().enumerate().fold(
            (0u32, 0u32),
            |(gamma, epsilon), (bit_num, excess_ones)| {
                if *excess_ones >= 0 {
                    (gamma | 1 << bit_num, epsilon)
                } else {
                    (gamma, epsilon | 1 << bit_num)
                }
            },
        );
        gamma * epsilon
    }
    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(198, run(input_string))
    }
}

mod part2 {
    pub fn parse(input: &str) -> Vec<&str> {
        input.lines().collect::<Vec<_>>()
    }

    pub fn most_common(lines: &[&str], index: usize) -> char {
        let (ones, zeroes) = lines.iter().fold((0, 0), |(ones, zeroes), s| {
            match s.as_bytes()[index] as char {
                '1' => (ones + 1, zeroes),
                '0' => (ones, zeroes + 1),
                _ => (ones, zeroes),
            }
        });
        if ones >= zeroes {
            '1'
        } else {
            '0'
        }
    }
    pub fn least_common(lines: &[&str], index: usize) -> char {
        if most_common(lines, index) == '1' {
            '0'
        } else {
            '1'
        }
    }

    pub fn parse_binary(line: &str) -> u32 {
        line.chars()
            .fold(0, |acc, c| if c == '1' { (acc << 1) | 1 } else { acc << 1 })
    }

    pub fn run(input: &str) -> u32 {
        let lines = parse(input);
        let oxy_rating = get_oxy_rating(lines.clone());
        let co2_rating = get_co2_rating(lines);
        oxy_rating * co2_rating
    }

    fn get_oxy_rating(mut lines: Vec<&str>) -> u32 {
        let mut oxy_rating = 0u32;
        for bit_num in 0..lines.first().unwrap().len() {
            let digit_to_keep = most_common(&lines, bit_num) as u8;
            lines = lines
                .into_iter()
                .filter(|s| s.as_bytes()[bit_num] == digit_to_keep)
                .collect::<Vec<_>>();
            if lines.len() == 1 {
                oxy_rating = parse_binary(lines[0]);
                break;
            }
        }
        oxy_rating
    }
    fn get_co2_rating(mut lines: Vec<&str>) -> u32 {
        let mut co2_rating = 0u32;
        for bit_num in 0..lines.first().unwrap().len() {
            let digit_to_keep = least_common(&lines, bit_num) as u8;
            lines = lines
                .into_iter()
                .filter(|s| s.as_bytes()[bit_num] == digit_to_keep)
                .collect::<Vec<_>>();
            if lines.len() == 1 {
                co2_rating = parse_binary(lines[0]);
                break;
            }
        }
        co2_rating
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(230, run(input_string))
    }
    #[test]
    fn test_most_common() {
        assert_eq!('1', most_common(&["1010", "1111", "0000"], 0));
        assert_eq!('1', most_common(&["1010", "1111", "0000", "0110"], 1));
        assert_eq!('0', most_common(&["1010", "1101", "0000", "0100"], 2));
        assert_eq!(
            '0',
            most_common(
                &["11110", "10110", "10111", "10101", "11100", "10000", "11001",],
                1
            )
        );
        assert_eq!('0', least_common(&["1010", "1111", "0000"], 0));
        assert_eq!('0', least_common(&["1010", "1111", "0000", "0110"], 1));
        assert_eq!('1', least_common(&["1010", "1101", "0000", "0100"], 2));
    }
    #[test]
    fn test_parse_binary() {
        assert_eq!(10, parse_binary("1010"));
        assert_eq!(0b10111101010, parse_binary("10111101010"));
    }
}
fn main() {
    let input_string = include_str!("../input.txt");
    println!(
        "Day  3 part 1 - power consumption = {}",
        part1::run(input_string)
    );
    println!(
        "Day  3 part 2 - life support rating = {}",
        part2::run(input_string)
    );
}
