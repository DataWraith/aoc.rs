use rayon::prelude::*;

use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    trail_heads(input)
        .par_iter()
        .map(|head| trail_head_score(input, *head))
        .sum::<usize>()
        .to_string()
}

fn trail_head_score(input: &PuzzleInput, head: Coordinate) -> usize {
    let mut score = 0;

    let mut seen = input.map.map(|_square| false);
    let mut q = VecDeque::from([head]);

    while let Some(p) = q.pop_front() {
        if input.map.get(p) == Some(&9) {
            score += 1;
            continue;
        }

        for neighbor in p.neighbors() {
            if let Some(n) = input.map.get(neighbor) {
                if *n == input.map[p] + 1 && !seen[neighbor] {
                    seen[neighbor] = true;
                    q.push_back(neighbor);
                }
            }
        }
    }

    score
}

pub fn trail_heads(input: &PuzzleInput) -> Vec<Coordinate> {
    input
        .map
        .iter()
        .filter(|(_, &c)| c == 0)
        .map(|(p, _)| p)
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = utility_belt::prelude::indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO");
        assert_eq!(part1(&input), "36");
    }
}
