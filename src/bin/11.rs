use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{newline, u32 as nom32, u64 as nom64};
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    power: u32,
    multiply: u32,
    add: u32,
    divisible: u32,
    positive_target: usize,
    negative_target: usize,
    inspected_count: u32,
}

impl Monkey {
    fn inspect(self: &mut Self, magic: u32) -> Vec<(usize, u64)> {
        self.items
            .drain(..)
            .map(|item| {
                // Monkey inspects
                let mut new_item = item;
                self.inspected_count += 1;
                new_item = new_item.pow(self.power);
                new_item *= self.multiply as u64;
                new_item += self.add as u64;

                // Use Chinese remainder theorem
                if magic == 0 {
                    // Not valid denom to do drop
                    new_item /= 3;
                } else {
                    new_item = new_item % magic as u64;
                }

                // Find target
                let mut destination = self.negative_target;
                if new_item % self.divisible as u64 == 0 {
                    destination = self.positive_target
                }
                (destination, new_item)
            })
            .collect()
    }
}

fn power(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let (input, _) = tag("* old")(input)?;
    Ok((input, (2, 1, 0)))
}
fn multiply(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let (input, value) = preceded(tag("* "), nom32)(input)?;
    Ok((input, (1, value, 0)))
}
fn add(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let (input, add) = preceded(tag("+ "), nom32)(input)?;
    Ok((input, (1, 1, add)))
}
fn parse_op(input: &str) -> IResult<&str, (u32, u32, u32)> {
    let (input, (power, multiply, add)) = terminated(alt((power, multiply, add)), newline)(input)?;
    Ok((input, (power, multiply, add)))
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _monkey_num) = delimited(tag("Monkey "), nom32, tag(":"))(input)?;
    let (input, _) = newline(input)?;
    let (input, _starting_text) = tag("  Starting items: ")(input)?;
    let (input, items) = terminated(separated_list0(tag(", "), nom64), newline)(input)?;
    let (input, _operation_text) = tag("  Operation: new = old ")(input)?;
    let (input, (power, multiply, add)) = parse_op(input)?;
    let (input, _test) = tag("  Test: divisible by ")(input)?;
    let (input, divisible) = nom32(input)?;
    let (input, _) = newline(input)?;
    let (input, _true) = tag("    If true: throw to monkey ")(input)?;
    let (input, positive_target) = nom32(input)?;
    let (input, _) = newline(input)?;
    let (input, _true) = tag("    If false: throw to monkey ")(input)?;
    let (input, negative_target) = nom32(input)?;
    let (input, _) = newline(input)?;

    let monkey = Monkey {
        items,
        power,
        multiply,
        add,
        divisible,
        positive_target: positive_target as usize,
        negative_target: negative_target as usize,
        inspected_count: 0,
    };
    Ok((input, monkey))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list0(newline, parse_monkey)(input)
}

fn process(input: &str, use_magic: bool, rounds: u32) -> u64 {
    let (_, mut monkeys) = parse_monkeys(input).unwrap();
    let mut magic = monkeys.iter().map(|m| m.divisible).product();
    if !use_magic {
        magic = 0
    }
    for _ in 0..rounds {
        for mky_idx in 0..monkeys.len() {
            let items = monkeys[mky_idx].inspect(magic);
            for item in items {
                monkeys[item.0].items.push(item.1);
            }
        }
    }
    let total = monkeys
        .iter()
        .map(|m| m.inspected_count as u64)
        .sorted()
        .rev()
        .take(2)
        .product::<u64>();
    total
}

pub fn part_one(input: &str) -> Option<u32> {
    let product = process(input, false, 20);
    dbg!(product);
    Some(product as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let product = process(input, true, 10_000);
    dbg!(product);
    Some(product as u32)
}

fn main() {
    let input = &aoc::read_file("inputs", 11);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
