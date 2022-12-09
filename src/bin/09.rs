use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Posn {
    x: i32,
    y: i32,
}

// Given head and tail positions, return new tail position
fn move_tail(head: Posn, tail: Posn) -> Posn {
    let (d_x, d_y) = (tail.x - head.x, tail.y - head.y);

    if d_x.abs() <= 1 && d_y.abs() <= 1 {
        // We are still close enough so no need to move tail
        return tail.clone();
    }
    if d_x.abs() == d_y.abs() {
        // We are on a diagonal so we need to stay on the diagonal
        return Posn {
            x: head.x + d_x / d_x.abs(),
            y: head.y + d_y / d_y.abs(),
        };
    }
    if d_x.abs() > d_y.abs() {
        // x direction dominates so y will end up level with head
        return Posn {
            x: head.x + d_x / d_x.abs(),
            y: head.y,
        };
    }
    if d_x.abs() < d_y.abs() {
        // y direction dominates so x will end up level with head
        return Posn {
            x: head.x,
            y: head.y + d_y / d_y.abs(),
        };
    }

    Posn {
        x: tail.x,
        y: tail.y,
    }
}

// Apply all moves for a line and update head and tail. Track all tail locations visited
fn execute_line(direction: &str, count: u32, rope: &mut Vec<Posn>, visited: &mut HashSet<Posn>) {
    let delta = match direction {
        "U" => Posn { x: 0, y: 1 },
        "D" => Posn { x: 0, y: -1 },
        "L" => Posn { x: -1, y: 0 },
        "R" => Posn { x: 1, y: 0 },
        _ => Posn { x: 0, y: 0 },
    };
    for _ in 0..count {
        // Move the very head of the rope first
        rope[0].x += delta.x;
        rope[0].y += delta.y;
        for idx in 1..rope.len() {
            let lead = rope[idx - 1];
            let follow = rope[idx];
            rope[idx] = move_tail(lead, follow);
        }
        visited.insert(rope.last().unwrap().clone());
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let start_pos = Posn { x: 0, y: 0 };
    let mut locations = HashSet::<Posn>::new();
    let mut rope: Vec<Posn> = vec![start_pos; 2];
    locations.insert(rope.last().unwrap().clone());

    for line in input.lines() {
        let mut words = line.split(" ");
        let direction = words.next().unwrap();
        let count = words.next().unwrap().parse::<u32>().unwrap();
        execute_line(direction, count, &mut rope, &mut locations);
    }

    Some(locations.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let start_pos = Posn { x: 0, y: 0 };
    let mut locations = HashSet::<Posn>::new();
    let mut rope: Vec<Posn> = vec![start_pos; 10];
    locations.insert(rope.last().unwrap().clone());

    for line in input.lines() {
        let mut words = line.split(" ");
        let direction = words.next().unwrap();
        let count = words.next().unwrap().parse::<u32>().unwrap();
        execute_line(direction, count, &mut rope, &mut locations);
    }

    Some(locations.len() as u32)
}

fn main() {
    let input = &aoc::read_file("inputs", 9);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }

    #[test]
    fn test_move_tail() {
        let head = Posn { x: 0, y: 0 };
        assert_eq!(
            move_tail(head, Posn { x: -1, y: -1 }),
            Posn { x: -1, y: -1 }
        );
        let head = Posn { x: 0, y: 0 };
        let tail = Posn { x: -2, y: -1 };
        let expected = Posn { x: -1, y: 0 };
        assert_eq!(move_tail(head, tail), expected);

        let head = Posn { x: 0, y: 0 };
        let tail = Posn { x: 2, y: 2 };
        let expected = Posn { x: 1, y: 1 };
        assert_eq!(move_tail(head, tail), expected);

        let head = Posn { x: 0, y: 0 };
        let tail = Posn { x: 2, y: -2 };
        let expected = Posn { x: 1, y: -1 };
        assert_eq!(move_tail(head, tail), expected);

        let head = Posn { x: 4, y: 2 };
        let tail = Posn { x: 3, y: 0 };
        let expected = Posn { x: 4, y: 1 };
        assert_eq!(move_tail(head, tail), expected);

        let head = Posn { x: 5, y: 3 };
        let tail = Posn { x: 3, y: 4 };
        let expected = Posn { x: 4, y: 3 };
        assert_eq!(move_tail(head, tail), expected);

        let head = Posn { x: 5, y: 3 };
        let tail = Posn { x: 4, y: 5 };
        let expected = Posn { x: 5, y: 4 };
        assert_eq!(move_tail(head, tail), expected);

        let head = Posn { x: 5, y: 3 };
        let tail = Posn { x: 6, y: 5 };
        let expected = Posn { x: 5, y: 4 };
        assert_eq!(move_tail(head, tail), expected);

        let head = Posn { x: 5, y: 3 };
        let tail = Posn { x: 7, y: 2 };
        let expected = Posn { x: 6, y: 3 };
        assert_eq!(move_tail(head, tail), expected);

        let head = Posn { x: 5, y: 3 };
        let tail = Posn { x: 2, y: 2 };
        let expected = Posn { x: 4, y: 3 };
        assert_eq!(move_tail(head, tail), expected);
    }
}
