use std::path::{Component, Path};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Burrow {
    locations: Vec<char>,
    room_size: usize,
}
const ROOM_COUNT: usize = 4;
const HALL_SIZE: usize = 11;
impl Burrow {
    fn new(room_size: usize) -> Self {
        let locations = vec!['.'; HALL_SIZE + ROOM_COUNT * room_size];
        Burrow { locations, room_size }
    }

    fn room_a(&self) -> &[char] {
        &self.locations[HALL_SIZE..HALL_SIZE + self.room_size]
    }
    fn room_b(&self) -> &[char] {
        &self.locations[HALL_SIZE + self.room_size..HALL_SIZE + self.room_size * 2]
    }
    fn room_c(&self) -> &[char] {
        &self.locations[HALL_SIZE + self.room_size * 2..HALL_SIZE + self.room_size * 3]
    }
    fn room_d(&self) -> &[char] {
        &self.locations[HALL_SIZE + self.room_size * 3..HALL_SIZE + self.room_size * 4]
    }

    fn is_at_end_state(&self) -> bool {
        self.room_a().iter().all(|c| *c == 'A')
            && self.room_b().iter().all(|c| *c == 'B')
            && self.room_c().iter().all(|c| *c == 'C')
            && self.room_d().iter().all(|c| *c == 'D')
    }

    fn can_move_from_room(&self, pos: usize, c: char) -> bool {
        let (room_start, room_from_pos) = if (HALL_SIZE..HALL_SIZE + self.room_size).contains(&pos) {
            (HALL_SIZE, self.room_a())
        } else if (HALL_SIZE + self.room_size..HALL_SIZE + self.room_size * 2).contains(&pos) {
            (HALL_SIZE + self.room_size, self.room_b())
        } else if (HALL_SIZE + self.room_size * 2..HALL_SIZE + self.room_size * 3).contains(&pos) {
            (HALL_SIZE + self.room_size * 2, self.room_c())
        } else if (HALL_SIZE + self.room_size * 3..HALL_SIZE + self.room_size * 4).contains(&pos) {
            (HALL_SIZE + self.room_size * 3, self.room_d())
        } else {
            panic!("Bad location {}", pos)
        };
        let room_from_char = match c {
            'A' => self.room_a(),
            'B' => self.room_b(),
            'C' => self.room_c(),
            'D' => self.room_d(),
            _ => panic!("Bad colour {}", c),
        };
        // If the position is at an end state, it can't move
        if room_from_char.as_ptr() == room_from_pos.as_ptr()
            && room_from_pos[0..=pos - room_start].iter().all(|r| *r == c)
        {
            false
        }
        // Is the position clear to move out to the hallway?
        else {
            assert_eq!(room_from_pos[pos - room_start], c);
            room_from_pos[pos + 1 - room_start..self.room_size]
                .iter()
                .all(|r| *r == '.')
        }
    }

    fn end_point(&self, c: char) -> Option<usize> {
        fn find_end_point(room: &[char], c: char, offset: usize) -> Option<usize> {
            if room.iter().all(|r| *r == '.' || *r == c) {
                room.iter().position(|r| *r == '.').map(|i| i + offset)
            } else {
                None
            }
        }
        match c {
            'A' => find_end_point(self.room_a(), c, HALL_SIZE),
            'B' => find_end_point(self.room_b(), c, HALL_SIZE + self.room_size),
            'C' => find_end_point(self.room_c(), c, HALL_SIZE + self.room_size * 2),
            'D' => find_end_point(self.room_d(), c, HALL_SIZE + self.room_size * 3),
            _ => panic!("Bad colour {}", c),
        }
    }

    fn dist(&self, from: usize, to: usize) -> usize {
        let (hall_entry, room_exit_dist) = if from >= HALL_SIZE {
            (self.junction_location(from), self.room_size - ((from - HALL_SIZE) % self.room_size))
        } else {
            (from, 0)
        };
        let (hall_exit, room_entry_dist) = if to >= HALL_SIZE {
            (self.junction_location(to), self.room_size - ((to - HALL_SIZE) % self.room_size))
        } else {
            (to, 0)
        };
        room_exit_dist + (hall_entry.max(hall_exit) - hall_entry.min(hall_exit)) + room_entry_dist
    }

    fn cost(&self, c: char) -> usize {
        match c {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => panic!("Bad colour {}", c),
        }
    }

    fn junction_location(&self, p: usize) -> usize {
        let room_number = (p - HALL_SIZE) / self.room_size;
        match room_number {
            0 => 2,
            1 => 4,
            2 => 6,
            3 => 8,
            _ => panic!("Bad location {}", p),
        }
    }

    fn is_junction_location(&self, p: usize) -> bool {
        matches!(p, 2 | 4 | 6 | 8)
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!(
            "
#############
#{}#
###{}#{}#{}#{}###",
            self.locations[0..=10].iter().collect::<String>(),
            self.locations[HALL_SIZE + self.room_size - 1],
            self.locations[HALL_SIZE + self.room_size * 2 - 1],
            self.locations[HALL_SIZE + self.room_size * 3 - 1],
            self.locations[HALL_SIZE + self.room_size * 4 - 1]
        );
        for i in 1..self.room_size {
            println!(
                "  #{}#{}#{}#{}#",
                self.locations[HALL_SIZE + self.room_size - i - 1],
                self.locations[HALL_SIZE + self.room_size * 2 - i - 1],
                self.locations[HALL_SIZE + self.room_size * 3 - i - 1],
                self.locations[HALL_SIZE + self.room_size * 4 - i - 1]
            );
        }
        println!("  #########\n\n");
    }
    fn get_moves(&self, pos: usize) -> Vec<usize> {
        assert_ne!(self.locations[pos], '.');
        let c = self.locations[pos];

        if pos < HALL_SIZE {
            // Starting within the hallway - find if an end point is available.
            if let Some(e) = self.end_point(c) {
                let junction = self.junction_location(e);
                let mut r = if pos < junction { pos + 1..=junction } else { junction..=pos - 1 };
                if !r.any(|p| self.locations[p] != '.') {
                    vec![e]
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        } else if self.can_move_from_room(pos, c) {
            // In a room and it can be moved.

            let hall_entry = self.junction_location(pos);

            // Find if an end point is available.
            if let Some(e) = self.end_point(c) {
                let hall_exit = self.junction_location(e);
                let hall_min = hall_entry.min(hall_exit);
                let hall_max = hall_entry.max(hall_exit);
                if self.locations[hall_min..=hall_max]
                    .iter()
                    .all(|c| *c == '.')
                {
                    return vec![e];
                }
            }
            let min = self.locations[0..=hall_entry]
                .iter()
                .rposition(|c| *c != '.')
                .map(|p| p + 1)
                .unwrap_or(0);
            let max = (hall_entry..=10)
                .position(|p| self.locations[p] != '.')
                .map(|p| hall_entry + p - 1)
                .unwrap_or(10);
            (min..=max)
                .into_iter()
                .filter(|p| !self.is_junction_location(*p))
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        }
    }

    fn do_moves(&mut self) -> Option<usize> {
        let amphs = (0..self.locations.len())
            .filter(|pos| self.locations[*pos] != '.')
            .collect::<Vec<_>>();
        let mut min_cost = None;
        for amph in amphs {
            let moves = self.get_moves(amph);
            if !moves.is_empty() {
                let cost_factor = self.cost(self.locations[amph]);
                for this_move in moves {
                    let move_cost = (self.dist(amph, this_move)) * cost_factor;
                    self.locations.swap(amph, this_move);
                    if self.is_at_end_state() {
                        min_cost = Some(move_cost.min(min_cost.unwrap_or(usize::MAX)));
                    } else {
                        min_cost = match (min_cost, self.do_moves()) {
                            (Some(existing_min), Some(new_cost)) => Some(existing_min.min(move_cost + new_cost)),
                            (None, Some(new_cost)) => Some(move_cost + new_cost),
                            (Some(existing_min), None) => Some(existing_min),
                            (None, None) => None,
                        };
                    }
                    self.locations.swap(amph, this_move);
                }
            }
        }
        min_cost
    }
}

fn parse(input: &str) -> Burrow {
    let mut new_burrow = Burrow::new(2);
    let rooms_1 = input.lines().nth(2).unwrap();
    new_burrow.locations[12] = rooms_1.chars().nth(3).unwrap();
    new_burrow.locations[14] = rooms_1.chars().nth(5).unwrap();
    new_burrow.locations[16] = rooms_1.chars().nth(7).unwrap();
    new_burrow.locations[18] = rooms_1.chars().nth(9).unwrap();
    let rooms_0 = input.lines().nth(3).unwrap();
    new_burrow.locations[11] = rooms_0.chars().nth(3).unwrap();
    new_burrow.locations[13] = rooms_0.chars().nth(5).unwrap();
    new_burrow.locations[15] = rooms_0.chars().nth(7).unwrap();
    new_burrow.locations[17] = rooms_0.chars().nth(9).unwrap();
    new_burrow
}

mod part1 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let mut burrow = parse(input);
        burrow.do_moves().unwrap()
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(12521, run(input_string))
    }
}

mod part2 {
    use super::*;

    pub fn run(input: &str) -> usize {
        let parsed_burrow = parse(input);
        let mut burrow = Burrow::new(4);
        burrow.locations[11] = parsed_burrow.locations[11];
        burrow.locations[15] = parsed_burrow.locations[13];
        burrow.locations[19] = parsed_burrow.locations[15];
        burrow.locations[23] = parsed_burrow.locations[17];

        burrow.locations[12] = 'D';
        burrow.locations[16] = 'B';
        burrow.locations[20] = 'A';
        burrow.locations[24] = 'C';

        burrow.locations[13] = 'D';
        burrow.locations[17] = 'C';
        burrow.locations[21] = 'B';
        burrow.locations[25] = 'A';

        burrow.locations[14] = parsed_burrow.locations[12];
        burrow.locations[18] = parsed_burrow.locations[14];
        burrow.locations[22] = parsed_burrow.locations[16];
        burrow.locations[26] = parsed_burrow.locations[18];

        burrow.do_moves().unwrap()
    }

    #[test]
    fn test_run() {
        let input_string = include_str!("../test.txt");
        assert_eq!(44169, run(input_string))
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
    assert_eq!(part1_ans, 13066);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 47328);
}
