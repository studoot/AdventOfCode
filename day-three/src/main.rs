fn parse_line(line: &str, ones_count: &mut std::vec::Vec<usize>) {
    if line.len() > ones_count.len() {
        ones_count.resize(line.len(), 0);
    }
    for (i, c) in line.chars().enumerate() {
        if c == '1' {
            let index_to_update = line.len() - i - 1;
            ones_count[index_to_update] += 1;
        }
    }
}

pub fn parse(input: &str) -> (Vec<usize>, usize) {
    let mut ones_count = Vec::<usize>::new();
    let mut line_count = 0usize;
    input.lines().for_each(|s| {
        line_count += 1;
        parse_line(s, &mut ones_count)
    });
    (ones_count, line_count)
}

pub fn run(input: &str) -> u32 {
    let (ones_count, line_count) = parse(input);
    let half_line_count = line_count / 2;
    let (gamma, epsilon) = ones_count.iter().enumerate().fold(
        (0u32, 0u32),
        |(gamma, epsilon), (bit_num, ones_count)| {
            if *ones_count >= half_line_count {
                (gamma | 1 << bit_num, epsilon)
            } else {
                (gamma, epsilon | 1 << bit_num)
            }
        },
    );
    gamma*epsilon
}

fn main() {
    let input_string = include_str!("../input.txt");
    println!("Day  3 part 1 - power consumption = {}", run(input_string));
}

#[test]
fn test_run()
{
    let input_string = include_str!("../test.txt");
    assert_eq!(198, run(input_string))
}