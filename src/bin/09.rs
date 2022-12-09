use std::collections::HashSet;
use std::ops::{Add, Sub};

use nom::character::complete::{space0, u32 as nom32};
use nom::sequence::separated_pair;
use nom::{character::complete::alpha1, IResult};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Posn {
    x: i32,
    y: i32,
}

impl Default for Posn {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Posn {
    fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
    fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}
impl Add for Posn {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Posn {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
fn parse_direction(input: &str) -> IResult<&str, Direction> {
    let (input, word) = alpha1(input)?;
    match word {
        "U" => Ok((input, Direction::Up)),
        "D" => Ok((input, Direction::Down)),
        "L" => Ok((input, Direction::Left)),
        "R" => Ok((input, Direction::Right)),
        _ => Ok((input, Direction::Right)),
    }
}

fn parse_move(input: &str) -> IResult<&str, (Direction, u32)> {
    separated_pair(parse_direction, space0, nom32)(input)
}

// Given head and tail positions, return new tail position
fn move_tail(head: Posn, tail: Posn) -> Posn {
    let diff = tail - head;
    let unit = diff.signum();
    let abs = diff.abs();

    let mut new_pos = head + unit;
    if abs.x > abs.y {
        // x direction dominates, y will end up level with head
        new_pos.y = head.y;
    }
    if abs.x < abs.y {
        // y direction dominates so x will end up level with head
        new_pos.x = head.x;
    }
    new_pos
}

// Apply all moves for a line and update head and tail. Track all tail locations visited
fn execute_line(line: &str, rope: &mut Vec<Posn>, visited: &mut HashSet<Posn>) {
    let (_, (direction, count)) = parse_move(line).unwrap();
    let delta = match direction {
        Direction::Up => Posn { x: 0, y: 1 },
        Direction::Down => Posn { x: 0, y: -1 },
        Direction::Left => Posn { x: -1, y: 0 },
        Direction::Right => Posn { x: 1, y: 0 },
    };
    for _ in 0..count {
        // Move the very head of the rope first
        rope[0] = rope[0] + delta;
        for idx in 1..rope.len() {
            let lead = rope[idx - 1];
            let follow = rope[idx];
            rope[idx] = move_tail(lead, follow);
        }
        visited.insert(rope.last().unwrap().clone());
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let start_pos = Posn::default();
    let mut locations = HashSet::<Posn>::new();
    let mut rope: Vec<Posn> = vec![start_pos; 2];
    locations.insert(rope.last().unwrap().clone());

    for line in input.lines() {
        execute_line(line, &mut rope, &mut locations);
    }

    Some(locations.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let start_pos = Posn::default();
    let mut locations = HashSet::<Posn>::new();
    let mut rope: Vec<Posn> = vec![start_pos; 10];
    locations.insert(rope.last().unwrap().clone());

    for line in input.lines() {
        execute_line(line, &mut rope, &mut locations);
    }

    Some(locations.len() as u32)
}

fn main() {
    let input = &aoc::read_file("inputs", 9);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }

    #[test]
    fn test_move_tail() {
        let head = Posn { x: 0, y: 0 };
        assert_eq!(
            move_tail(head, Posn { x: -1, y: -1 }),
            Posn { x: -1, y: -1 }
        );
        let head = Posn { x: 0, y: 0 };
        let tail = Posn { x: -2, y: -1 };
        let expected = Posn { x: -1, y: 0 };
        assert_eq!(move_tail(head, tail), expected);

        let head = Posn { x: 0, y: 0 };
        let tail = Posn { x: 2, y: 2 };
        let expected = Posn { x: 1, y: 1 };
        assert_eq!(move_tail(head, tail), expected);

        let head = Posn { x: 0, y: 0 };
        let tail = Posn { x: 2, y: -2 };
        let expected = Posn { x: 1, y: -1 };
        assert_eq!(move_tail(head, tail), expected);

        let head = Posn { x: 4, y: 2 };
        let tail = Posn { x: 3, y: 0 };
        let expected = Posn { x: 4, y: 1 };
        assert_eq!(move_tail(head, tail), expected);

        let head = Posn { x: 5, y: 3 };
        let tail = Posn { x: 3, y: 4 };
        let expected = Posn { x: 4, y: 3 };
        assert_eq!(move_tail(head, tail), expected);

        let head = Posn { x: 5, y: 3 };
        let tail = Posn { x: 4, y: 5 };
        let expected = Posn { x: 5, y: 4 };
        assert_eq!(move_tail(head, tail), expected);

        let head = Posn { x: 5, y: 3 };
        let tail = Posn { x: 6, y: 5 };
        let expected = Posn { x: 5, y: 4 };
        assert_eq!(move_tail(head, tail), expected);

        let head = Posn { x: 5, y: 3 };
        let tail = Posn { x: 7, y: 2 };
        let expected = Posn { x: 6, y: 3 };
        assert_eq!(move_tail(head, tail), expected);

        let head = Posn { x: 5, y: 3 };
        let tail = Posn { x: 2, y: 2 };
        let expected = Posn { x: 4, y: 3 };
        assert_eq!(move_tail(head, tail), expected);
    }
}
