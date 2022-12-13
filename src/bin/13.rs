use std::cmp;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{newline, u32 as nom32};
use nom::multi::separated_list0;
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Element {
    List(Vec<Element>),
    Value(u32),
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

#[derive(Debug, PartialEq, Eq)]
enum Status {
    Correct,
    Incorrect,
    Continue,
}

fn go_compare(left: &Element, right: &Element) -> Status {
    use std::cmp::Ordering::*;
    use Element::*;
    use Status::*;
    match (left, right) {
        (List(left_l), List(right_l)) => {
            let common_len = cmp::min(left_l.len(), right_l.len());
            for idx in 0..common_len {
                let result = go_compare(&left_l[idx].clone(), &right_l[idx].clone());
                match result {
                    Correct => return Correct,
                    Incorrect => return Incorrect,
                    Continue => {}
                }
            }
            // If we get here, the lists we didn't find a definitive result yet.
            // If the lists are not the same length, we can work out the correctness
            match left_l.len().partial_cmp(&right_l.len()).unwrap() {
                Less => return Correct,
                Equal => return Continue,
                Greater => return Incorrect,
            }
        }
        (List(l), Value(r)) => go_compare(&List(l.clone()), &List(vec![Value(r.clone())])),
        (Value(l), List(r)) => go_compare(&List(vec![Value(l.clone())]), &List(r.clone())),
        (Value(left_v), Value(right_v)) => match left_v.partial_cmp(right_v).unwrap() {
            Less => Correct,
            Equal => Continue,
            Greater => Incorrect,
        },
    }
}
/// Note this sort algorithm requires that subsequent elements in the input
/// will have a place to go in the sorted list. E.g. would work for sorting this
/// list of numbers:
/// 6,5,4 because 5 needs to go before 6, which will already have been put in the sorted list
/// It would also work for 5, 6, 4 by chance because, when we try to add 6, it has nowhere to go and
/// will get put on the end (which happens to be correct too)
///
/// But for 6, 4, 5 we will end up with 5, 6, 4 as our sorted list because trying to add 4 will put
/// it on the end due to 5 not yet being in the list.
///
/// If we wanted to generalise this properly, we'd need not put on the end if we can't find an index
/// and keep re-processing until all elements are found a home
/// But it seems the input for this problem is in the right order for us to make this work so we don't
/// have to fix this
fn sort_elements(elements: Vec<Element>) -> Vec<Element> {
    let mut result = Vec::new();
    for el in elements {
        match result
            .iter()
            .find_position(|e| go_compare(&el, &e) == Status::Correct)
        {
            Some((idx, _)) => result.insert(idx, el),
            None => result.push(el.clone()),
        };
    }
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, elements) = parse_all_input(input).unwrap();
    let result = elements
        .into_iter()
        .enumerate()
        .map(|(idx, pair)| (idx + 1, go_compare(&pair.0, &pair.1)))
        .filter(|(_, result)| result == &Status::Correct)
        .map(|(idx, _)| idx as u32)
        .sum::<u32>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elements = input
        .lines()
        .filter(|line| line != &"")
        .map(|line| {
            let (_, e) = parse_element(line).unwrap();
            e
        })
        .collect::<Vec<Element>>();
    let divider_1 = Element::List(vec![Element::Value(2)]);
    let divider_2 = Element::List(vec![Element::Value(6)]);
    elements.push(divider_1.clone());
    elements.push(divider_2.clone());

    let sorted_elements = sort_elements(elements);
    let idx_1 = sorted_elements
        .iter()
        .position(|e| e == &divider_1)
        .unwrap() as u32;
    let idx_2 = sorted_elements
        .iter()
        .position(|e| e == &divider_2)
        .unwrap() as u32;

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
