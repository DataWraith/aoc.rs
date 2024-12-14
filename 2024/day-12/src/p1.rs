use std::collections::BTreeSet;

use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    let mut sum = 0;

    for region in find_regions(&input.garden).into_iter() {
        let border = generate_border(&region);
        sum += border.len() * region.len();
    }

    sum.to_string()
}

pub fn generate_border(region: &BTreeSet<Coordinate>) -> Vec<Coordinate> {
    let mut border = Vec::new();

    for coord in region.iter() {
        for neighbor in coord.neighbors() {
            if !region.contains(&neighbor) {
                border.push(neighbor);
            }
        }
    }

    border
}

// Find all connected regions with plants of the same type using a
// Breadth-First Search.
pub fn find_regions(input: &Grid2D<char>) -> Vec<BTreeSet<Coordinate>> {
    let mut seen = input.map(|_| false);
    let mut result = vec![];

    for (coord, &plant) in input.iter() {
        if seen[coord] {
            continue;
        }

        let mut region = BTreeSet::new();
        let mut q = VecDeque::from([coord]);

        while let Some(c) = q.pop_front() {
            if seen[c] {
                continue;
            } else {
                seen[c] = true;
            }

            region.insert(c);

            q.extend(
                c.neighbors()
                    .into_iter()
                    .filter(|&n| input.get(n) == Some(&plant) && !seen[n]),
            );
        }

        result.push(region);
    }

    result
}
#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

    const TEST_INPUT: &str = indoc! {"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
"};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO");
        assert_eq!(part1(&input), "772");
    }
}
