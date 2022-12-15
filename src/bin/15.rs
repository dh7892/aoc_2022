use std::collections::HashSet;

use nom::bytes::complete::tag;
use nom::character::complete::{i32 as nom32, newline};
use nom::{multi::separated_list1, IResult};

#[derive(Debug, Clone, Copy)]
struct Sensor {
    x: i32,
    y: i32,
    b_x: i32,
    b_y: i32,
}

impl Sensor {
    fn x_range_at_row(self: &Self, row: i32) -> Option<Range> {
        let m = manhattan((self.x, self.y), (self.b_x, self.b_y));
        let dy = (row - self.y).abs();
        if dy > m {
            return None;
        }
        let min = self.x - (m - dy).max(0);
        let max = self.x + (m - dy).max(0);
        Some(Range {
            start: min,
            end: max,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn overlapping(self: &Self, other: &Range) -> bool {
        (other.start >= self.start && other.start <= self.end)
            || (other.end >= self.start && other.end <= self.end)
    }
}

/// Struct that holds a sorted list of non-overlapping ranges bounded at both ends
struct BoundedRangeList {
    min: Option<i32>,
    max: Option<i32>,
    ranges: Vec<Range>,
}

impl BoundedRangeList {
    fn new(min: Option<i32>, max: Option<i32>) -> BoundedRangeList {
        BoundedRangeList {
            min: min,
            max: max,
            ranges: Vec::<Range>::new(),
        }
    }
    fn insert(self: &mut Self, range: &mut Range) -> () {
        // Set range within bounds
        if let Some(min) = self.min {
            if range.end < min {
                return;
            }
            range.start = range.start.max(min);
        }
        if let Some(max) = self.max {
            if range.start > max {
                return;
            }
            range.end = range.end.min(max);
        }

        // Find index of overlapping ranges
        let mut new_ranges = Vec::<Range>::new();
        let mut have_pushed = false;
        for r in &self.ranges {
            if r.overlapping(range) || range.overlapping(r) {
                range.start = range.start.min(r.start);
                range.end = range.end.max(r.end);
            } else {
                if range.end < r.start {
                    new_ranges.push(range.clone());
                    have_pushed = true;
                }
                new_ranges.push(r.clone());
            }
        }
        if !have_pushed {
            new_ranges.push(range.clone());
        }
        self.ranges = new_ranges;
    }
    fn len(self: &Self) -> u32 {
        self.ranges
            .iter()
            .map(|r| (r.end - r.start + 1) as u32)
            .sum()
    }
}

fn manhattan(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn line_to_beacon(input: &str) -> IResult<&str, Sensor> {
    let (input, _) = tag("Sensor at x=")(input)?;
    let (input, x) = nom32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y) = nom32(input)?;
    let (input, _) = tag(": closest beacon is at x=")(input)?;
    let (input, b_x) = nom32(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, b_y) = nom32(input)?;
    Ok((input, Sensor { x, y, b_x, b_y }))
}

fn input_to_sensors(input: &str) -> IResult<&str, Vec<Sensor>> {
    separated_list1(newline, line_to_beacon)(input)
}

fn excluded_from_row(sensors: &Vec<Sensor>, row: i32) -> u32 {
    let mut ranges = BoundedRangeList::new(None, None);
    let mut beacons_on_row = HashSet::<i32>::new();
    for s in sensors {
        if s.b_y == row {
            beacons_on_row.insert(s.b_x);
        }
        let range_on_row = s.x_range_at_row(row);
        if let Some(mut range) = range_on_row {
            ranges.insert(&mut range);
        }
    }
    ranges.len() - beacons_on_row.len() as u32
}

fn row_has_missing_spot(sensors: &Vec<Sensor>, row: i32, min: i32, max: i32) -> Option<i32> {
    // Check if the row has exactly 1 unknown item on it
    // if so ,return the idx of that missing space
    let mut ranges = BoundedRangeList::new(Some(min), Some(max));
    let mut beacons_on_row = HashSet::<i32>::new();
    for s in sensors {
        if s.b_y == row {
            beacons_on_row.insert(s.b_x);
        }
        let range_on_row = s.x_range_at_row(row);
        if let Some(mut range) = range_on_row {
            ranges.insert(&mut range);
        }
    }
    // We have our missing space if we have exactly 2 ranges with a single gap in between
    if ranges.ranges.len() == 2 && ranges.ranges[0].end + 2 == ranges.ranges[1].start {
        return Some(ranges.ranges[0].end + 1);
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, sensors) = input_to_sensors(input).unwrap();
    let row = 2000000;
    Some(excluded_from_row(&sensors, row))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, sensors) = input_to_sensors(input).unwrap();
    let max = 4000000;
    let scale = 4000000;
    for row in 0..=max {
        if let Some(col) = row_has_missing_spot(&sensors, row, 0, max) {
            dbg!(col as u128 * scale as u128 + row as u128);
        }
    }
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 15);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 15);
        let row = 10;
        let (_, sensors) = input_to_sensors(&input).unwrap();

        let val = excluded_from_row(&sensors, row);
        assert_eq!(val, 26);
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 15);
        let (_, sensors) = input_to_sensors(&input).unwrap();
        let max = 20;
        let scale = 4000000;
        let mut result: Option<u32> = None;
        for row in 0..=max {
            if let Some(col) = row_has_missing_spot(&sensors, row, 0, max) {
                result = Some((col * scale + row) as u32);
            }
        }
        assert_eq!(result, Some(56000011));
    }

    #[test]
    fn test_range_for_row() {
        let s = Sensor {
            x: 8,
            y: 7,
            b_x: 2,
            b_y: 10,
        };
        assert_eq!(s.x_range_at_row(-1), Some(Range { start: 7, end: 10 }));
        assert_eq!(s.x_range_at_row(-2), Some(Range { start: 8, end: 9 }));
        assert_eq!(s.x_range_at_row(-3), None);
        assert_eq!(s.x_range_at_row(-4), None);
        assert_eq!(s.x_range_at_row(16), Some(Range { start: 8, end: 9 }));
        assert_eq!(s.x_range_at_row(17), None);
    }
    #[test]
    fn test_merge_ranges() {
        let mut ranges = BoundedRangeList::new(Some(0), Some(10));
        ranges.insert(&mut Range { start: 0, end: 2 });
        assert_eq!(ranges.ranges, vec![Range { start: 0, end: 2 }]);
        assert_eq!(ranges.len(), 3);
        ranges.insert(&mut Range { start: 0, end: 2 });
        assert_eq!(ranges.ranges, vec![Range { start: 0, end: 2 }]);
        assert_eq!(ranges.len(), 3);
        ranges.insert(&mut Range { start: 1, end: 3 });
        assert_eq!(ranges.ranges, vec![Range { start: 0, end: 3 }]);
        assert_eq!(ranges.len(), 4);
        ranges.insert(&mut Range { start: 6, end: 7 });
        assert_eq!(
            ranges.ranges,
            vec![Range { start: 0, end: 3 }, Range { start: 6, end: 7 }]
        );
        assert_eq!(ranges.len(), 6);
        ranges.insert(&mut Range { start: 3, end: 6 });
        assert_eq!(ranges.ranges, vec![Range { start: 0, end: 7 }]);
        assert_eq!(ranges.len(), 8);

        // New ranges
        let mut ranges = BoundedRangeList::new(None, None);
        ranges.insert(&mut Range { start: 0, end: 2 });
        ranges.insert(&mut Range { start: 3, end: 3 });
        ranges.insert(&mut Range { start: 6, end: 8 });
        ranges.insert(&mut Range { start: 1, end: 7 });
        assert_eq!(ranges.ranges, vec![Range { start: 0, end: 8 }]);
        assert_eq!(ranges.len(), 9);
    }
}
