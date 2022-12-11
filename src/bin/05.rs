pub fn part_one(input: &str) -> Option<String> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut sections = input.split("\n\n");
    let mut crates: Vec<&str> = sections.next().unwrap().lines().collect();
    let instructions: Vec<&str> = sections.next().unwrap().lines().collect();

    for _ in 0..9 {
        stacks.push(Vec::new());
    }

    // Take off the line of stack numbers as we don't need it
    crates.pop();

    for line in crates {
        let chars: Vec<char> = line.chars().collect();
        let crates_on_line = (chars.len() - 3) / 4 + 1;
        for idx in 0..crates_on_line {
            let c = chars[1 + idx * 4];
            if c != ' ' {
                stacks[idx].insert(0, c);
            }
        }
    }

    // Now we can read and process the moves
    for instruction in instructions {
        let words = instruction.split(" ").collect::<Vec<&str>>();
        let count = words[1].parse::<usize>().unwrap();
        let source = words[3].parse::<usize>().unwrap();
        let dest = words[5].parse::<usize>().unwrap();

        // Process instruction
        for _ in 0..count {
            let c = stacks[source - 1].pop().unwrap();
            stacks[dest - 1].push(c);
        }
    }
    let mut result: Vec<char> = Vec::new();
    for mut s in stacks {
        let c = s.pop().unwrap_or(' ');
        if c != ' ' {
            result.push(c);
        }
    }

    Some(result.into_iter().collect())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut sections = input.split("\n\n");
    let mut crates: Vec<&str> = sections.next().unwrap().lines().collect();
    let instructions: Vec<&str> = sections.next().unwrap().lines().collect();

    for _ in 0..9 {
        stacks.push(Vec::new());
    }

    // Take off the line of stack numbers as we don't need it
    crates.pop();

    for line in crates {
        let chars: Vec<char> = line.chars().collect();
        let crates_on_line = (chars.len() - 3) / 4 + 1;
        for idx in 0..crates_on_line {
            let c = chars[1 + idx * 4];
            if c != ' ' {
                stacks[idx].insert(0, c);
            }
        }
    }

    // Now we can read and process the moves
    for instruction in instructions {
        let words = instruction.split(" ").collect::<Vec<&str>>();
        let count = words[1].parse::<usize>().unwrap();
        let source = words[3].parse::<usize>().unwrap();
        let dest = words[5].parse::<usize>().unwrap();

        // Process instruction
        let mut tmp: Vec<char> = Vec::new();
        for _ in 0..count {
            let c = stacks[source - 1].pop().unwrap();
            tmp.push(c);
        }
        for _ in 0..count {
            stacks[dest - 1].push(tmp.pop().unwrap());
        }
    }
    let mut result: Vec<char> = Vec::new();
    for mut s in stacks {
        let c = s.pop().unwrap_or(' ');
        if c != ' ' {
            result.push(c);
        }
    }

    Some(result.into_iter().collect())
}

fn main() {
    let input = &aoc::read_file("inputs", 5);
    println!("Part A solution: {}", part_one(input).unwrap());
    println!("Part B solution: {}", part_two(input).unwrap());
    //aoc::solve!(1, part_one, input);
    //aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_owned()));
    }
}
