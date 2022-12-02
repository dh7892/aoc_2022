fn add_calories_for_elf(elf_carry: &str) -> u32 {
        elf_carry
        .lines()
        .map(|item| item.parse::<u32>().unwrap())
        .sum::<u32>()
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .map(add_calories_for_elf)
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = input
        .split("\n\n")
        .map(add_calories_for_elf)
        .collect::<Vec<_>>();
    result.sort_by(|a,b| b.cmp(a));
    let sum: u32 = result.iter().take(3).sum();
    Some(sum)
 }

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(53636));
    }

    #[test]
    fn test_dave_one() {
        let input: String = "7505
8207
3934
8305
10764
11568
3353".to_owned();
        assert_eq!(part_one(&input), Some(53636));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
