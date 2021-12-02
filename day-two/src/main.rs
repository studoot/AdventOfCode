#[derive(Debug)]
struct Depth(i32);

#[derive(Debug)]
struct Horizontal(i32);

#[derive(Debug)]
struct Location(Horizontal, Depth);

#[derive(Debug)]
enum Motion {
    Up(i32),
    Down(i32),
    Forward(i32),
}

fn parse_motion(line: &str) -> Motion {
    if line.starts_with("forward ") {
        Motion::Forward(line[8..].parse::<i32>().unwrap())
    } else if line.starts_with("down ") {
        Motion::Down(line[5..].parse::<i32>().unwrap())
    } else if line.starts_with("up ") {
        Motion::Up(line[3..].parse::<i32>().unwrap())
    } else {
        panic!("bad input line {}", line);
    }
}

fn parse(input: &str) -> Vec<Motion> {
    input.lines().map(|s| parse_motion(s)).collect()
}

fn perform_motion(start: Location, motion: &Motion) -> Location {
    match motion {
        Motion::Down(depth_change) => Location(start.0, Depth(start.1 .0 + depth_change)),
        Motion::Up(depth_change) => Location(start.0, Depth(start.1 .0 - depth_change)),
        Motion::Forward(pos_change) => Location(Horizontal(start.0 .0 + pos_change), start.1),
    }
}

fn perform_motions(start: Location, motions: &[Motion]) -> Location {
    motions.iter().fold(start, &perform_motion)
}

fn main() {
    let input_string = include_str!("../input.txt");
    let motions = parse(input_string);

    let end = perform_motions(Location(Horizontal(0), Depth(0)), &motions);

    println!(
        "Day  2 part 1 - end location is {:?}, multiplied = {}",
        end,
        end.0 .0 * end.1 .0
    );
}
