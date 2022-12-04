use std::ops::RangeInclusive;

use nom::bytes::complete::tag;
use nom::character::complete::{newline, u32 as nom32};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::{self, IResult};

fn assignment(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, (start, end)) = separated_pair(nom32, tag("-"), nom32)(input)?;
    Ok((input, start..=end))
}

fn assignments(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (input, (first, second)) = separated_pair(assignment, tag(","), assignment)(input)?;
    Ok((input, (first, second)))
}

fn all_assignments(input: &str) -> IResult<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    let (input, all_assignments) = separated_list1(newline, assignments)(input)?;
    Ok((input, all_assignments))
}

/// Return true if one range completely contains the other
fn range_contains(r1: &RangeInclusive<u32>, r2: &RangeInclusive<u32>) -> bool {
    (r1.start() <= r2.start() && r1.end() >= r2.end())
        || (r2.start() <= r1.start() && r2.end() >= r1.end())
}

/// Return true if one range overlaps the other
fn range_overlap(r1: &RangeInclusive<u32>, r2: &RangeInclusive<u32>) -> bool {
    !(r1.start() > r2.end() || r2.start() > r1.end())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, lines) = all_assignments(input).unwrap();
    let result = lines
        .into_iter()
        .filter(|(r1, r2)| range_contains(r1, r2))
        .count();
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, lines) = all_assignments(input).unwrap();
    let result = lines
        .into_iter()
        .filter(|(r1, r2)| range_overlap(r1, r2))
        .count();
    Some(result as u32)
}

fn main() {
    let input = &aoc::read_file("inputs", 4);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
