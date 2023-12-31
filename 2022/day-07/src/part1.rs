use std::collections::BTreeMap;

use crate::structs::*;

use utility_belt::prelude::*;

pub fn part1(input: &PuzzleInput) -> String {
    let sizes = directory_sizes(input);
    let mut sum = 0;

    for (_dir, size) in sizes.iter() {
        if *size <= 100_000 {
            sum += size;
        }
    }

    sum.to_string()
}

pub fn directory_sizes(input: &PuzzleInput) -> BTreeMap<String, usize> {
    let mut directories = input
        .filesystem
        .iter()
        .filter(|(_path, size)| **size == 0)
        .map(|(path, _)| path)
        .cloned()
        .collect::<Vec<_>>();

    directories.push("/".to_string());

    let mut directory_sizes = BTreeMap::new();

    for dir in directories.iter() {
        let mut size = 0;

        for (path, file_size) in input.filesystem.iter() {
            if path.starts_with(dir) {
                size += file_size.saturating_sub(1);
            }
        }

        directory_sizes.insert(dir.clone(), size);
    }

    directory_sizes
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part1() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part1(&input), "95437");
    }
}
