use std::collections::HashMap;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_till;
use nom::character::complete::{newline, u32 as nom32};
use nom::multi::separated_list0;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::IResult;

use indicatif::ProgressBar;

#[derive(Debug, Clone)]
struct DaveNum {
    remainders: HashMap<u32, u32>,
}

impl DaveNum {
    fn new(value: u32) -> Self {
        let mut num = DaveNum {
            remainders: HashMap::<u32, u32>::new(),
        };
        num.init();
        num.assign(value);
        num
    }
    fn init(self: &mut Self) {
        for modulus in [2, 3, 5, 7, 11, 13, 17, 19, 23] {
            self.remainders.insert(modulus, 0);
        }
    }
    fn is_divisible_by(self: &Self, base: u32) -> bool {
        self.remainders[&base] == 0
    }
    fn assign(self: &mut Self, value: u32) {
        for modulus in self.remainders.clone().keys() {
            self.remainders.insert(*modulus, value % modulus);
        }
    }
    fn add(self: &mut Self, value: u32) {
        for modulus in self.remainders.clone().keys() {
            self.remainders
                .insert(*modulus, (self.remainders[modulus] + value) % modulus);
        }
    }
    fn mult(self: &mut Self, value: u32) {
        for modulus in self.remainders.clone().keys() {
            self.remainders.insert(
                *modulus,
                (self.remainders[modulus] * value % modulus) % modulus,
            );
        }
    }
    fn pow(self: &mut Self, value: u32) {
        match value {
            2 => {
                for modulus in self.remainders.clone().keys() {
                    self.remainders.insert(
                        *modulus,
                        (self.remainders[modulus] * self.remainders[modulus]) % modulus,
                    );
                }
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<DaveNum>,
    power: u32,
    multiply: u32,
    add: u32,
    divisible: u32,
    positive_target: usize,
    negative_target: usize,
    inspected_count: u32,
}

impl Monkey {
    fn inspect_1(self: &mut Self) -> Vec<(usize, DaveNum)> {
        self.items
            .drain(..)
            .map(|item| {
                // Monkey inspects
                let mut new_item = item.clone();
                self.inspected_count += 1;
                new_item.pow(self.power);
                new_item.mult(self.multiply);
                new_item.add(self.add);

                // Natural drop
                //new_item /= 3;
                let mut destination = self.negative_target;
                if new_item.is_divisible_by(self.divisible) {
                    destination = self.positive_target
                }
                let moo = new_item.clone();
                (destination, moo)
            })
            .collect()
    }
    fn inspect_2(self: &mut Self) -> Vec<(usize, DaveNum)> {
        self.items
            .drain(..)
            .map(|item| {
                // Monkey inspects
                let mut new_item = item.clone();
                self.inspected_count += 1;
                new_item.pow(self.power);
                new_item.mult(self.multiply);
                new_item.add(self.add);

                let mut destination = self.negative_target;
                if new_item.is_divisible_by(self.divisible) {
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
    let (input, mult) = preceded(tag("* "), nom32)(input)?;
    Ok((input, (1, mult, 0)))
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
    let (input, _first_line) = take_till(|c| c == ':')(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = newline(input)?;
    let (input, _starting_text) = tag("  Starting items: ")(input)?;
    let (input, items) = terminated(separated_list0(tag(", "), nom32), newline)(input)?;
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
    let items_as_ints = items.into_iter().map(|i| DaveNum::new(i)).collect();

    let monkey = Monkey {
        items: items_as_ints,
        power: power,
        multiply: multiply,
        add: add,
        divisible: divisible,
        positive_target: positive_target as usize,
        negative_target: negative_target as usize,
        inspected_count: 0,
    };
    Ok((input, monkey))
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list0(newline, parse_monkey)(input)
}

fn print_monkeys(monkeys: &Vec<Monkey>) {
    for m in monkeys {
        dbg!(&m.items);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, mut monkeys) = parse_monkeys(input).unwrap();
    let num_rounds = 20;
    for _ in 0..num_rounds {
        for mky_idx in 0..monkeys.len() {
            let items = monkeys[mky_idx].inspect_1();
            for item in items {
                monkeys[item.0].items.push(item.1);
            }
        }
    }
    let total = monkeys
        .iter()
        .map(|m| m.inspected_count as u32)
        .sorted()
        .rev()
        .take(2)
        .reduce(|acc, e| acc * e)
        .unwrap();
    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, mut monkeys) = parse_monkeys(input).unwrap();
    let num_rounds = 10000;
    let p = ProgressBar::new(num_rounds);
    for _ in 0..num_rounds {
        p.inc(1);
        for mky_idx in 0..monkeys.len() {
            let items = monkeys[mky_idx].inspect_2();
            for item in items {
                monkeys[item.0].items.push(item.1);
            }
        }
    }
    dbg!(&monkeys);
    let total = monkeys
        .iter()
        .map(|m| m.inspected_count as u128)
        .sorted()
        .rev()
        .take(2)
        .reduce(|acc, e| acc * e)
        .unwrap();
    dbg!(&total);
    Some(total as u32)
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
    #[test]
    fn test_dave_num() {
        let mut full_num = 10;
        let mut num = DaveNum::new(full_num);
        let bases = [2, 3, 5, 7, 11, 13, 17, 19];
        for base in bases {
            assert_eq!(num.remainders[&base], full_num % base);
        }

        full_num += 1;
        num.add(1);
        for base in bases {
            assert_eq!(num.remainders[&base], full_num % base);
        }

        full_num *= 3;
        num.mult(3);
        for base in bases {
            assert_eq!(num.remainders[&base], full_num % base);
        }

        full_num *= full_num;
        num.pow(2);
        for base in bases {
            assert_eq!(num.remainders[&base], full_num % base);
        }
    }
}
