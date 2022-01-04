#![allow(dead_code)]
use std::path::{Component, Path};

type Registers = [isize; 4];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operand {
    Reg(usize),
    Lit(isize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Inp(usize),
    Add(usize, Operand),
    Mul(usize, Operand),
    Div(usize, Operand),
    Mod(usize, Operand),
    Eql(usize, Operand),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Machine {
    registers: Registers,
    pc: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EvalResult {
    OK,
    AwaitingInput(usize),
    Done,
}
impl Machine {
    fn new() -> Self {
        Machine { registers: [0; 4], pc: 0 }
    }
    fn set_reg(&mut self, reg: usize, value: isize) {
        self.registers[reg] = value;
    }
    fn get_reg(&mut self, reg: usize) -> isize {
        self.registers[reg]
    }
    fn eval(&mut self, program: &[Instruction]) -> EvalResult {
        if self.pc >= program.len() {
            EvalResult::Done
        } else {
            let instr = program[self.pc];
            self.pc += 1;
            match instr {
                Instruction::Inp(reg) => EvalResult::AwaitingInput(reg),
                Instruction::Add(reg, operand) => {
                    self.registers[reg] += self.value(operand);
                    EvalResult::OK
                }
                Instruction::Mul(reg, operand) => {
                    self.registers[reg] *= self.value(operand);
                    EvalResult::OK
                }
                Instruction::Div(reg, operand) => {
                    self.registers[reg] /= self.value(operand);
                    EvalResult::OK
                }
                Instruction::Mod(reg, operand) => {
                    self.registers[reg] %= self.value(operand);
                    EvalResult::OK
                }
                Instruction::Eql(reg, operand) => {
                    self.registers[reg] = (self.registers[reg] == self.value(operand)) as isize;
                    EvalResult::OK
                }
            }
        }
    }
    fn eval_until(&mut self, program: &[Instruction]) -> EvalResult {
        loop {
            let r = self.eval(program);
            if r != EvalResult::OK {
                return r;
            }
        }
    }

    fn value(&self, op: Operand) -> isize {
        match op {
            Operand::Lit(i) => i,
            Operand::Reg(r) => self.registers[r],
        }
    }
}

fn parse_reg(i: &str) -> usize {
    match i.chars().next() {
        Some('w') => 0,
        Some('x') => 1,
        Some('y') => 2,
        Some('z') => 3,
        _ => panic!("Bad register character in \"{}\"", i),
    }
}
fn parse_num(i: &str) -> isize {
    i.parse::<isize>().unwrap()
}

fn parse_operand(i: &str) -> Operand {
    if ('w'..='z').contains(&i.chars().next().unwrap()) {
        Operand::Reg(parse_reg(i))
    } else {
        Operand::Lit(parse_num(i))
    }
}

fn parse_instr(i: &str) -> Instruction {
    let bits = i.split_ascii_whitespace().collect::<Vec<_>>();
    match (bits[0], bits.len()) {
        // inp a - Read an input value and write it to variable a.
        ("inp", 2) => Instruction::Inp(parse_reg(bits[1])),
        // add a b - Add the value of a to the value of b, then store the result in variable a.
        ("add", 3) => Instruction::Add(parse_reg(bits[1]), parse_operand(bits[2])),
        // mul a b - Multiply the value of a by the value of b, then store the result in variable a.
        ("mul", 3) => Instruction::Mul(parse_reg(bits[1]), parse_operand(bits[2])),
        // div a b - Divide the value of a by the value of b, truncate the result to an integer, then store the result in variable a. (Here, "truncate" means to round the value toward zero.)
        ("div", 3) => Instruction::Div(parse_reg(bits[1]), parse_operand(bits[2])),
        // mod a b - Divide the value of a by the value of b, then store the remainder in variable a. (This is also called the modulo operation.)
        ("mod", 3) => Instruction::Mod(parse_reg(bits[1]), parse_operand(bits[2])),
        // eql a b - If the value of a and b are equal, then store the value 1 in variable a. Otherwise, store the value 0 in variable a.
        ("eql", 3) => Instruction::Eql(parse_reg(bits[1]), parse_operand(bits[2])),
        _ => panic!("Bad input line in \"{}\"", i),
    }
}

fn parse(i: &str) -> Vec<Instruction> {
    i.lines().map(parse_instr).collect::<Vec<_>>()
}

fn hand_translated_fn(d: &[isize; 14]) -> bool {
    // inp w
    let w = d[0];
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 13
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 8
    // mul y x
    // add z y
    let y = w + 8;
    let z = y;

    // inp w
    let w = d[1];
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 12
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    let y = 26;
    // mul z y
    let z = z * y;
    // mul y 0
    // add y w
    // add y 16
    // mul y x
    // add z y
    let y = w + 16;
    let z = z + y;

    // inp w
    let w = d[2];
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 10
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    let z = z * 26;
    // mul y 0
    // add y w
    // add y 4
    // mul y x
    let y = w + 4;
    // add z y
    let z = z + y;

    // inp w
    let w = d[3];
    // mul x 0
    // add x z
    // mod x 26
    let x = z % 26;
    // div z 26
    let z = z / 26;
    // add x -11
    let x = x - 11;
    // eql x w
    // eql x 0
    let x = x != w;
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // let y = if x { 26 } else { 1 };
    // mul z y
    // let z = z * y;
    // mul y 0
    // add y w
    // add y 1
    // mul y x
    // let y = if x { w + 1 } else { 0 };
    // add z y
    let z = if x { (z * 26) + w + 1 } else { z };

    // inp w
    let w = d[4];
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 14
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 13
    // mul y x
    // add z y
    let z = (26 * z) + w + 13;

    // inp w
    let w = d[5];
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 13
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 5
    // mul y x
    // add z y
    let z = (z * 26) + w + 5;

    // inp w
    let w = d[6];
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 12
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 0
    // mul y x
    // add z y
    let z = (z * 26) + w;

    // inp w
    let w = d[7];
    // mul x 0
    // add x z
    // mod x 26
    let x = z % 26;
    // div z 26
    let z = z / 26;
    // add x -5
    let x = (x - 5) != w;
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // let y = if x { 26 } else { 1 };
    // mul z y
    // let z = z * y;
    // mul y 0
    // add y w
    // add y 10
    // mul y x
    // let y = if x { w + 10 } else { 0 };
    // add z y
    let z = if x { (z * 26) + w + 10 } else { z };

    // inp w
    let w = d[8];
    // mul x 0
    // add x z
    // mod x 26
    // div z 1
    // add x 10
    // eql x w
    // eql x 0
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // mul z y
    // mul y 0
    // add y w
    // add y 7
    // mul y x
    // add z y
    let z = (z * 26) + w + 7;

    // inp w
    let w = d[9];
    // mul x 0
    // add x z
    // mod x 26
    let x = z % 26;
    // div z 26
    let z = z / 26;
    // add x 0
    // eql x w
    // eql x 0
    let x = x != w;
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // let y = if x { 26 } else { 1 };
    // mul z y
    // let z = z * y;
    // mul y 0
    // add y w
    // add y 2
    // mul y x
    // let y = if x { w + 2 } else { 0 };
    // add z y
    let z = if x { z * 26 + (w + 2) } else { z };

    // inp w
    let w = d[10];
    // mul x 0
    // add x z
    // mod x 26
    let x = z % 26;
    // div z 26
    let z = z / 26;
    // add x -11
    // eql x w
    // eql x 0
    let x = x - 11 != w;
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // let y = if x { 26 } else { 1 };
    // mul z y
    // let z = z * y;
    // mul y 0
    // add y w
    // add y 13
    // mul y x
    // let y = if x { w + 13 } else { 0 };
    // add z y
    let z = if x { (z * 26) + w + 13 } else { z };

    // inp w
    let w = d[11];
    // mul x 0
    // add x z
    // mod x 26
    let x = z % 26;
    // div z 26
    let z = z / 26;
    // add x -13
    // eql x w
    // eql x 0
    let x = (x - 13) != w;
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // let y = if x { 26 } else { 1 };
    // mul z y
    // let z = z * y;
    // mul y 0
    // add y w
    // add y 15
    // mul y x
    // let y = if x { w + 15 } else { 0 };
    // add z y
    let z = if x { (z * 26) + w + 15 } else { z };

    // inp w
    let w = d[12];
    // mul x 0
    // add x z
    // mod x 26
    let x = z % 26;
    // div z 26
    let z = z / 26;
    // add x -13
    // eql x w
    // eql x 0
    let x = (x - 13) != w;
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // let y = if x { 26 } else { 1 };
    // mul z y
    // let z = z * y;
    // mul y 0
    // add y w
    // add y 14
    // mul y x
    // let y = if x { w + 14 } else { 0 };
    // add z y
    let z = if x { (z * 26) + w + 14 } else { z };

    // inp w
    let w = d[13];
    // mul x 0
    // add x z
    // mod x 26
    let x = z % 26;
    // div z 26
    let z = z / 26;
    // add x -11
    // eql x w
    // eql x 0
    let x = (x - 11) != w;
    // mul y 0
    // add y 25
    // mul y x
    // add y 1
    // let y = if x { 26 } else { 1 };
    // mul z y
    // let z = z * y;
    // mul y 0
    // add y w
    // add y 9
    // mul y x
    // let y = if x { w + 9 } else { 0 };
    // add z y
    let z = if x { (z * 26) + w + 9 } else { z };

    z == 0
}

fn iterate_input(machine: &mut Machine, program: &[Instruction], version: usize) -> Option<usize> {
    if version % 100_000_000 == 11_111_111 {
        dbg!(version);
    }
    if let EvalResult::AwaitingInput(reg) = machine.eval_until(program) {
        (1..=9usize).rev().find_map(|i| {
            let mut new_m = machine.clone();
            new_m.set_reg(reg, i as isize);
            iterate_input(&mut new_m, program, (version * 10) + i)
        })
    } else {
        // Must be EvalResult::Done - check if z (reg 3) is 0. If so, return the version
        if machine.get_reg(3) == 0 {
            println!("Valid version {}", version);
            Some(version)
        } else {
            None
        }
    }
}

mod part1 {
    use super::*;

    pub fn run(_input: &str) -> isize {
        let mut d = [0; 14];
        for d0 in (1..=9).rev() {
            d[0] = d0;
            for d1 in (1..=9).rev() {
                d[1] = d1;
                for d2 in (1..=9).rev() {
                    d[2] = d2;
                    for d3 in (1..=9).rev() {
                        d[3] = d3;
                        for d4 in (1..=9).rev() {
                            d[4] = d4;
                            for d5 in (1..=9).rev() {
                                d[5] = d5;
                                println!("{:?}", &d);
                                for d6 in (1..=9).rev() {
                                    d[6] = d6;
                                    for d7 in (1..=9).rev() {
                                        d[7] = d7;
                                        for d8 in (1..=9).rev() {
                                            d[8] = d8;
                                            for d9 in (1..=9).rev() {
                                                d[9] = d9;
                                                for d10 in (1..=9).rev() {
                                                    d[10] = d10;
                                                    for d11 in (1..=9).rev() {
                                                        d[11] = d11;
                                                        for d12 in (1..=9).rev() {
                                                            d[12] = d12;
                                                            for d13 in (1..=9).rev() {
                                                                d[13] = d13;
                                                                if hand_translated_fn(&d) {
                                                                    return d.iter().fold(1, |acc, d| (acc * 10) + d);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        0
    }

    #[test]
    fn test_run() {
        // let input_string = include_str!("../test.txt");
        // assert_eq!(1, run(input_string))
    }

    #[test]
    fn test_one() {
        let input_string = "inp x
                                 mul x -1";
        let program = parse(input_string);
        let mut machine = Machine::new();
        assert_eq!(machine.eval_until(&program), EvalResult::AwaitingInput(1));
        machine.set_reg(1, 13);
        assert_eq!(machine.eval_until(&program), EvalResult::Done);
        assert_eq!(machine.get_reg(1), -13);
    }

    #[test]
    fn test_two() {
        let input_string = "inp z
                                 inp x
                                 mul z 3
                                 eql z x";
        let program = parse(input_string);
        let mut machine = Machine::new();
        assert_eq!(machine.eval_until(&program), EvalResult::AwaitingInput(3));
        machine.set_reg(3, 13);
        assert_eq!(machine.eval_until(&program), EvalResult::AwaitingInput(1));
        machine.set_reg(1, 39);
        assert_eq!(machine.eval_until(&program), EvalResult::Done);
        assert_eq!(machine.get_reg(3), 1);
        let mut machine = Machine::new();
        assert_eq!(machine.eval_until(&program), EvalResult::AwaitingInput(3));
        machine.set_reg(3, 13);
        assert_eq!(machine.eval_until(&program), EvalResult::AwaitingInput(1));
        machine.set_reg(1, 42);
        assert_eq!(machine.eval_until(&program), EvalResult::Done);
        assert_eq!(machine.get_reg(3), 0);
    }
    #[test]
    fn test_three() {
        let input_string = "inp w
                                 add z w
                                 mod z 2
                                 div w 2
                                 add y w
                                 mod y 2
                                 div w 2
                                 add x w
                                 mod x 2
                                 div w 2
                                 mod w 2";
        let program = parse(input_string);
        let mut machine = Machine::new();
        assert_eq!(machine.eval_until(&program), EvalResult::AwaitingInput(0));
        machine.set_reg(0, 13);
        assert_eq!(machine.eval_until(&program), EvalResult::Done);
        assert_eq!(machine.get_reg(0), 1);
        assert_eq!(machine.get_reg(1), 1);
        assert_eq!(machine.get_reg(2), 0);
        assert_eq!(machine.get_reg(3), 1);
    }

    #[test]
    fn test_hand() {
        assert_eq!(hand_translated_fn(&[9, 6, 9, 2, 9, 9, 9, 4, 2, 9, 3, 9, 9, 6,]), true);
        assert_eq!(hand_translated_fn(&[4, 1, 8, 1, 1, 7, 6, 1, 1, 8, 1, 1, 4, 1,]), true);
    }
}

mod part2 {
    use super::*;

    pub fn run(_input: &str) -> isize {
        let mut d = [0; 14];
        for d0 in 1..=9 {
            d[0] = d0;
            for d1 in 1..=9 {
                d[1] = d1;
                for d2 in 1..=9 {
                    d[2] = d2;
                    for d3 in 1..=9 {
                        d[3] = d3;
                        for d4 in 1..=9 {
                            d[4] = d4;
                            for d5 in 1..=9 {
                                d[5] = d5;
                                println!("{:?}", &d);
                                for d6 in 1..=9 {
                                    d[6] = d6;
                                    for d7 in 1..=9 {
                                        d[7] = d7;
                                        for d8 in 1..=9 {
                                            d[8] = d8;
                                            for d9 in 1..=9 {
                                                d[9] = d9;
                                                for d10 in 1..=9 {
                                                    d[10] = d10;
                                                    for d11 in 1..=9 {
                                                        d[11] = d11;
                                                        for d12 in 1..=9 {
                                                            d[12] = d12;
                                                            for d13 in 1..=9 {
                                                                d[13] = d13;
                                                                if hand_translated_fn(&d) {
                                                                    return d.iter().fold(1, |acc, d| (acc * 10) + d);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        0
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
    assert_eq!(part1_ans, 96929994293996);

    let now = std::time::Instant::now();
    let part2_ans = part2::run(input_string);
    println!("Day {} part 2 - {} - took {} milliseconds.", day_number, part2_ans, now.elapsed().as_millis());
    assert_eq!(part2_ans, 41811761181141);
}
