#[derive(Debug, Clone)]
struct Number {
    value: i64,
    id: usize,
}

fn parse(s: &str) -> Vec<Number> {
    s.lines()
        .map(str::parse::<i64>)
        .enumerate()
        .map(|(id, res)| res.map(|value| Number { value, id }))
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|_| panic!("Bad input!"))
}

fn mix_n<SomeType>(nums: &mut Vec<SomeType>, initial_pos: usize, num_shifts: i64) {
    let size = nums.len() as i64;
    let reduced_shifts = num_shifts % (size - 1);
    let mut new_pos = (((initial_pos as i64) + reduced_shifts).rem_euclid(size)) as usize;
    if new_pos > initial_pos && num_shifts < 0 {
        new_pos -= 1;
    } else if new_pos < initial_pos && num_shifts > 0 {
        new_pos += 1;
    }
    if new_pos != initial_pos {
        let value = nums.remove(initial_pos);
        nums.insert(new_pos, value);
    }
}

fn part1_evaluate(s: &str) -> i64 {
    let original = parse(s);
    let mut mixed = original.clone();
    original.iter().for_each(|i| {
        let mix_pos = mixed.iter().position(|n| i.id == n.id).unwrap();
        mix_n(&mut mixed, mix_pos, i.value);
    });

    let zero_pos = mixed.iter().position(|v| v.value == 0).unwrap();
    mixed[(zero_pos + 1000) % mixed.len()].value
        + mixed[(zero_pos + 2000) % mixed.len()].value
        + mixed[(zero_pos + 3000) % mixed.len()].value
}

fn part2_evaluate(s: &str) -> i64 {
    let original = parse(s);
    let mut mixed = original
        .iter()
        .map(|Number { id, value }| Number { id: *id, value: value * 811_589_153 })
        .collect::<Vec<_>>();
    for _ in 0..10 {
        original.iter().for_each(|i| {
            let mix_pos = mixed.iter().position(|n| i.id == n.id).unwrap();
            let mix_value = mixed[mix_pos].value;
            mix_n(&mut mixed, mix_pos, mix_value);
        });
    }

    let zero_pos = mixed.iter().position(|v| v.value == 0).unwrap();
    mixed[(zero_pos + 1000) % mixed.len()].value
        + mixed[(zero_pos + 2000) % mixed.len()].value
        + mixed[(zero_pos + 3000) % mixed.len()].value
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = "\
1
2
-3
3
-2
0
4";
#[cfg(test)]
#[test]
fn test_mix() {
    let mut v1 = vec![1, 2, 3, 4, 5, 6];
    mix_n(&mut v1, 2, 3);
    assert_eq!(&v1, &[1, 2, 4, 5, 6, 3]);
    let mut v1 = vec![1, 2, 4, 3, 5, 6];
    mix_n(&mut v1, 3, 3);
    assert_eq!(&v1, &[1, 3, 2, 4, 5, 6]);
    let mut v1 = vec![1, 2, 3, 4, 5, 6];
    mix_n(&mut v1, 3, 4);
    assert_eq!(&v1, &[1, 2, 4, 3, 5, 6]);
    let mut v1 = vec![1, 2, 3, 5, -1, -2];
    mix_n(&mut v1, 3, 5);
    assert_eq!(&v1, &[1, 2, 3, 5, -1, -2]);
    let mut v1 = vec![1, 2, 3, 6, -1, -2];
    mix_n(&mut v1, 3, 6);
    assert_eq!(&v1, &[1, 2, 3, -1, 6, -2]);
    let mut v1 = vec![1, 2, 3, 7, -1, -2];
    mix_n(&mut v1, 3, 7);
    assert_eq!(&v1, &[1, 2, 3, -1, -2, 7]);
    let mut v1 = vec![1, 2, 3, 8, -1, -2];
    mix_n(&mut v1, 3, 8);
    assert_eq!(&v1, &[1, 8, 2, 3, -1, -2]);
    let mut v1 = vec![1, 2, 3, 9, -1, -2];
    mix_n(&mut v1, 3, 9);
    assert_eq!(&v1, &[1, 2, 9, 3, -1, -2]);
    let mut v1 = vec![1, 2, 3, 10, -1, -2];
    mix_n(&mut v1, 3, 10);
    assert_eq!(&v1, &[1, 2, 3, 10, -1, -2]);
    let mut v1 = vec![1, 2, 3, 11, -1, -2];
    mix_n(&mut v1, 3, 11);
    assert_eq!(&v1, &[1, 2, 3, -1, 11, -2]);
    let mut v1 = vec![1, 2, 3, 12, -1, -2];
    mix_n(&mut v1, 3, 12);
    assert_eq!(&v1, &[1, 2, 3, -1, -2, 12]);
    let mut v1 = vec![1, 2, 3, 12, -1, -2];
    mix_n(&mut v1, 3, 13);
    assert_eq!(&v1, &[1, 12, 2, 3, -1, -2]);
    let mut v1 = vec![1, 2, 3, 12, -1, -2];
    mix_n(&mut v1, 3, 14);
    assert_eq!(&v1, &[1, 2, 12, 3, -1, -2]);
    let mut v1 = vec![1, 2, 3, 12, -1, -2];
    mix_n(&mut v1, 3, 15);
    assert_eq!(&v1, &[1, 2, 3, 12, -1, -2]);

    let mut v1 = vec![1, 2, 3, 12, -1, -2];
    mix_n(&mut v1, 3, -1);
    assert_eq!(&v1, &[1, 2, 12, 3, -1, -2]);

    let mut v1 = vec![1, 2, 3, 12, -1, -2];
    mix_n(&mut v1, 3, -2);
    assert_eq!(&v1, &[1, 12, 2, 3, -1, -2]);
    let mut v1 = vec![1, 2, 3, 12, -1, -2];
    mix_n(&mut v1, 3, -3);
    assert_eq!(&v1, &[12, 1, 2, 3, -1, -2]);
    let mut v1 = vec![1, 2, 3, 12, -1, -2];
    mix_n(&mut v1, 3, -4);
    assert_eq!(&v1, &[1, 2, 3, -1, 12, -2]);
    let mut v1 = vec![1, 2, 3, 12, -1, -2];
    mix_n(&mut v1, 3, -5);
    assert_eq!(&v1, &[1, 2, 3, 12, -1, -2]);

    let mut v1 = vec![1, 2, 3, 12, -1, -2];
    mix_n(&mut v1, 3, -6);
    assert_eq!(&v1, &[1, 2, 12, 3, -1, -2]);

    let mut v1 = vec![1, 2, 3, 12, -1, -2];
    mix_n(&mut v1, 3, -7);
    assert_eq!(&v1, &[1, 12, 2, 3, -1, -2]);

    let mut v1 = vec![1, 2, 3, 12, -1, -2];
    mix_n(&mut v1, 3, -8);
    assert_eq!(&v1, &[12, 1, 2, 3, -1, -2]);

    let mut v1 = vec![1, 2, 3, 12, -1, -2];
    mix_n(&mut v1, 3, -9);
    assert_eq!(&v1, &[1, 2, 3, -1, 12, -2]);

    let mut v1 = vec![1, 2, 3, 12, -1, -2];
    mix_n(&mut v1, 3, -10);
    assert_eq!(&v1, &[1, 2, 3, 12, -1, -2]);

    let mut v1 = vec![1, 2, 3, 12, -1, -2];
    mix_n(&mut v1, 3, -11);
    assert_eq!(&v1, &[1, 2, 12, 3, -1, -2]);

    let mut v1 = vec![1, 2, 3, 12, -1, -2];
    mix_n(&mut v1, 3, -12);
    assert_eq!(&v1, &[1, 12, 2, 3, -1, -2]);
}

#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1_evaluate(TEST_INPUT_STRING), 3);
}

#[test]
fn test_part2() {
    assert_eq!(part2_evaluate(TEST_INPUT_STRING), 1623178306);
}

pub fn run() -> Option<(i64, bool, i64, bool)> {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string);
    let part2_answer = part2_evaluate(input_string);
    Some((part1_answer, part1_answer == 7228, part2_answer, part2_answer == 4526232706281))
}
