use std::collections::HashMap;

use itertools::Itertools;

fn update_tree(tree: &mut HashMap<String, usize>, path: &Vec<String>, size: usize) {
    // Update the tree with the file provided
    let mut cwd = path.clone();
    while cwd.len() > 0 {
        let path_string = cwd.join("");
        let current_val = tree.get(&path_string);
        match current_val {
            Some(_) => {
                tree.get_mut(&path_string).map(|old| *old += size);
            }
            _ => {
                tree.insert(path_string, size);
            }
        }
        cwd.pop();
    }
}

fn dirs_from_input(input: &str) -> HashMap<String, usize> {
    let mut cwd: Vec<String> = Vec::new();
    let mut dirs: HashMap<String, usize> = HashMap::new();
    let lines: Vec<&str> = input.lines().collect();

    for line in lines {
        let words: Vec<&str> = line.split(" ").collect();
        if words[0] == "$" {
            // Command
            match words[1] {
                "cd" => {
                    let dest = words[2];
                    match dest {
                        ".." => {
                            cwd.pop();
                        }
                        "/" => {
                            cwd.clear();
                            cwd.push(dest.to_owned());
                        }
                        _ => {
                            let mut new_dir = dest.to_owned();
                            new_dir.push_str("/");
                            cwd.push(new_dir);
                        }
                    }
                }
                _ => {}
            }
        } else {
            match words[1] {
                "dir" => {}
                _ => {
                    let first = words[0];
                    let second = words[1];
                    match first {
                        "dir" => {}
                        _ => {
                            let size = first.parse::<usize>().unwrap();
                            let _ = second.to_owned();
                            update_tree(&mut dirs, &cwd, size);
                        }
                    }
                }
            }
        }
    }
    dirs
}

pub fn part_one(input: &str) -> Option<u32> {
    let dirs = dirs_from_input(input);
    let max_size: usize = 100000;
    let total: usize = dirs.values().filter(|v| v <= &&max_size).sum();
    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let dirs = dirs_from_input(input);
    let current_size = dirs.get("/").unwrap();
    let max_size = 70000000;
    let needed_space = 30000000;
    let current_space = max_size - current_size;
    let need_to_free = needed_space - current_space;

    let size_to_free = dirs
        .values()
        .sorted()
        .find(|v| v > &&need_to_free)
        .unwrap()
        .clone();
    Some(size_to_free as u32)
}

fn main() {
    let input = &aoc::read_file("inputs", 7);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
