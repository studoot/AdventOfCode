use std::path::{Component, Path};

type Password = [u8; 8];

fn to_password(s: &str) -> Password {
    [
        *s.as_bytes().get(0).unwrap(),
        *s.as_bytes().get(1).unwrap(),
        *s.as_bytes().get(2).unwrap(),
        *s.as_bytes().get(3).unwrap(),
        *s.as_bytes().get(4).unwrap(),
        *s.as_bytes().get(5).unwrap(),
        *s.as_bytes().get(6).unwrap(),
        *s.as_bytes().get(7).unwrap(),
    ]
}

fn increment(password: &mut Password) {
    for b in password.iter_mut().rev() {
        match *b {
            b'z' => *b = b'a',
            _ => {
                *b += 1;
                break;
            }
        }
    }
}

fn is_valid(password: &Password) -> bool {
    let mut illegal_letters_found = false;
    let mut triplet_found = false;
    let mut pair1_found = false;
    let mut pair2_found = false;
    for i in 0..8 {
        illegal_letters_found |= password[i] == b'i' || password[i] == b'l' || password[i] == b'o';
        triplet_found |= i < 6 && password[i] + 1 == password[i + 1] && password[i] + 2 == password[i + 2];
        if !pair1_found {
            pair1_found |= i > 0 && password[i] == password[i - 1];
        } else {
            pair2_found |= i < 7 && password[i] == password[i + 1];
        };
    }
    triplet_found && pair1_found && pair2_found && !illegal_letters_found
}

fn new_password(from: &str) -> String {
    let mut p = to_password(from);

    loop {
        increment(&mut p);
        if is_valid(&p) {
            return p.into_iter().map(|b| b as char).collect::<String>();
        }
    }
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> String {
        new_password(input)
    }

    #[test]
    fn test_run() {
        assert!(!is_valid(&to_password("hijklmmn")));
        assert!(!is_valid(&to_password("abbceffg")));
        assert!(!is_valid(&to_password("abbceffg")));
        assert!(!is_valid(&to_password("abbcegjk")));
        assert!(is_valid(&to_password("abcdffaa")));
        assert!(is_valid(&to_password("ghjaabcc")));

        assert_eq!(new_password("abcdefgh"), "abcdffaa");
        assert_eq!(new_password("ghijklmn"), "ghjaabcc");
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> String {
        new_password(input)
    }
}

fn main() {
    let input_string = "hepxcrrq";
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
    assert_eq!(part1_ans, "hepxxyzz");

    let now = std::time::Instant::now();
    let part2_ans = part2::run("hepxxyzz");
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, "heqaabcc");
}
