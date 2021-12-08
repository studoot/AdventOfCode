pub fn parse(input: &str) -> (usize, Vec<usize>) {
    let data = input
        .lines()
        .map(|s| usize::from_str_radix(s, 2).unwrap())
        .collect::<Vec<_>>();
    let max_bit_count = input
        .lines()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .len();
    (max_bit_count, data)
}

pub fn bit_is_set(v: usize, bit_num: usize) -> bool {
    (v & (1 << bit_num)) != 0
}

pub fn set_bit(v: usize, bit_num: usize) -> usize {
    v | (1 << bit_num)
}

mod part1 {
    use super::*;
    fn process_data_value(value: usize, mut excess_ones: Vec<isize>) -> Vec<isize> {
        for (bit_num, current) in excess_ones.iter_mut().enumerate() {
            if bit_is_set(value, bit_num) {
                *current += 1;
            } else {
                *current -= 1;
            }
        }
        excess_ones
    }

    pub fn run(input: &str) -> usize {
        let (bit_count, data) = parse(input);
        let excess_ones = data
            .into_iter()
            .fold(vec![0isize; bit_count], |excess_ones, value| {
                process_data_value(value, excess_ones)
            });

        let (gamma, epsilon) = excess_ones.into_iter().enumerate().fold(
            (0usize, 0usize),
            |(gamma, epsilon), (bit_num, excess_ones)| {
                if excess_ones >= 0 {
                    (set_bit(gamma, bit_num), epsilon)
                } else {
                    (gamma, set_bit(epsilon, bit_num))
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
    use super::*;

    pub fn most_common_bit_value(data: &[usize], bit_num: usize) -> bool {
        let (ones, zeroes) = data.iter().fold((0, 0), |(ones, zeroes), v| {
            if bit_is_set(*v, bit_num) {
                (ones + 1, zeroes)
            } else {
                (ones, zeroes + 1)
            }
        });
        ones >= zeroes
    }
    pub fn least_common_bit_value(data: &[usize], bit_num: usize) -> bool {
        !most_common_bit_value(data, bit_num)
    }

    pub fn run(input: &str) -> usize {
        let (bit_count, data) = parse(input);
        get_oxy_rating(bit_count, data.clone()) * get_co2_rating(bit_count, data)
    }

    fn filter_on_bit(bit_num: usize, should_keep_set_bits: bool, data: Vec<usize>) -> Vec<usize> {
        data.into_iter()
            .filter(|v| bit_is_set(*v, bit_num) == should_keep_set_bits)
            .collect::<Vec<_>>()
    }

    fn get_oxy_rating(bit_count: usize, mut data: Vec<usize>) -> usize {
        let mut oxy_rating = 0usize;
        for bit_num in (0..bit_count).rev() {
            let should_keep_set_bit = most_common_bit_value(&data, bit_num);
            data = filter_on_bit(bit_num, should_keep_set_bit, data);
            if data.len() == 1 {
                oxy_rating = data[0];
                break;
            }
        }
        oxy_rating
    }

    fn get_co2_rating(bit_count: usize, mut data: Vec<usize>) -> usize {
        let mut co2_rating = 0usize;
        for bit_num in (0..bit_count).rev() {
            let should_keep_set_bit = least_common_bit_value(&data, bit_num);
            data = filter_on_bit(bit_num, should_keep_set_bit, data);
            if data.len() == 1 {
                co2_rating = data[0];
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
        assert_eq!(true, most_common_bit_value(&[0b1010, 0b1111, 0b0000], 3));
        assert_eq!(
            true,
            most_common_bit_value(&[0b1010, 0b1111, 0b0000, 0b0110], 2)
        );
        assert_eq!(
            false,
            most_common_bit_value(&[0b1010, 0b1101, 0b0000, 0b0100], 1)
        );
        assert_eq!(
            false,
            most_common_bit_value(
                &[0b11110, 0b10110, 0b10111, 0b10101, 0b11100, 0b10000, 0b11001],
                3
            )
        );
        assert_eq!(false, least_common_bit_value(&[0b1010, 0b1111, 0b0000], 3));
        assert_eq!(
            false,
            least_common_bit_value(&[0b1010, 0b1111, 0b0000, 0b0110], 2)
        );
        assert_eq!(
            true,
            least_common_bit_value(&[0b1010, 0b1101, 0b0000, 0b0100], 1)
        );
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
