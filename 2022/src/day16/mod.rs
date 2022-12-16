use itertools::Itertools;
use ndarray::Array2;
use sscanf::sscanf;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};
use std::str::FromStr;

#[derive(Debug)]
struct Valve {
    id: usize,
    flow_rate: usize,
    destinations: Vec<(usize, usize)>,
}

impl FromStr for Valve {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id0, id1, flow_rate,dest_str) =
        sscanf!(s, "Valve {char:/[A-Z]/}{char:/[A-Z]/} has flow rate={usize}; tunnels lead to valves {str:/[A-Z]{2}(?:, [A-Z]{2})+/}").or_else(|_|
        sscanf!(s, "Valve {char:/[A-Z]/}{char:/[A-Z]/} has flow rate={usize}; tunnel leads to valve {str:/[A-Z]{2}/}")).map_err(|e|e.to_string())?;
        let destinations = dest_str
            .split(", ")
            .map(|s| (chars_to_id(s.chars().next().unwrap(), s.chars().nth(1).unwrap()), 1))
            .collect::<Vec<_>>();
        Ok(Valve { id: chars_to_id(id0, id1), flow_rate, destinations })
    }
}

fn chars_to_id(id0: char, id1: char) -> usize {
    ((id0 as usize) << 8) | (id1 as usize)
}

#[allow(dead_code)]
fn id_as_string(id: usize) -> String {
    String::from_iter([((id >> 8) as u8) as char, (id as u8) as char].iter())
}

fn parse(s: &str) -> Vec<Valve> {
    let mut valves = s
        .lines()
        .map(|s| Valve::from_str(s).unwrap_or_else(|e| panic!("{s}: {e}")))
        .sorted_by_key(|v| v.id)
        .collect::<Vec<_>>();

    // Remap valve IDs into indices
    let valve_ids = valves.iter().map(|v| v.id).sorted().collect::<Vec<_>>();
    for v in &mut valves {
        for dest in &mut v.destinations {
            *dest = (valve_ids.iter().position(|id| id == &dest.0).unwrap(), 1);
        }
    }

    // Calculate minimum distances between all the valve locations using Floyd-Warshall
    let mut dist = Array2::<usize>::from_elem((valves.len(), valves.len()), usize::MAX);
    // Initialise distances
    for (i, v) in valves.iter().enumerate() {
        for (p, w) in &v.destinations {
            dist[[i, *p]] = *w;
        }
        dist[[i, i]] = 0;
    }
    // Now calculate minimum distances by enumerating all intermediate paths
    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                if dist[[i, j]] > dist[[i, k]].saturating_add(dist[[k, j]]) {
                    dist[[i, j]] = dist[[i, k]].saturating_add(dist[[k, j]]);
                }
            }
        }
    }

    // Now populate edges between working valves and also from the initial node (at index 0)
    let working_valves = valves
        .iter()
        .enumerate()
        .filter_map(|(pos, v)| (v.flow_rate > 0).then_some(pos))
        .collect::<Vec<_>>();

    for (i, v) in valves.iter_mut().enumerate() {
        v.destinations = working_valves
            .iter()
            .filter_map(|pos| (i != *pos).then_some((*pos, dist[[i, *pos]])))
            .collect();
    }

    valves
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    total_acc_flow: usize,
    time: usize,
    valve_id: usize,
    valves_on: u64,
}

impl State {
    fn is_valve_on(&self, id: usize) -> bool {
        (self.valves_on & (1 << id)) != 0
    }
    fn turn_valve_on(&mut self, id: usize) {
        self.valves_on |= 1 << id;
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        self.total_acc_flow
            .cmp(&other.total_acc_flow)
            .then_with(|| self.time.cmp(&other.time))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1_evaluate(s: &str) -> usize {
    let valves = parse(s);

    let usable_valves = valves.iter().filter(|v| v.flow_rate > 0).count();

    let mut heap = BinaryHeap::new();
    let mut max_flows = BTreeMap::new();

    // We're at `start`, with a zero flow
    heap.push(State { total_acc_flow: 0, time: 0, valve_id: 0, valves_on: 0 });
    while let Some(State { total_acc_flow, time, valve_id: current_valve, valves_on }) = heap.pop() {
        // If all valves are on, we're done...
        if valves_on.count_ones() as usize == usable_valves {
            continue;
        }
        if total_acc_flow < *max_flows.get(&valves_on).unwrap_or(&0) {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a higher flow going through this node
        for (next_valve, distance) in &valves[current_valve].destinations {
            let mut next = State { total_acc_flow, time: time + *distance, valve_id: *next_valve, valves_on };

            if !next.is_valve_on(*next_valve) && next.time < 30 {
                next.time += 1;
                next.total_acc_flow += valves[*next_valve].flow_rate * (30 - next.time);
                next.turn_valve_on(*next_valve);
                if next.total_acc_flow > *max_flows.get(&next.valves_on).unwrap_or(&0) {
                    heap.push(next);
                    max_flows.insert(next.valves_on, next.total_acc_flow);
                }
            }
        }
    }
    *max_flows.values().max().unwrap_or(&0)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State2 {
    total_acc_flow: usize,
    my_valve_id: usize,
    my_time: usize,
    ele_valve_id: usize,
    ele_time: usize,
    valves_on: u64,
}

fn is_valve_on(valves_on: u64, id: usize) -> bool {
    (valves_on & (1 << id)) != 0
}
fn turn_valve_on(valves_on: u64, id: usize) -> u64 {
    valves_on | 1 << id
}

impl Ord for State2 {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        self.total_acc_flow
            .cmp(&other.total_acc_flow)
            .then_with(|| {
                self.my_time
                    .min(self.ele_time)
                    .cmp(&other.my_time.min(other.ele_time))
            })
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part2_evaluate(s: &str) -> usize {
    let valves = parse(s);

    let usable_valves = valves.iter().filter(|v| v.flow_rate > 0).count();

    let mut heap = BinaryHeap::new();
    let mut max_flows = BTreeMap::new();

    // We're at `start`, with a zero flow
    heap.push(State2 { total_acc_flow: 0, my_valve_id: 0, ele_valve_id: 0, my_time: 0, ele_time: 0, valves_on: 0 });
    while let Some(current_state) = heap.pop() {
        if current_state.valves_on.count_ones() as usize == usable_valves {
            continue;
        }
        if current_state.total_acc_flow < *max_flows.get(&current_state.valves_on).unwrap_or(&0) {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a higher flow going through this node
        if current_state.my_time <= current_state.ele_time {
            for (next_valve, distance) in &valves[current_state.my_valve_id].destinations {
                let next_my_time = current_state.my_time + *distance + 1;
                let next_valves_on = turn_valve_on(current_state.valves_on, *next_valve);
                let next_total_acc_flow =
                    current_state.total_acc_flow + valves[*next_valve].flow_rate * (26 - next_my_time);
                if !is_valve_on(current_state.valves_on, *next_valve)
                    && next_my_time < 26
                    && next_total_acc_flow > *max_flows.get(&next_valves_on).unwrap_or(&0)
                {
                    heap.push(State2 {
                        total_acc_flow: next_total_acc_flow,
                        my_valve_id: *next_valve,
                        my_time: next_my_time,
                        valves_on: next_valves_on,
                        ..current_state
                    });
                    max_flows.insert(next_valves_on, next_total_acc_flow);
                };
            }
        } else {
            for (next_valve, distance) in &valves[current_state.ele_valve_id].destinations {
                let next_ele_time = current_state.ele_time + *distance + 1;
                let next_valves_on = turn_valve_on(current_state.valves_on, *next_valve);
                let next_total_acc_flow =
                    current_state.total_acc_flow + valves[*next_valve].flow_rate * (26 - next_ele_time);
                if !is_valve_on(current_state.valves_on, *next_valve)
                    && next_ele_time < 26
                    && next_total_acc_flow > *max_flows.get(&next_valves_on).unwrap_or(&0)
                {
                    heap.push(State2 {
                        total_acc_flow: next_total_acc_flow,
                        ele_valve_id: *next_valve,
                        ele_time: next_ele_time,
                        valves_on: turn_valve_on(current_state.valves_on, *next_valve),
                        ..current_state
                    });
                    max_flows.insert(next_valves_on, next_total_acc_flow);
                }
            }
        }
    }
    *max_flows.values().max().unwrap_or(&0)
}

#[cfg(test)]
const TEST_INPUT_STRING: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
#[cfg(test)]
#[test]
fn test_part1() {
    assert_eq!(part1_evaluate(TEST_INPUT_STRING), 1651);
}

#[test]
fn test_part2() {
    assert_eq!(part2_evaluate(TEST_INPUT_STRING), 1707);
}

pub fn run() -> Option<(usize, bool, usize, bool)> {
    let input_string = include_str!("./input.txt");
    let part1_answer = part1_evaluate(input_string);
    let part2_answer = part2_evaluate(input_string);
    Some((part1_answer, part1_answer == 2330, part2_answer, part2_answer == 2675))
}
