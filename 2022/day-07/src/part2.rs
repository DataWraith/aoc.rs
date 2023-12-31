use std::path::PathBuf;

use crate::{part1::directory_sizes, structs::*};

pub fn part2(input: &PuzzleInput) -> String {
    let sizes = directory_sizes(input);
    let free_space = 70000000 - sizes[&PathBuf::from("/")];
    let space_to_free = 30000000 - free_space;

    sizes
        .iter()
        .fold(usize::MAX, |acc, (_, &size)| {
            if size < space_to_free {
                return acc;
            }

            if size < acc {
                return size;
            }

            acc
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test.txt");

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "24933642");
    }
}
