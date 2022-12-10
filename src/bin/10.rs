use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{i32 as nom32, newline};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

use std::collections::VecDeque;

#[derive(Debug)]
enum OpType {
    Add,
    Processing,
    Noop,
}

#[derive(Debug)]
struct Op {
    op_type: OpType,
    arg: Option<i32>,
}

fn noop(input: &str) -> IResult<&str, Op> {
    let (input, _) = tag("noop")(input)?;
    Ok((
        input,
        Op {
            op_type: OpType::Noop,
            arg: None,
        },
    ))
}
fn addx(input: &str) -> IResult<&str, Op> {
    let (input, (_, arg)) = separated_pair(tag("addx"), tag(" "), nom32)(input)?;
    Ok((
        input,
        Op {
            op_type: OpType::Add,
            arg: Some(arg),
        },
    ))
}

fn parse_line(input: &str) -> IResult<&str, Op> {
    alt((addx, noop))(input)
}

fn parse_lines(input: &str) -> IResult<&str, VecDeque<Op>> {
    let (input, ops) = separated_list1(newline, parse_line)(input)?;
    let mut op_queue = VecDeque::<Op>::new();
    for op in ops {
        match op.op_type {
            OpType::Add => {
                op_queue.push_back(Op {
                    op_type: OpType::Processing,
                    arg: None,
                });
                op_queue.push_back(op);
            }
            _ => {
                op_queue.push_back(op);
            }
        }
    }
    Ok((input, op_queue))
}

struct CPU {
    cycle: usize,
    register: i32,
    current_op: Op,
    op_queue: VecDeque<Op>,
}

impl CPU {
    /// Advance once cycle
    fn tick(self: &mut Self) {
        self.cycle += 1;
        match self.current_op.op_type {
            OpType::Add => {
                self.register += self.current_op.arg.unwrap();
            }
            OpType::Noop | OpType::Processing => {}
        }
        self.current_op = self.op_queue.pop_front().unwrap();
    }
}

fn cycle_is_interesting(cycle: usize) -> bool {
    (cycle as i32 - 19) % 40 == 0
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, mut ops) = parse_lines(input).unwrap();
    let first_op = ops.pop_front().unwrap();
    let mut cpu = CPU {
        cycle: 0,
        current_op: first_op,
        register: 1,
        op_queue: ops,
    };
    let mut score = 0;
    while !cpu.op_queue.is_empty() {
        if cycle_is_interesting(cpu.cycle) {
            score += cpu.register * (cpu.cycle as i32 + 1);
        }
        cpu.tick();
    }
    Some(score as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 10);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_really_small() {
        let input = "noop
addx 3
addx -5";
        assert_eq!(part_one(&input), Some(0));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
