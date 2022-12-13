#![feature(iter_intersperse)]
use std::{cmp, fmt::Display};

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{newline, u32 as nom32};
use nom::multi::separated_list0;
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;

#[derive(Debug, Clone, Eq)]
enum Element {
    List(Vec<Element>),
    Value(u32),
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        use Element::*;
        match (self, other) {
            (List(l), List(r)) => l.cmp(r),
            (List(l), Value(r)) => l.cmp(&vec![Value(*r)]),
            (Value(l), List(r)) => vec![Value(*l)].cmp(r),
            (Value(l), Value(r)) => l.cmp(r),
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        use Element::*;
        match (self, other) {
            (List(l), List(r)) => l == r,
            (List(l), Value(r)) => l == &vec![Value(*r)],
            (Value(l), List(r)) => r == &vec![Value(*l)],
            (Value(l), Value(r)) => l == r,
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Element::List(list) => format!(
                    "[{}]",
                    list.iter()
                        .map(|v| v.to_string())
                        .intersperse(",".to_string())
                        .collect::<String>()
                ),
                Element::Value(num) => num.to_string(),
            }
        )
    }
}

fn element_value(input: &str) -> IResult<&str, Element> {
    let (input, value) = nom32(input)?;
    Ok((input, Element::Value(value)))
}

fn sub_list(input: &str) -> IResult<&str, Element> {
    delimited(tag("["), parse_element, tag("]"))(input)
}

fn parse_element(input: &str) -> IResult<&str, Element> {
    let (input, data) = separated_list0(tag(","), alt((element_value, sub_list)))(input)?;
    Ok((input, Element::List(data)))
}

fn parse_pair(input: &str) -> IResult<&str, (Element, Element)> {
    let (input, (left, _, right)) = tuple((sub_list, newline, sub_list))(input)?;
    Ok((input, (left, right)))
}

fn parse_all_input(input: &str) -> IResult<&str, Vec<(Element, Element)>> {
    separated_list0(separated_pair(newline, tag(""), newline), parse_pair)(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    use std::cmp::Ordering::*;
    let (_, elements) = parse_all_input(input).unwrap();
    let result = elements
        .into_iter()
        .enumerate()
        .filter_map(|(idx, pair)| match pair.0.cmp(&pair.1) {
            Less => Some(idx as u32 + 1),
            _ => None,
        })
        .sum::<u32>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    use Element::*;
    let mut elements = input
        .lines()
        .filter_map(|line| match sub_list(line) {
            Ok((_, e)) => Some(e),
            Err(_) => None,
        })
        .collect::<Vec<Element>>();
    let divider_1 = List(vec![Value(2)]);
    let divider_2 = List(vec![Value(6)]);
    elements.push(divider_1.clone());
    elements.push(divider_2.clone());

    // let sorted_elements = sort_elements(elements);
    elements.sort();

    let idx_1 = elements.iter().position(|e| e == &divider_1).unwrap() as u32;
    let idx_2 = elements.iter().position(|e| e == &divider_2).unwrap() as u32;

    Some((idx_1 + 1) * (idx_2 + 1))
}

fn main() {
    let input = &aoc::read_file("inputs", 13);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }

    #[test]
    fn test_parse_input() {
        let input = aoc::read_file("examples", 13);
        let (_, result) = parse_all_input(&input).unwrap();
        assert_eq!(result.len(), 8);
    }
}
