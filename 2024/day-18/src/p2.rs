use utility_belt::prelude::*;

use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    // This solution uses UnionFind again. I thought of trying this myself, but
    // concluded that it wouldn't work -- and it doesn't in the forward
    // direction. But I later found out on Reddit that you can just reverse
    // everything, and it works marvelously.

    // Step 1: Find all obstacle coordinates and make a map
    let mut grid = Grid2D::new(input.width, input.height, '.');

    for b in input.bytes.iter() {
        grid.set(*b, '#');
    }

    // Step 2: Prepare the UnionFind datastructure
    let mut union_find = UnionFind::default();
    let mut sets = grid.map(|_| 0);

    for (c, _) in grid.iter() {
        sets[c] = union_find.make_set();
    }

    // Step 3: Union all coordinates that never have obstacles
    for (c, _) in grid.iter() {
        if grid.get(c) != Some(&'.') {
            continue;
        }

        for dir in [Direction::Right, Direction::Down] {
            let n = c.neighbor(dir);

            if grid.get(n) == Some(&'.') {
                union_find.union(sets[c], sets[n]).unwrap();
            }
        }
    }

    // Step 4: Iterate over the obstacles in reverse, and union the coordinates.
    // This is the non-obvious part. Instead of adding obstacles one by one,
    // we start with all obstacles and then remove them one by one, unioning
    // the free space.
    let start = Coordinate::new(0, 0);
    let end = Coordinate::new(input.width as i32 - 1, input.height as i32 - 1);

    for b in input.bytes.iter().rev() {
        grid.set(*b, '.');

        for n in b.neighbors() {
            if grid.get(n) == Some(&'.') {
                union_find.union(sets[*b], sets[n]).unwrap();
            }
        }

        // Step 5: If the start and end coordinates are connected, the last
        // obstacle we removed must have been the one blocking the path.
        if union_find.find(sets[start]) == union_find.find(sets[end]) {
            return format!("{},{}", b.x, b.y);
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"};

    #[test]
    fn test_part2_example() {
        let mut input = crate::parser::part2(TEST_INPUT);
        input.width = 7;
        input.height = 7;
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "6,1");
    }
}
