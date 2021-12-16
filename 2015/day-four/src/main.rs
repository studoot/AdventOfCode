use std::path::{Component, Path};

mod part1 {
    pub fn run(input: &str) -> usize {
        let mut x = input.to_owned();
        let mut buffer = itoa::Buffer::new();
        let original_length = x.len();
        (0..)
            .find(|i| /* {i%10==5} */ {
                x.truncate(original_length);
                x += buffer.format(*i);
                let hash: [u8; 16] = md5::compute(x.as_bytes()).into();
                hash[0..2] == [0;2] && (hash[2]&0xf0)==0
            })
            .unwrap()
    }

    #[test]
    fn test_run() {
        assert_eq!(609043, run("abcdef"));
        assert_eq!(1048970, run("pqrstuv"));
    }
}

mod part2 {
    pub fn run(input: &str) -> usize {
        let mut x = input.to_owned();
        let mut buffer = itoa::Buffer::new();
        let original_length = x.len();
        (0..)
            .find(|i| /* {i%10==5} */ {
                x.truncate(original_length);
                x += buffer.format(*i);
                let hash: [u8; 16] = md5::compute(x.as_bytes()).into();
                hash[0..3] == [0;3]
            })
            .unwrap()
    }
}

fn main() {
    let input_string = "ckczppom";
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
    assert_eq!(part1_ans, 117946);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 3938038);
}
