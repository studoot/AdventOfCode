#[derive(Debug)]
pub struct Depth(i32);

#[derive(Debug)]
pub struct Horizontal(i32);

#[derive(Debug)]
pub enum Motion {
    Up(i32),
    Down(i32),
    Forward(i32),
}

pub fn parse_motion(line: &str) -> Motion {
    if let Some(rest) = line.strip_prefix("forward ") {
        Motion::Forward(rest.parse::<i32>().unwrap())
    } else if let Some(rest) = line.strip_prefix("down ") {
        Motion::Down(rest.parse::<i32>().unwrap())
    } else if let Some(rest) = line.strip_prefix("up ") {
        Motion::Up(rest.parse::<i32>().unwrap())
    } else {
        panic!("bad input line {}", line);
    }
}

pub fn parse(input: &str) -> Vec<Motion> {
    input.lines().map(parse_motion).collect()
}

pub mod part1 {
    use super::*;

    #[derive(Debug)]
    pub struct Location(pub Horizontal, pub Depth);

    pub fn perform_motion(start: Location, motion: &Motion) -> Location {
        match motion {
            Motion::Down(depth_change) => Location(start.0, Depth(start.1 .0 + depth_change)),
            Motion::Up(depth_change) => Location(start.0, Depth(start.1 .0 - depth_change)),
            Motion::Forward(pos_change) => Location(Horizontal(start.0 .0 + pos_change), start.1),
        }
    }

    pub fn perform_motions(start: Location, motions: &[Motion]) -> Location {
        motions.iter().fold(start, &perform_motion)
    }
}

pub mod part2 {
    use super::*;

    #[derive(Debug)]
    pub struct Aim(pub i32);

    #[derive(Debug)]
    pub struct Location(pub Horizontal, pub Depth, pub Aim);

    pub fn perform_motion(start: Location, motion: &Motion) -> Location {
        match motion {
            Motion::Down(aim_change) => Location(start.0, start.1, Aim(start.2 .0 + aim_change)),
            Motion::Up(aim_change) => Location(start.0, start.1, Aim(start.2 .0 - aim_change)),
            Motion::Forward(pos_change) => {
                Location(Horizontal(start.0 .0 + pos_change), Depth(start.1 .0 + (pos_change * start.2 .0)), start.2)
            }
        }
    }

    pub fn perform_motions(start: Location, motions: &[Motion]) -> Location {
        motions.iter().fold(start, &perform_motion)
    }
}

fn main() {
    let input_string = include_str!("../input.txt");
    let motions = parse(input_string);

    {
        use part1::*;
        let end = perform_motions(Location(Horizontal(0), Depth(0)), &motions);
        println!("Day  2 part 1 - end location is {:?}, multiplied = {}", end, end.0 .0 * end.1 .0);
    }
    {
        use part2::*;
        let end = perform_motions(Location(Horizontal(0), Depth(0), Aim(0)), &motions);
        println!("Day  2 part 2 - end location is {:?}, multiplied = {}", end, end.0 .0 * end.1 .0);
    }
}
