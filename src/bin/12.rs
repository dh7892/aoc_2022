
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};


type Map = Vec<Vec<u32>>;

#[derive(Debug, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    row_idx: usize,
    col_idx: usize,
    distance: u32,
}
// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.row_idx.cmp(&other.row_idx))
            .then_with(|| self.col_idx.cmp(&other.col_idx))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn target(row_idx: usize, col_idx: usize, direction: &Direction) -> (usize, usize) {
    match direction {
        Direction::Up => (row_idx - 1, col_idx),
        Direction::Down => (row_idx + 1, col_idx),
        Direction::Left => (row_idx, col_idx - 1),
        Direction::Right => (row_idx, col_idx + 1),
    }
}

fn letter_to_height(letter: char) -> u32 {
    let height_lookup = ('S'..='S')
        .chain('a'..='z')
        .chain('E'..='E')
        .enumerate()
        .map(|(idx, c)| (c, idx))
        .collect::<HashMap<char, usize>>();

    height_lookup[&letter] as u32
}

fn input_to_heights(input: &str) -> (Map, Map, usize, usize, usize, usize) {
    let (mut start_row, mut start_col) = (0, 0);
    let (mut end_row, mut end_col) = (0, 0);
    let result = input
        .lines()
        .enumerate()
        .map(|(row_idx, row)| {
            row.chars()
                .enumerate()
                .map(|(col_idx, c)| {
                    if c == 'S' {
                        (start_row, start_col) = (row_idx, col_idx);
                    }
                    if c == 'E' {
                        (end_row, end_col) = (row_idx, col_idx);
                    }
                    letter_to_height(c)
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Map>();

    let mut distance_map: Map = Vec::new();
    let rows = result.len();
    let cols = result[0].len();
    for _ in 0..rows {
        distance_map.push(vec![std::u32::MAX; cols]);
    }
    (result, distance_map, start_row, start_col, end_row, end_col)
}

/// Given a location return all allowed directions we could move
fn allowed_directions_for_location(
    row_idx: usize,
    col_idx: usize,
    heights: &Map,
) -> HashSet<Direction> {
    let mut result: HashSet<Direction> = HashSet::new();
    let current_height = heights[row_idx][col_idx];
    if row_idx > 0 {
        if (heights[row_idx - 1][col_idx] as i32 - current_height as i32) <= 1 {
            result.insert(Direction::Up);
        }
    }
    if row_idx < heights.len() - 1 {
        if (heights[row_idx + 1][col_idx] as i32 - current_height as i32) <= 1 {
            result.insert(Direction::Down);
        }
    }
    if col_idx > 0 {
        if (heights[row_idx][col_idx - 1] as i32 - current_height as i32) <= 1 {
            result.insert(Direction::Left);
        }
    }
    if col_idx < heights[0].len() - 1 {
        if (heights[row_idx][col_idx + 1] as i32 - current_height as i32) <= 1 {
            result.insert(Direction::Right);
        }
    }
    result
}

/// From a given location, consider adjacent locations and update their distances
fn process_location(
    distance: u32,
    row_idx: usize,
    col_idx: usize,
    heights: &Map,
    distances: &mut Map,
    reset_at_a: bool,
) {
    // Update our distance
    let mut corrected_dist = distance;
    if reset_at_a && heights[row_idx][col_idx] == 1 {
        // Reset to 0 as we are on the ground
        corrected_dist = 0;
    }
    distances[row_idx][col_idx] = corrected_dist;
    let allowed_directions = allowed_directions_for_location(row_idx, col_idx, heights);
    for direction in allowed_directions.iter() {
        let (target_row, target_col) = target(row_idx, col_idx, direction);
        let target_distance = corrected_dist + 1;
        if distances[target_row][target_col] > target_distance {
            process_location(
                target_distance,
                target_row,
                target_col,
                heights,
                distances,
                reset_at_a,
            );
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (heights, mut distances, start_row, start_col, end_row, end_col) = input_to_heights(input);
    process_location(0, start_row, start_col, &heights, &mut distances, false);
    let result = distances[end_row][end_col];
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (heights, mut distances, start_row, start_col, end_row, end_col) = input_to_heights(input);
    process_location(0, start_row, start_col, &heights, &mut distances, true);
    let result = distances[end_row][end_col];
    Some(result)
}

fn main() {
    let input = &aoc::read_file("inputs", 12);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
    #[test]
    fn test_letter_to_height() {
        assert_eq!(letter_to_height('S'), 0);
        assert_eq!(letter_to_height('E'), 27);
        assert_eq!(letter_to_height('a'), 1);
        assert_eq!(letter_to_height('g'), 7);
    }
}
