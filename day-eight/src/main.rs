fn parse(input: &str) -> Vec<(Vec<u8>, Vec<u8>)> {
    fn parse_words(input: &str) -> Vec<u8> {
        input
            .split_ascii_whitespace()
            .map(|word| {
                word.chars()
                    .fold(0, |acc, c| acc | (1 << (c as u32 - 'a' as u32)))
            })
            .collect::<Vec<_>>()
    }
    fn parse_line(input: &str) -> (Vec<u8>, Vec<u8>) {
        let (before, after) = input
            .split_once(" | ")
            .unwrap_or_else(|| panic!("No separator in line {}", input));
        (parse_words(before), parse_words(after))
    }

    input.lines().map(parse_line).collect::<Vec<_>>()
}

fn is_unique_segment_pattern(segment_pattern: u8) -> Option<u8> {
    match segment_pattern.count_ones() {
        2 => Some(1),
        3 => Some(7),
        4 => Some(4),
        7 => Some(8),
        _ => None,
    }
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let lines = parse(input);
        lines
            .into_iter()
            .map(|(_before, after)| after)
            .map(|segment_patterns| {
                segment_patterns
                    .into_iter()
                    .filter(|p| is_unique_segment_pattern(*p).is_some())
                    .count()
            })
            .sum()
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(26, run(input_string))
    }
}

mod part2 {
    use super::*;

    #[derive(Default)]
    struct UniqueSegments {
        pub one: u8,
        pub four: u8,
        pub seven: u8,
        pub eight: u8,
        pub six_segments: [u8; 3],
    }
    impl UniqueSegments {
        pub fn new((before, after): &(Vec<u8>, Vec<u8>)) -> UniqueSegments {
            let mut unique_segs: UniqueSegments = Default::default();
            let mut six_segment_count = 0;

            for pattern in before.iter().chain(after.iter()) {
                match pattern.count_ones() {
                    2 => unique_segs.one = *pattern,
                    4 => unique_segs.four = *pattern,
                    3 => unique_segs.seven = *pattern,
                    7 => unique_segs.eight = *pattern,
                    6 => {
                        if !unique_segs.six_segments.contains(pattern) {
                            unique_segs.six_segments[six_segment_count] = *pattern
                        };
                        six_segment_count += 1;
                    }
                    5 => {}
                    _ => panic!("Bad segment pattern {:b}", pattern),
                }
            }
            unique_segs
        }
        fn deduce_segment_mappings(&self) -> [Option<u8>; 128] {
            fn invert(pattern: u8) -> u8 {
                (!pattern) & 0b0111_1111
            }

            let seg_cf = self.one;
            let seg_bd = self.four & invert(self.one);
            let seg_eg = self.eight & invert(self.four) & invert(self.seven);

            let seg_ecd = (self.eight & invert(self.six_segments[0]))
                | (self.eight & invert(self.six_segments[1]))
                | (self.eight & invert(self.six_segments[2]));
            let seg_a = self.seven & invert(self.one);
            let seg_d = seg_ecd & seg_bd;
            let seg_b = seg_bd & invert(seg_d);
            let seg_c = seg_ecd & seg_cf;
            let seg_f = seg_cf & invert(seg_c);
            let seg_e = seg_ecd & seg_eg;
            let seg_g = seg_eg & invert(seg_e);

            let mut digits = [None; 128];
            digits[(seg_a | seg_b | seg_c | seg_e | seg_f | seg_g) as usize] = Some(0u8);
            digits[(seg_c | seg_f) as usize] = Some(1);
            digits[(seg_a | seg_c | seg_d | seg_e | seg_g) as usize] = Some(2);
            digits[(seg_a | seg_c | seg_d | seg_f | seg_g) as usize] = Some(3);
            digits[(seg_b | seg_c | seg_d | seg_f) as usize] = Some(4);
            digits[(seg_a | seg_b | seg_d | seg_f | seg_g) as usize] = Some(5);
            digits[(seg_a | seg_b | seg_d | seg_e | seg_f | seg_g) as usize] = Some(6);
            digits[(seg_a | seg_c | seg_f) as usize] = Some(7);
            digits[(seg_a | seg_b | seg_c | seg_d | seg_e | seg_f | seg_g) as usize] = Some(8);
            digits[(seg_a | seg_b | seg_c | seg_d | seg_f | seg_g) as usize] = Some(9);
            digits
        }
    }

    pub fn run(input: &str) -> usize {
        let lines = parse(input);
        lines
            .into_iter()
            .map(|items| {
                let segment_info = UniqueSegments::new(&items);
                let digit_mapping = segment_info.deduce_segment_mappings();
                items.1.iter().fold(0usize, |acc, segment_pattern| {
                    (acc * 10) + digit_mapping[*segment_pattern as usize].unwrap() as usize
                })
            })
            .sum()
    }

    #[test]
    fn test_line() {
        let input_string = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(5353, run(input_string))
    }
    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(61229, run(input_string))
    }
}
fn main() {
    let input_string = include_str!("../input.txt");
    let part1_ans = part1::run(input_string);
    println!("Day  8 part 1 - {}", part1_ans);
    assert_eq!(part1_ans, 521);
    let part2_ans = part2::run(input_string);
    println!("Day  8 part 2 - {}", part2_ans);
    // assert_eq!(part2_ans, 98231647);
}
