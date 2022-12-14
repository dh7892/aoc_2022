use std::fmt::Display;
use std::rc::Rc;

use array2d::Array2D;
use euclid::Vector2D;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{i32 as nom32, newline};
use nom::sequence::{delimited, separated_pair, tuple};
use nom::{multi::separated_list1, IResult};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Content {
    Sand,
    Rock,
    Void,
}

impl Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Content::Sand => write!(f, "o"),
            Content::Rock => write!(f, "#"),
            Content::Void => write!(f, "."),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn point(input: &str) -> IResult<&str, Point> {
    let (input, (x, _, y)) = tuple((nom32, tag(","), nom32))(input)?;
    Ok((input, Point { x, y }))
}

struct IndexMapper {
    x_min: i32,
    y_min: i32,
}
impl IndexMapper {
    fn to_index(self: &Self, point: &Point) -> (usize, usize) {
        (
            // x => col, y=> row so need to swap order
            (point.y - self.y_min) as usize,
            (point.x - self.x_min) as usize,
        )
    }
}
fn path(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(tag(" -> "), point)(input)
}

fn path_list(input: &str) -> IResult<&str, Vec<Vec<Point>>> {
    separated_list1(newline, path)(input)
}

fn min_max_of_two(start: &Point, end: &Point) -> (Point, Point) {
    let x_min = std::cmp::min(start.x, end.x);
    let x_max = std::cmp::max(start.x, end.x);
    let y_min = std::cmp::min(start.y, end.y);
    let y_max = std::cmp::max(start.y, end.y);
    (Point { x: x_min, y: y_min }, Point { x: x_max, y: y_max })
}

fn find_min_max(paths: &Vec<Vec<Point>>) -> (Point, Point) {
    let mut min = Point { x: 500, y: 0 };
    let mut max = Point { x: 500, y: 0 };
    for path in paths {
        for point in path {
            if point.x < min.x {
                min.x = point.x;
            }
            if point.x > max.x {
                max.x = point.x;
            }
            if point.y < min.y {
                min.y = point.y;
            }
            if point.y > max.y {
                max.y = point.y;
            }
        }
    }
    (min, max)
}

fn input_to_grid(input: &str) -> (Array2D<Content>, IndexMapper) {
    let (_, paths) = path_list(&input).unwrap();
    let (min, max) = find_min_max(&paths);
    let mapper = IndexMapper {
        x_min: min.x,
        y_min: min.y,
    };
    let (cols, rows) = (max.x - min.x + 1, max.y - min.y + 1);
    let mut grid = Array2D::filled_with(Content::Void, rows as usize, cols as usize);

    for path in paths {
        for (start, end) in path.iter().tuple_windows() {
            let (min, max) = min_max_of_two(start, end);
            for x in min.x..=max.x {
                for y in min.y..=max.y {
                    let (i, j) = mapper.to_index(&Point { x, y });
                    grid[(i, j)] = Content::Rock;
                }
            }
        }
    }
    (grid, mapper)
}

fn input_to_grid_with_floor(input: &str) -> (Array2D<Content>, IndexMapper) {
    let (_, paths) = path_list(&input).unwrap();
    let (mut min, mut max) = find_min_max(&paths);
    let floor_depth = max.y + 2;
    max.y = floor_depth;
    min.x -= floor_depth + 10;
    max.x += floor_depth + 10;

    let mapper = IndexMapper {
        x_min: min.x,
        y_min: min.y,
    };
    let (cols, rows) = (max.x - min.x + 1, max.y - min.y + 1);
    let mut grid = Array2D::filled_with(Content::Void, rows as usize, cols as usize);

    // Put in floor
    for x in min.x..=max.x {
        let y = max.y;
        grid[mapper.to_index(&Point { x, y })] = Content::Rock;
    }

    for path in paths {
        for (start, end) in path.iter().tuple_windows() {
            let (min, max) = min_max_of_two(start, end);
            for x in min.x..=max.x {
                for y in min.y..=max.y {
                    let (i, j) = mapper.to_index(&Point { x, y });
                    grid[(i, j)] = Content::Rock;
                }
            }
        }
    }

    (grid, mapper)
}

fn print_grid(grid: &Array2D<Content>) {
    let s = grid
        .as_rows()
        .into_iter()
        .map(|row| row.into_iter().map(|e| e).join(""))
        .join("\n");
    println!("{}", s);
}

#[derive(PartialEq, Eq)]
enum SandOutcome {
    Stuck,
    Free,
}

fn future_locations(point: &Point) -> Vec<Point> {
    let next_row = point.y + 1;
    vec![
        Point {
            x: point.x,
            y: next_row,
        },
        Point {
            x: point.x - 1,
            y: next_row,
        },
        Point {
            x: point.x + 1,
            y: next_row,
        },
    ]
}

fn simulate_grain(
    grid: &mut Array2D<Content>,
    mapper: &IndexMapper,
    ingress: &Point,
) -> SandOutcome {
    let mut location = Point {
        x: ingress.x,
        y: ingress.y,
    };
    'outer: loop {
        '_inner: for l in future_locations(&location) {
            let (row, col) = mapper.to_index(&l);
            match grid.get(row, col) {
                Some(e) => match e {
                    Content::Void => {
                        location.x = l.x;
                        location.y = l.y;
                        continue 'outer; // No need to check other locations, we can move
                    }

                    _ => {} // blocked here so just continue checking
                },
                None => {
                    // We can move off the end of the grid so the sand grain must be free
                    return SandOutcome::Free;
                }
            }
        }
        grid[mapper.to_index(&location)] = Content::Sand;
        return SandOutcome::Stuck;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut grid, mapper) = input_to_grid(input);
    let ingress = Point { x: 500, y: 0 };
    print_grid(&grid);
    let mut count: u32 = 0;

    while simulate_grain(&mut grid, &mapper, &ingress) != SandOutcome::Free {
        count += 1;
    }
    print_grid(&grid);
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut grid, mapper) = input_to_grid_with_floor(input);
    let ingress = Point { x: 500, y: 0 };
    print_grid(&grid);
    let mut count: u32 = 0;

    while grid[mapper.to_index(&ingress)] != Content::Sand {
        simulate_grain(&mut grid, &mapper, &ingress);
        count += 1;
    }
    print_grid(&grid);
    let grains = grid
        .elements_column_major_iter()
        .map(|e| match e {
            Content::Sand => 1,
            _ => 0,
        })
        .sum::<u32>();
    Some(grains)
}

fn main() {
    let input = &aoc::read_file("inputs", 14);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }

    #[test]
    fn test_get_paths() {
        let input = aoc::read_file("examples", 14);
        let (_, paths) = path_list(&input).unwrap();
        assert_eq!(paths.len(), 2);
    }
    #[test]
    fn test_min_max() {
        let input = aoc::read_file("examples", 14);
        let (_, paths) = path_list(&input).unwrap();
        let (min, max) = find_min_max(&paths);
        let expected_min = Point { x: 494, y: 0 };
        let expected_max = Point { x: 503, y: 9 };
        assert_eq!(min, expected_min);
        assert_eq!(max, expected_max);
    }
    #[test]
    fn test_to_index() {
        let input = aoc::read_file("examples", 14);
        let (_, paths) = path_list(&input).unwrap();
        let (min, max) = find_min_max(&paths);
        let mapper = IndexMapper {
            x_min: min.x,
            y_min: min.y,
        };
        assert_eq!(mapper.to_index(&min), (0, 0));
        assert_eq!(mapper.to_index(&Point { x: 500, y: 0 }), (6, 0));
    }
}
