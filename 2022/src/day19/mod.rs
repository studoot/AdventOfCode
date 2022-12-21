use rayon::prelude::*;
use sscanf::sscanf;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Robot {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Blueprint {
    id: usize,
    ore: Robot,
    clay: Robot,
    obsidian: Robot,
    geode: Robot,
    max_needs: Robot,
}

impl FromStr for Blueprint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        sscanf!(s, "Blueprint {usize}: Each ore robot costs {usize} ore. Each clay robot costs {usize} ore. Each obsidian robot costs {usize} ore and {usize} clay. Each geode robot costs {usize} ore and {usize} obsidian.")
            .map_err(|e| e.to_string())
            .map(|(id, ore_ore,clay_ore,obsidian_ore,obsidian_clay,geode_ore,geode_obsidian)| Blueprint{
                id,
                ore:Robot{ore:ore_ore,clay:0,obsidian:0,},
                clay:Robot{ore:clay_ore,clay:0,obsidian:0,},
                obsidian:Robot{ore:obsidian_ore,clay:obsidian_clay,obsidian:0,},
                geode:Robot{ore:geode_ore,clay:0,obsidian:geode_obsidian,},
                max_needs:Robot{
                    ore:ore_ore.max(clay_ore).max(obsidian_ore).max(geode_ore),
                    clay:obsidian_clay,
                    obsidian:geode_obsidian,
                },
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl State {
    fn new() -> Self {
        State {
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
}

#[derive(Debug)]
enum TryToBuild {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

fn timestep(mut time_left: usize, b: &Blueprint, mut state: State, will_build: TryToBuild) -> usize {
    loop {
        if time_left == 0 {
            return state.geode;
        }
        time_left -= 1;
        let will_construct = match will_build {
            TryToBuild::Ore => state.ore >= b.ore.ore,
            TryToBuild::Clay => state.ore >= b.clay.ore,
            TryToBuild::Obsidian => state.ore >= b.obsidian.ore && state.clay >= b.obsidian.clay,
            TryToBuild::Geode => state.ore >= b.geode.ore && state.obsidian >= b.geode.obsidian,
        };

        state.ore += state.ore_robots;
        state.clay += state.clay_robots;
        state.obsidian += state.obsidian_robots;
        state.geode += state.geode_robots;

        if will_construct {
            match will_build {
                TryToBuild::Ore => {
                    state.ore -= b.ore.ore;
                    state.ore_robots += 1;
                }
                TryToBuild::Clay => {
                    state.clay_robots += 1;
                    state.ore -= b.clay.ore;
                }
                TryToBuild::Obsidian => {
                    state.obsidian_robots += 1;
                    state.ore -= b.obsidian.ore;
                    state.clay -= b.obsidian.clay;
                }
                TryToBuild::Geode => {
                    state.geode_robots += 1;
                    state.ore -= b.geode.ore;
                    state.obsidian -= b.geode.obsidian;
                }
            }
            break;
        }
    }

    let mut max_geodes = state.geode;
    for try_to_build in [TryToBuild::Ore, TryToBuild::Clay, TryToBuild::Obsidian, TryToBuild::Geode] {
        match try_to_build {
            TryToBuild::Ore if state.ore_robots >= b.max_needs.ore => {
                continue;
            }
            TryToBuild::Clay if state.clay_robots >= b.max_needs.clay => continue,
            TryToBuild::Obsidian if state.obsidian_robots >= b.max_needs.obsidian || state.clay_robots == 0 => continue,
            TryToBuild::Geode if state.obsidian_robots == 0 || state.clay_robots == 0 => continue,
            _ => max_geodes = max_geodes.max(timestep(time_left, b, state.clone(), try_to_build)),
        }
    }
    max_geodes
}

fn simulate(b: &Blueprint, time_allowed: usize) -> usize {
    let quantity_ore = timestep(time_allowed, b, State::new(), TryToBuild::Ore);
    let quantity_clay = timestep(time_allowed, b, State::new(), TryToBuild::Clay);
    quantity_ore.max(quantity_clay)
}

fn parse(s: &str) -> Vec<Blueprint> {
    s.lines()
        .map(|l| Blueprint::from_str(l).unwrap_or_else(|e| panic!("Bad blueprint: '{l}' - {e}")))
        .collect::<Vec<_>>()
}

fn part1_evaluate(s: &str) -> usize {
    let blueprints = parse(s);
    blueprints
        .par_iter()
        .map(|b| b.id * simulate(b, 24))
        .sum::<usize>()
}

fn part2_evaluate(s: &str) -> usize {
    let blueprints = parse(s);
    blueprints
        .par_iter()
        .take(3)
        .map(|b| simulate(b, 32))
        .product::<usize>()
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = "\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
#[cfg(test)]
#[test]
fn test_part1() {
    let blueprints = parse(TEST_INPUT_STRING);

    let now = std::time::Instant::now();
    assert_eq!(dbg!(simulate(&blueprints[0], 24)), 9);
    let duration = now.elapsed().as_micros();
    println!("Took {duration} microseconds");

    let now = std::time::Instant::now();
    assert_eq!(dbg!(simulate(&blueprints[1], 24)), 12);
    let duration = now.elapsed().as_micros();
    println!("Took {duration} microseconds");

    let now = std::time::Instant::now();
    assert_eq!(dbg!(part1_evaluate(TEST_INPUT_STRING)), 33);
    let duration = now.elapsed().as_micros();
    println!("Took {duration} microseconds");
}

#[test]
fn test_part2() {
    let blueprints = parse(TEST_INPUT_STRING);

    // let now = std::time::Instant::now();
    // assert_eq!(dbg!(simulate(&blueprints[0], 32)), 56);
    // let duration = now.elapsed().as_micros();
    // println!("Took {duration} microseconds");

    let now = std::time::Instant::now();
    assert_eq!(dbg!(simulate(&blueprints[1], 32)), 62);
    let duration = now.elapsed().as_micros();
    println!("Took {duration} microseconds");
}

pub fn run() -> Option<(usize, bool, usize, bool)> {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string);
    let part2_answer = part2_evaluate(input_string);
    Some((part1_answer, part1_answer == 1962, part2_answer, part2_answer == 88160))
}
