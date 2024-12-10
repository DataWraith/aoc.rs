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

fn destinations(input: &PuzzleInput) -> Vec<Coordinate> {
    input
        .map
        .iter()
        .filter(|(_, &c)| c == 9)
        .map(|(p, _)| p)
        .collect_vec()
}

fn trail_destination_score(input: &PuzzleInput, head: Coordinate) -> usize {
    let destinations = destinations(input);

    let mut seen = HashSet::new();

    let mut successors = move |p: &(Coordinate, Vec<Coordinate>)| {
        let (current, trail) = p;

        let mut result = Vec::new();

        if *current == head {
            return result;
        }

        for dir in Direction::cardinal() {
            let neighbor = current.neighbor(dir);

            if let Some(n) = input.map.get(neighbor) {
                if *n + 1 == *input.map.get(*current).unwrap() {
                    let mut trail = trail.clone();
                    trail.push(neighbor);

                    if seen.insert((neighbor, trail.clone())) {
                        result.push((neighbor, trail));
                    }
                }
            }
        }

        result
    };

    let mut score = 0;
    let mut bfs = BrFS::new(destinations.iter().map(|d| (*d, vec![*d])).collect_vec());
    while let Some((next, _d)) = bfs.next(&mut successors) {
        if next == head {
            score += 1;
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
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
