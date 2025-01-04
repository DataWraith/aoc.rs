use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    num_cache_paths(input).to_string()
}

pub fn num_cache_paths(input: &PuzzleInput) -> usize {
    let mut paths = 0;
    let mut q = VecDeque::new();

    q.push_back(vec!["start"]);

    while let Some(current) = q.pop_front() {
        let last = current.last().unwrap();

        if *last == "end" {
            paths += 1;
            continue;
        }

        for neighbor in input.cave.neighbors(last) {
            if neighbor.starts_with(|c: char| c.is_ascii_lowercase()) && current.contains(&neighbor)
            {
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

    #[test]
    fn test_part1_example() {
        let input = parser::part1(parser::TEST_INPUT);
        assert_ne!(parser::TEST_INPUT.trim(), "TODO");
        assert_eq!(part1(&input), "10");
    }
}
