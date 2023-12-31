use std::{collections::BTreeMap, path::PathBuf};

use crate::structs::*;

pub fn part1(input: &PuzzleInput) -> String {
    let sizes = directory_sizes(input);
    let mut sum = 0;

    for size in sizes.values() {
        if *size <= 100_000 {
            sum += size;
        }
    }

    sum.to_string()
}

pub fn directory_sizes(input: &PuzzleInput) -> BTreeMap<PathBuf, usize> {
    let directories = input
        .filesystem
        .iter()
        .filter(|(_path, size)| **size == 0)
        .map(|(path, _)| path)
        .cloned()
        .collect::<Vec<_>>();

    let mut directory_sizes = BTreeMap::new();

    for dir in directories.iter() {
        let mut size = 0;

        // This is inefficient, but whatever. It still runs fast enough.
        for (path, file_size) in input.filesystem.iter() {
            if path.starts_with(dir) {
                size += file_size;
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
