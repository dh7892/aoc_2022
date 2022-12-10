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
    arg: i32,
}

fn noop(input: &str) -> IResult<&str, Op> {
    let (input, _) = tag("noop")(input)?;
    Ok((
        input,
        Op {
            op_type: OpType::Noop,
            arg: 0,
        },
    ))
}
fn addx(input: &str) -> IResult<&str, Op> {
    let (input, (_, arg)) = separated_pair(tag("addx"), tag(" "), nom32)(input)?;
    Ok((
        input,
        Op {
            op_type: OpType::Add,
            arg: arg,
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
                    arg: 0,
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

fn register_for_cycles(input: &str) -> Vec<i32> {
    let (_, mut ops) = parse_lines(input).unwrap();
    let mut reg_over_time: Vec<i32> = Vec::new();
    let mut current_reg = 1;
    ops.push_front(Op {
        op_type: OpType::Processing,
        arg: 0,
    });
    reg_over_time.push(current_reg);
    for op in ops {
        current_reg += op.arg;
        reg_over_time.push(current_reg);
    }
    reg_over_time
}

pub fn part_one(input: &str) -> Option<u32> {
    let reg_over_time = register_for_cycles(input);
    let score = reg_over_time
        .iter()
        .enumerate()
        .skip(20)
        .step_by(40)
        .map(|(idx, value)| idx as i32 * value)
        .sum::<i32>();
    Some(score as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reg_over_time = register_for_cycles(input);
    let mut row = vec![" "; 40];
    for idx in 0..240 {
        let x_pos = idx % 40;
        if x_pos == 39 {
            // Print out and reset row
            println!("{}", row.clone().into_iter().collect::<String>());
            row.fill(" ");
        }
        let r = reg_over_time[idx + 1];
        if (r - x_pos as i32).abs() <= 1 {
            row[x_pos] = "#";
        }
    }
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
