fn visible_in_matrix(matrix: Vec<Vec<u32>>, viz: &mut Vec<Vec<u32>>) {
    let (rows, cols) = (matrix[0].len(), matrix.len());

    for (row_idx, row) in matrix.iter().enumerate() {
        let mut current_highest: i32 = -1;
        for (col_idx, cell) in row.iter().enumerate() {
            if *cell as i32 > current_highest {
                viz[row_idx][col_idx] = 1;
                current_highest = *cell as i32;
            }
        }

        let mut current_highest: i32 = -1;
        for (col_idx, cell) in row.iter().enumerate().rev() {
            if *cell as i32 > current_highest {
                viz[row_idx][col_idx] = 1;
                current_highest = *cell as i32;
            }
        }
    }
    // Do cols
    for col_idx in 0..cols {
        let mut current_highest: i32 = -1;
        for row_idx in 0..rows {
            let val = matrix[row_idx][col_idx];
            if val as i32 > current_highest {
                viz[row_idx][col_idx] = 1;
                current_highest = val as i32;
            }
        }
        let mut current_highest: i32 = -1;
        for row_idx in (0..rows).rev() {
            let val = matrix[row_idx][col_idx];
            if val as i32 > current_highest {
                viz[row_idx][col_idx] = 1;
                current_highest = val as i32;
            }
        }
    }
}

fn probe(map: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    let (rows, cols) = (map[0].len(), map.len());
    let height_at_poi = map[row][col];
    // Look left
    let mut total_left = 0;
    for row_idx in (0..row).rev() {
        total_left += 1;
        if map[row_idx][col] >= height_at_poi {
            break;
        }
    }
    // Look right
    let mut total_right = 0;
    for row_idx in row + 1..rows {
        total_right += 1;
        if map[row_idx][col] >= height_at_poi {
            break;
        }
    }
    // Look up
    let mut total_up = 0;
    for col_idx in (0..col).rev() {
        total_up += 1;
        if map[row][col_idx] >= height_at_poi {
            break;
        }
    }
    // Look down
    let mut total_down = 0;
    for col_idx in col + 1..cols {
        total_down += 1;
        if map[row][col_idx] >= height_at_poi {
            break;
        }
    }
    total_up * total_left * total_down * total_right
}

fn input_to_matrix(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.split("")
                .filter(|v| !v.is_empty())
                .map(|v| v.parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}
pub fn part_one(input: &str) -> Option<u32> {
    let matrix = input_to_matrix(input);
    let mut viz: Vec<Vec<u32>> = matrix
        .iter()
        .map(|row| row.iter().map(|_| 0).collect())
        .collect();

    visible_in_matrix(matrix, &mut viz);

    let total = viz.iter().map(|row| row.iter().sum::<u32>()).sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = input_to_matrix(input);
    let (rows, cols) = (matrix[0].len(), matrix.len());
    let los_map: Vec<Vec<u32>> = (0..rows)
        .map(|row_idx| {
            (0..cols)
                .map(|col_idx| probe(&matrix, row_idx as usize, col_idx as usize))
                .collect()
        })
        .collect();
    let result = los_map
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap();
    Some(*result)
}

fn main() {
    let input = &aoc::read_file("inputs", 8);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
