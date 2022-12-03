use std::collections::HashSet;

fn char_to_num(c: &char) -> u32 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 26 + 1,
        'B' => 26 + 2,
        'C' => 26 + 3,
        'D' => 26 + 4,
        'E' => 26 + 5,
        'F' => 26 + 6,
        'G' => 26 + 7,
        'H' => 26 + 8,
        'I' => 26 + 9,
        'J' => 26 + 10,
        'K' => 26 + 11,
        'L' => 26 + 12,
        'M' => 26 + 13,
        'N' => 26 + 14,
        'O' => 26 + 15,
        'P' => 26 + 16,
        'Q' => 26 + 17,
        'R' => 26 + 18,
        'S' => 26 + 19,
        'T' => 26 + 20,
        'U' => 26 + 21,
        'V' => 26 + 22,
        'W' => 26 + 23,
        'X' => 26 + 24,
        'Y' => 26 + 25,
        'Z' => 26 + 26,
        _ => 0,
    }
}


fn line_to_score(line: &str) -> u32 {
    let all_items: Vec<char> = line.chars().collect();
    let mut comp_1 = HashSet::new();
    let mut comp_2 = HashSet::new();
    let comp_size = all_items.len()/2;
    for i in 0..comp_size {
        comp_1.insert(all_items[i]);
        comp_2.insert(all_items[i+ comp_size]);
    }
    let items = comp_1.intersection(&comp_2);
    let outlier= items.take(1).next().unwrap();
    char_to_num(outlier)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(line_to_score).sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines().collect::<Vec<&str>>();
    let mut score: u32 = 0;
    while lines.len() >= 3 {
        let chars: Vec<char> = lines.pop().unwrap().chars().collect();
        let line1: HashSet<char> = HashSet::from_iter(chars.iter().cloned());
        let chars: Vec<char> = lines.pop().unwrap().chars().collect();
        let line2: HashSet<char> = HashSet::from_iter(chars.iter().cloned());
        let chars: Vec<char> = lines.pop().unwrap().chars().collect();
        let line3: HashSet<char> = HashSet::from_iter(chars.iter().cloned());
        let common = line1.intersection(&line2);
        let next_set:HashSet<char> = HashSet::from_iter(common.into_iter().cloned());
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
        assert_eq!(char_to_num(&'a'), 1 );
        assert_eq!(char_to_num(&'z'), 26);
        assert_eq!(char_to_num(&'A'), 27 );
        assert_eq!(char_to_num(&'Z'), 52 );
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
