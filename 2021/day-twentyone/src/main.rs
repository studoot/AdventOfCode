use itertools::Itertools;
use std::path::{Component, Path};

#[derive(Debug, Clone, Copy)]
struct PlayerState {
    pos: usize,
    score: usize,
    rolls: usize,
}

impl PlayerState {
    fn new(start_pos: usize) -> Self {
        PlayerState { pos: start_pos, score: 0, rolls: 0 }
    }

    fn play_round(&mut self, total_dice_score: usize) {
        self.pos = (((self.pos - 1) + total_dice_score) % 10) + 1;
        self.score += self.pos;
        self.rolls += 3;
    }
    fn play_round_to_new(&self, total_dice_score: usize) -> Self {
        let new_pos = (((self.pos - 1) + total_dice_score) % 10) + 1;
        PlayerState { pos: new_pos, score: self.score + new_pos, rolls: self.rolls + 3 }
    }
}

fn parse(input: &str) -> (PlayerState, PlayerState) {
    let mut lines = input
        .lines()
        .map(|s| s.split_once(": ").unwrap().1.parse::<usize>().unwrap());
    let p0 = lines.next().unwrap();
    let p1 = lines.next().unwrap();
    (PlayerState::new(p0), PlayerState::new(p1))
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let (mut p0, mut p1) = parse(input);
        (1..=100)
            .cycle()
            .chunks(6)
            .into_iter()
            .find_map(|mut c| {
                let p0_rolls = c.next().unwrap() + c.next().unwrap() + c.next().unwrap();
                p0.play_round(p0_rolls);
                if p0.score >= 1000 {
                    return Some(p1.score * (p0.rolls + p1.rolls));
                }
                let p1_rolls = c.next().unwrap() + c.next().unwrap() + c.next().unwrap();
                p1.play_round(p1_rolls);
                if p1.score >= 1000 {
                    return Some(p0.score * (p0.rolls + p1.rolls));
                }
                None
            })
            .unwrap()
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(739785, run(input_string))
    }
    #[test]
    fn test_bits() {
        let input_string = include_str!("../test.txt");
        let (mut p0, mut p1) = parse(input_string);

        p0.play_round(1 + 2 + 3);
        assert_eq!(p0.pos, 10);
        assert_eq!(p0.score, 10);
        assert_eq!(p0.rolls, 3);
        p1.play_round(4 + 5 + 6);
        assert_eq!(p1.pos, 3);
        assert_eq!(p1.score, 3);
        assert_eq!(p1.rolls, 3);
        p0.play_round(7 + 8 + 9);
        assert_eq!(p0.pos, 4);
        assert_eq!(p0.score, 14);
        assert_eq!(p0.rolls, 6);
    }
}

mod part2 {
    use std::collections::HashMap;
    use std::slice::from_ref;

    use super::*;

    struct Score(usize, usize);

    #[derive(Debug, Clone)]
    struct GameState {
        p0: PlayerState,
        p1: PlayerState,
        done: bool,
        outcomes: u64,
    }
    // Potential outcomes from 3 rolls of a 3-sided die:
    //         Score	Outcome Frequency
    //             3             1
    //             4             3
    //             5             6
    //             6             7
    //             7             6
    //             8             3
    //             9             1
    const SINGLE_ROUND_SCORES: [Score; 7] =
        [Score(3, 1), Score(4, 3), Score(5, 6), Score(6, 7), Score(7, 6), Score(8, 3), Score(9, 1)];
    const WINNING_SCORE: usize = 21;

    pub fn run(input: &str) -> u64 {
        let (p0, p1) = parse(input);
        let mut game_states = vec![GameState { p0, p1, done: false, outcomes: 1 }];
        loop {
            let game_states_count = game_states.len();
            game_states = game_states
                .into_iter()
                .flat_map(|game| {
                    if game.done {
                        vec![game]
                    } else {
                        from_ref(&game)
                            .iter()
                            .cartesian_product(SINGLE_ROUND_SCORES.iter())
                            .map(|(game, score)| {
                                let mut new_p0_state = game.p0.play_round_to_new(score.0);
                                new_p0_state.score = new_p0_state.score.min(WINNING_SCORE);
                                GameState {
                                    p0: new_p0_state,
                                    p1: game.p1,
                                    done: new_p0_state.score >= WINNING_SCORE,
                                    outcomes: game.outcomes * (score.1 as u64),
                                }
                            })
                            .collect_vec()
                    }
                })
                .fold(
                    HashMap::<(usize, usize, usize, usize), GameState>::with_capacity(game_states_count),
                    |mut map, g| {
                        map.entry((g.p0.score, g.p0.pos, g.p1.score, g.p1.pos))
                            .and_modify(|e| {
                                (*e).outcomes += g.outcomes;
                            })
                            .or_insert(g);
                        map
                    },
                )
                .into_values()
                .collect_vec();

            if game_states.iter().all(|g| g.done) {
                break;
            }
            let game_states_count = game_states.len();
            game_states = game_states
                .into_iter()
                .flat_map(|game| {
                    if game.done {
                        vec![game]
                    } else {
                        from_ref(&game)
                            .iter()
                            .cartesian_product(SINGLE_ROUND_SCORES.iter())
                            .map(|(game, score)| {
                                let mut new_p1_state = game.p1.play_round_to_new(score.0);
                                new_p1_state.score = new_p1_state.score.min(WINNING_SCORE);
                                GameState {
                                    p0: game.p0,
                                    p1: new_p1_state,
                                    done: new_p1_state.score >= WINNING_SCORE,
                                    outcomes: game.outcomes * (score.1 as u64),
                                }
                            })
                            .collect_vec()
                    }
                })
                .fold(
                    HashMap::<(usize, usize, usize, usize), GameState>::with_capacity(game_states_count),
                    |mut map, g| {
                        map.entry((g.p0.score, g.p0.pos, g.p1.score, g.p1.pos))
                            .and_modify(|e| {
                                (*e).outcomes += g.outcomes;
                            })
                            .or_insert(g);
                        map
                    },
                )
                .into_values()
                .collect_vec();

            if game_states.iter().all(|g| g.done) {
                break;
            }
        }
        let outcomes = game_states.into_iter().fold((0, 0), |acc, g| {
            if g.p0.score >= WINNING_SCORE {
                (acc.0 + g.outcomes, acc.1)
            } else {
                (acc.0, acc.1 + g.outcomes)
            }
        });
        outcomes.0.max(outcomes.1)
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(444356092776315, run(input_string))
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
    let part1_ans = part1::run(input_string);
    println!("Day {} part 1 - {} - took {} milliseconds.", day_number, part1_ans, now.elapsed().as_millis());
    assert_eq!(part1_ans, 864900);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 575111835924670);
}
