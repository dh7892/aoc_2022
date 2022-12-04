fn line_to_score(line: &str) -> u32 {
    let mut domains = line.split(',');
    let d1 = domains
        .next()
        .unwrap()
        .split('-')
        .map(|item| item.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let d2 = domains
        .next()
        .unwrap()
        .split('-')
        .map(|item| item.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let (mut outer_min, mut outer_max) = (d1[0], d1[1]);
    let (mut inner_min, mut inner_max) = (d2[0], d2[1]);
    if d1[1] - d1[0] < d2[1] - d2[0] {
        (outer_min, outer_max) = (d2[0], d2[1]);
        (inner_min, inner_max) = (d1[0], d1[1]);
    }

    if inner_min >= outer_min && inner_max <= outer_max {
        // dbg!((outer_min, outer_max, inner_min, inner_max, "intersecting"));
        1
    } else {
        // dbg!((outer_min, outer_max, inner_min, inner_max, "not"));
        0
    }
}

fn line_to_overlap(line: &str) -> u32 {
    let mut domains = line.split(',');
    let d1 = domains
        .next()
        .unwrap()
        .split('-')
        .map(|item| item.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let d2 = domains
        .next()
        .unwrap()
        .split('-')
        .map(|item| item.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let (a_s, a_e) = (d1[0], d1[1]);
    let (b_s, b_e) = (d2[0], d2[1]);

    if a_s > b_e || b_s > a_e {
        0
    } else {
        1
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let total = input.lines().map(line_to_score).sum();
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let total = input.lines().map(line_to_overlap).sum();
    Some(total)
}

fn main() {
    let input = &aoc::read_file("inputs", 4);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
