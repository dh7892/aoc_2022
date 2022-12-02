fn line_to_score(input: &str) -> u32 {
    match input {
        "A X" => 1+3,
        "A Y" => 2+6,
        "A Z" => 3+0,
        "B X" => 1+0,
        "B Y" => 2+3,
        "B Z" => 3+6,
        "C X" => 1+6,
        "C Y" => 2+0,
        "C Z" => 3+3,
        _ => 0
    }
}

fn work_out_score(input: &str) -> u32 {
    match input {
        "A X" => 3+0,
        "A Y" => 1+3,
        "A Z" => 2+6,
        "B X" => 1+0,
        "B Y" => 2+3,
        "B Z" => 3+6,
        "C X" => 2+0,
        "C Y" => 3+3,
        "C Z" => 1+6,
        _ => 0
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
      .lines()
      .map(line_to_score)
      .sum::<u32>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
      .lines()
      .map(work_out_score)
      .sum::<u32>();
    Some(result)
}

fn main() {
    let input = &aoc::read_file("inputs", 2);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(100));
    }
    #[test]
    fn test_work_out_score() {
        assert_eq!(work_out_score(&"A X".to_owned()), 3);
        assert_eq!(work_out_score(&"B X".to_owned()), 1);
        assert_eq!(work_out_score(&"C X".to_owned()), 2);

        assert_eq!(work_out_score(&"A Y".to_owned()), 1 + 3);
        assert_eq!(work_out_score(&"B Y".to_owned()), 2 + 3);
        assert_eq!(work_out_score(&"C Y".to_owned()), 3 + 3);

        assert_eq!(work_out_score(&"A Z".to_owned()), 2 + 6);
        assert_eq!(work_out_score(&"B Z".to_owned()), 3 + 6);
        assert_eq!(work_out_score(&"C Z".to_owned()), 1 + 6);
    }
}
