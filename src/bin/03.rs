use std::collections::HashMap;
use std::collections::HashSet;

fn char_to_num(c: &char) -> u32 {
    let chars = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();
    chars[c] as u32
}

pub fn part_one(input: &str) -> Option<u32> {
    let total = input
        .lines()
        .map(|line| {
            let size = line.len() / 2;
            let comp_1: HashSet<char> = HashSet::from_iter(line[0..size].chars());
            let comp_2: HashSet<char> = HashSet::from_iter(line[size..size * 2].chars());
            let common_char = comp_1.intersection(&comp_2).take(1).next().unwrap();
            char_to_num(common_char)
        })
        .sum::<u32>();
    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines().collect::<Vec<&str>>();
    let mut score: u32 = 0;
    while lines.len() >= 3 {
        let line1: HashSet<char> = HashSet::from_iter(lines.pop().unwrap().chars());
        let line2: HashSet<char> = HashSet::from_iter(lines.pop().unwrap().chars());
        let line3: HashSet<char> = HashSet::from_iter(lines.pop().unwrap().chars());
        let common = line1.intersection(&line2);
        let next_set: HashSet<char> = HashSet::from_iter(common.into_iter().cloned());
        let common = next_set.intersection(&line3);
        score = score + char_to_num(common.take(1).next().unwrap())
    }
    Some(score)
}

fn main() {
    let input = &aoc::read_file("inputs", 3);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(char_to_num(&'a'), 1);
        assert_eq!(char_to_num(&'z'), 26);
        assert_eq!(char_to_num(&'A'), 27);
        assert_eq!(char_to_num(&'Z'), 52);
    }

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
