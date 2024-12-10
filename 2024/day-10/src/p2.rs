use utility_belt::prelude::*;

use crate::{p1::trail_heads, parser::*};

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let heads = trail_heads(input);
    let mut scores = Vec::new();

    for head in heads {
        let score = trail_destination_score(input, head);
        scores.push(score);
    }

    scores.iter().sum::<usize>().to_string()
}

fn trail_destination_score(input: &PuzzleInput, head: Coordinate) -> usize {
    let start = head;

    let mut successors = move |p: &Coordinate| {
        let mut result = Vec::new();

        if input.map.get(*p) == Some(&9) {
            return result;
        }

        for dir in Direction::cardinal() {
            let neighbor = p.neighbor(dir);
            if let Some(n) = input.map.get(neighbor) {
                if *n == input.map.get(*p).unwrap() + 1 {
                    result.push(neighbor);
                }
            }
        }

        result
    };

    let mut score = 0;
    let mut bfs = BrFS::new(vec![start]);
    while let Some(next) = bfs.next(&mut successors) {
        if input.map.get(next) == Some(&9) {
            score += 1;
        }
    }

    score
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
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO");
        assert_eq!(part2(&input), "81");
    }
}
