use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    let heads = trail_heads(input);
    let mut scores = Vec::new();

    for head in heads {
        let score = trail_head_score(input, head);
        scores.push(score);
    }

    scores.iter().sum::<usize>().to_string()
}

fn trail_head_score(input: &PuzzleInput, head: Coordinate) -> usize {
    let mut seen = input.map.map(|_square| false);

    let mut successors = move |p: &Coordinate| {
        let mut result = Vec::new();

        for neighbor in p.neighbors() {
            if let Some(n) = input.map.get(neighbor) {
                if *n == input.map[*p] + 1 && !seen[neighbor] {
                    seen[neighbor] = true;
                    result.push(neighbor);
                }
            }
        }

        result
    };

    let mut score = 0;
    let mut bfs = BrFS::new(vec![head]);
    while let Some(next) = bfs.next(&mut successors) {
        if input.map.get(next) == Some(&9) {
            score += 1;
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
