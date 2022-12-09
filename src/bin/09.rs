use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Posn {
    x: i32,
    y: i32,
}

// Given head and tail positions, return new tail position
fn move_tail(head: Posn, tail: Posn) -> Posn {
    let (d_x, d_y) = (tail.x - head.x, tail.y - head.y);
    let mut move_x = 0;
    let mut move_y = 0;
    if d_x == 0 || d_y == 0 {
        if d_x == -2 {
            move_x = 1
        };
        if d_x == 2 {
            move_x = -1
        };
        if d_y == -2 {
            move_y = 1
        };
        if d_y == 2 {
            move_y = -1
        };
    }
    if d_x < -1 || d_x > 1 || d_y < -1 || d_y > 1 {
        // Need to move diagonally
        if d_x < 0 {
            move_x = 1;
        }
        if d_x > 0 {
            move_x = -1;
        }
        if d_y < 0 {
            move_y = 1;
        }
        if d_y > 0 {
            move_y = -1;
        }
    }

    Posn {
        x: tail.x + move_x,
        y: tail.y + move_y,
    }
}

// Apply all moves for a line and update head and tail. Track all tail locations visited
fn execute_line(
    direction: &str,
    count: u32,
    head: &mut Posn,
    tail: &mut Posn,
    visited: &mut HashSet<Posn>,
) {
    let delta = match direction {
        "U" => Posn { x: 0, y: 1 },
        "D" => Posn { x: 0, y: -1 },
        "L" => Posn { x: -1, y: 0 },
        "R" => Posn { x: 1, y: 0 },
        _ => Posn { x: 0, y: 0 },
    };
    for _ in 0..count {
        visited.insert(tail.clone());
        head.x += delta.x;
        head.y += delta.y;
        let new_tail = move_tail(head.clone(), tail.clone());
        tail.x = new_tail.x;
        tail.y = new_tail.y;
        visited.insert(tail.clone());
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut locations = HashSet::<Posn>::new();
    let mut head = Posn { x: 0, y: 0 };
    let mut tail = Posn { x: 0, y: 0 };
    for line in input.lines() {
        let mut words = line.split(" ");
        let direction = words.next().unwrap();
        let count = words.next().unwrap().parse::<u32>().unwrap();
        execute_line(direction, count, &mut head, &mut tail, &mut locations);
    }

    Some(locations.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
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
    }
}
