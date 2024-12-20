use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut sum = 0;

    for region in find_regions(&input.garden).into_iter() {
        let border = generate_border(&region);
        sum += border.len() * region.len();
    }

    sum.to_string()
}

pub fn generate_border(region: &HashSet<Coordinate>) -> Vec<Coordinate> {
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

pub fn find_regions(input: &Grid2D<char>) -> Vec<HashSet<Coordinate>> {
    let mut sets = input.map(|_| 0);
    let mut union_find = UnionFind::default();

    for (coord, _) in input.iter() {
        sets[coord] = union_find.make_set();
    }

    for (coord, &plant) in input.iter() {
        for neighbor in [
            coord.neighbor(Direction::Right),
            coord.neighbor(Direction::Down),
        ] {
            if let Some(neighbor_plant) = input.get(neighbor) {
                if *neighbor_plant == plant {
                    let _ = union_find.union(sets[coord], sets[neighbor]);
                }
            }
        }
    }

    let mut regions = HashMap::new();

    for (coord, &set) in sets.iter() {
        let root = union_find.find(set).unwrap();

        regions
            .entry(root)
            .or_insert_with(HashSet::new)
            .insert(coord);
    }

    regions.into_values().collect()
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
