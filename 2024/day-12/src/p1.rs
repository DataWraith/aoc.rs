use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    let mut sum = 0;

    for region in find_regions(&input.garden).into_iter() {
        let border = generate_border(&region);
        sum += border.values().sum::<usize>() * region.len();
    }

    sum.to_string()
}

pub fn generate_border(region: &HashSet<Coordinate>) -> HashMap<Coordinate, usize> {
    let mut border = HashMap::new();

    for coord in region.iter() {
        for neighbor in coord.neighbors() {
            if !region.contains(&neighbor) {
                border.entry(neighbor).and_modify(|c| *c += 1).or_insert(1);
            }
        }
    }

    border
}

pub fn find_regions(input: &Grid2D<char>) -> Vec<HashSet<Coordinate>> {
    let mut union_find = UnionFind::default();
    let mut sets = HashMap::new();
    let mut result = vec![];

    for (coord, &plant) in input.iter() {
        let set = union_find.make_set();
        sets.insert(coord, set);
    }

    for (coord, &plant) in input.iter() {
        for neighbor in coord.neighbors() {
            if let Some(neighbor_plant) = input.get(neighbor) {
                if *neighbor_plant == plant {
                    let _ = union_find.union(sets[&coord], sets[&neighbor]);
                }
            }
        }
    }

    for root in union_find.roots() {
        let mut plot = HashSet::new();

        for (coord, &set) in sets.iter() {
            if union_find.find(set) == Some(root) {
                plot.insert(*coord);
            }
        }

        result.push(plot);
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
