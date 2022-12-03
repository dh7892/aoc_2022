use std::str::FromStr;

enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}
impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Not a known move".to_owned())
        }
    }
}

enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl FromStr for Outcome {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err("Not a known outcome".to_owned())
        }
    }
}


fn line_to_score(input: &str) -> u32 {
    let mut split = input.split(" ");
    let their_move = split.next().unwrap().parse::<Move>().unwrap();
    let my_move =  split.next().unwrap().parse::<Move>().unwrap();
    let outcome = match (&their_move, &my_move) {
        (Move::Rock, Move::Rock) => Outcome::Draw,
        (Move::Rock, Move::Paper) => Outcome::Win,
        (Move::Rock, Move::Scissors) => Outcome::Lose,

        (Move::Paper, Move::Rock) => Outcome::Lose,
        (Move::Paper, Move::Paper) => Outcome::Draw,
        (Move::Paper, Move::Scissors) => Outcome::Win,

        (Move::Scissors, Move::Rock) => Outcome::Win,
        (Move::Scissors, Move::Paper) => Outcome::Lose,
        (Move::Scissors, Move::Scissors) => Outcome::Draw,
    };
    my_move as u32 + outcome as u32
}

fn work_out_score(input: &str) -> u32 {
    let mut split = input.split(" ");
    let their_move = split.next().unwrap().parse::<Move>().unwrap();
    let outcome =  split.next().unwrap().parse::<Outcome>().unwrap();
    let my_move = match (&their_move, &outcome) {
        (Move::Rock, Outcome::Draw) => Move::Rock,
        (Move::Rock, Outcome::Win) => Move::Paper,
        (Move::Rock, Outcome::Lose) => Move::Scissors,

        (Move::Paper, Outcome::Lose) => Move::Rock,
        (Move::Paper, Outcome::Draw) => Move::Paper,
        (Move::Paper, Outcome::Win) => Move::Scissors,

        (Move::Scissors,  Outcome::Win) => Move::Rock,
        (Move::Scissors,  Outcome::Lose) => Move::Paper,
        (Move::Scissors,  Outcome::Draw) => Move::Scissors,
    };
    my_move as u32 + outcome as u32
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
        assert_eq!(part_one(&input), Some(15));
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
