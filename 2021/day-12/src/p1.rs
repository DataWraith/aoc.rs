use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let paths = cave_paths(input);
    paths.len().to_string()
}

pub fn cave_paths(input: &PuzzleInput) -> Vec<Vec<&str>> {
    let mut paths = Vec::new();
    let mut q = VecDeque::new();

    q.push_back(vec!["start"]);

    while let Some(current) = q.pop_front() {
        let last = current.last().unwrap();

        if *last == "end" {
            paths.push(current);
            continue;
        }

        for neighbor in input.cave.neighbors(last) {
            if neighbor.chars().all(|c| c.is_ascii_lowercase()) && current.contains(&neighbor) {
                continue;
            }

            let mut new_path = current.clone();
            new_path.push(neighbor);

            q.push_back(new_path);
        }
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use utility_belt::prelude::*;

    #[test]
    fn test_part1_example() {
        let input = parser::part1(parser::TEST_INPUT);
        assert_ne!(parser::TEST_INPUT.trim(), "TODO");
        assert_eq!(part1(&input), "10");
    }
}
