use utility_belt::prelude::*;

use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    num_cave_paths(input).to_string()
}

pub fn num_cave_paths(input: &PuzzleInput) -> usize {
    let mut paths = 0;
    let mut q = VecDeque::new();

    q.push_back((vec!["start"], false));

    while let Some((current, small_cave_visited_twice)) = q.pop_front() {
        let last = current.last().unwrap();

        if *last == "end" {
            paths += 1;
            continue;
        }

        for neighbor in input.cave.neighbors(last) {
            if neighbor.starts_with(|c: char| c.is_ascii_lowercase()) && current.contains(&neighbor)
            {
                if small_cave_visited_twice || neighbor == "start" {
                    continue;
                }

                let mut new_path = current.clone();
                new_path.push(neighbor);

                q.push_back((new_path, true));
            } else {
                let mut new_path = current.clone();
                new_path.push(neighbor);

                q.push_back((new_path, small_cave_visited_twice));
            }
        }
    }

    paths
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn test_part2_example() {
        let input = parser::part2(parser::TEST_INPUT);
        assert_ne!(parser::TEST_INPUT.trim(), "TODO");
        assert_eq!(part2(&input), "36");
    }
}
