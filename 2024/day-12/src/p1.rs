use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    find_regions3(input);
    todo!("day_12::p1::part1");
}


fn find_regions3(input: &PuzzleInput) -> Vec<HashSet<Coordinate>> {
    let mut union_find = UnionFind::default();
    let mut sets = HashMap::new();

    for (coord, &plant) in input.garden.iter() {
        let set = union_find.make_set();
        sets.insert(coord, set);
    }

    for (coord, &plant) in input.garden.iter() {
        for neighbor in coord.neighbors() {
            if let Some(neighbor_plant) = input.garden.get(neighbor) {
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
                plot.insert(coord);
            }
        }

        let mut next_plot = HashSet::new();

        for coord in plot.iter() {
            next_plot.extend(coord.neighbors());

            for neighbor in coord.neighbors() {
                next_plot.insert(neighbor);
            }
        }

        let mut reg_fence = Vec::new();

        let mut cur = *next_plot.iter().min().unwrap();

        'outer: loop {
            reg_fence.push(cur);

            for neighbor in cur.neighbors() {
                if reg_fence.contains(&neighbor) {
                    continue;
                }

                if next_plot.contains(&neighbor) {
                    cur = neighbor;
                    continue 'outer;
                }
            }

            if reg_fence.len() == next_plot.len() {
                break;
            }
        }

        let vertices = reg_fence.into_iter().map(|c| (c.x, c.y)).collect_vec();

        let area = polygon_area(&vertices) + (1 + vertices.len() as i32) / 2;

        dbg!(&area);
    }

    todo!()
}

/*
fn find_regions2(input: &PuzzleInput) -> Vec<HashSet<Coordinate>> {
    let mut coordinates = HashMap::new();
    let mut regions = vec![];

    input.garden.iter().for_each(|(coord, &plant)| {
        coordinates.entry(plant).or_insert(Vec::new()).push(coord);
    });

    for (plant, coords) in coordinates.iter_mut() {
        let mut fence = HashSet::new();

        for coord in coords.iter() {
            for neighbor in coord.moore_neighbors() {
                if input.garden.get(neighbor).is_none() {
                    fence.insert(neighbor);
                } else if input.garden.get(neighbor).unwrap() != plant {
                    fence.insert(neighbor);
                }
            }
        }

        regions.push(fence);
    }

    for region in regions.iter() {
        let mut reg_fence = Vec::new();

        let mut cur = *region.iter().min().unwrap();

        'outer: loop {
            reg_fence.push(cur);

            for neighbor in cur.neighbors() {
                if reg_fence.contains(&neighbor) {
                    continue;
                }

                if region.contains(&neighbor) {
                    cur = neighbor;
                    continue 'outer;
                }
            }

            if reg_fence.len() == region.len() {
                break;
            }
        }

        let vertices = reg_fence.into_iter().map(|c| (c.x, c.y)).collect_vec();

        dbg!(&vertices);

        let area = polygon_area(&vertices) + vertices.len() as i32 / 2 + 1;

        dbg!(&area);
    }

    regions
}
*/
/*
fn find_regions(
    input: &PuzzleInput,
    seed: Coordinate,
    cols: Vec<Coordinate>,
    rows: Vec<Coordinate>,
) {
    let mut coordinates = vec![];

    let mut plant = input.garden[seed];

    dbg!(&cols);

    'outer: for (r, row) in rows.iter().enumerate() {
        for (c, col) in cols.iter().enumerate() {
            let r = r as i32;
            let c = c as i32;

            if let Some(x) = input.garden.get(Coordinate::new(c + 1, r)) {
                if *x != plant {
                    coordinates.push(Coordinate::new(c - 1, r));
                    continue 'outer;
                }
            } else {
                coordinates.push(Coordinate::new(c - 1, r));
            }

            if let Some(x) = input.garden.get(Coordinate::new(c, r + 1)) {
                if *x != plant {
                    coordinates.push(Coordinate::new(c, r - 1));
                    break 'outer;
                }
            } else {
                coordinates.push(Coordinate::new(c, r - 1));
            }
        }
    }

    coordinates = coordinates.into_iter().collect_vec();

    dbg!(&coordinates);
    dbg!(coordinates.len());
}

fn find_cols(input: &PuzzleInput) -> Vec<Coordinate> {
    let mut col_boundaries = vec![];

    for (r, row) in input.garden.row_iter().enumerate() {
        col_boundaries.push((r as i32, -1));

        let mut c = 0i32;
        for w in row.windows(2) {
            if w[0] != w[1] {
                col_boundaries.push((r as i32, c));
            }
            c += 1;
        }

        col_boundaries.push((r as i32, c));
    }

    col_boundaries
        .into_iter()
        .filter(|(_, c)| *c >= 0)
        .map(|(r, c)| Coordinate::new(c, r))
        .collect()
}

fn find_rows(input: &PuzzleInput) -> Vec<Coordinate> {

    let mut row_boundaries = vec![];

    for (c, col) in input.garden.col_iter().enumerate() {
        row_boundaries.push((-1i32, c as i32));

        let mut r = 0i32;
        for w in col.windows(2) {
            if w[0] != w[1] {
                row_boundaries.push((r + 1, c as i32));
            }
            r += 1;
        }

        row_boundaries.push((r as i32 + 1, c as i32));
    }

    row_boundaries
        .into_iter()
        .map(|(r, c)| Coordinate::new(c, r))
        .collect()
}

*/
#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
AAAA
BBCD
BBCC
EEEC
"};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO");
        assert_eq!(part1(&input), "140");
    }
}
