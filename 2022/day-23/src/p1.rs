use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut elves = input.cells.clone();
    let mut order = VecDeque::from([
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]);

    for _ in 0..10 {
        elves = step(&elves, &mut order);
        //debug_elves(&elves);
    }

    let bounding_box = compute_bounding_box(&elves);
    let area =
        (bounding_box.1.x - bounding_box.0.x + 1) * (bounding_box.1.y - bounding_box.0.y + 1);

    let free_tiles = area as usize - elves.len();

    free_tiles.to_string()
}

#[allow(unused)]
fn debug_elves(elves: &HashSet<Coordinate>) {
    let bounding_box = compute_bounding_box(elves);

    let mut grid = Grid2D::new(
        (bounding_box.1.x - bounding_box.0.x + 1) as usize,
        (bounding_box.1.y - bounding_box.0.y + 1) as usize,
        '.',
    );

    for elf in elves.iter() {
        grid.set(
            Coordinate::new(elf.x - bounding_box.0.x, elf.y - bounding_box.0.y),
            '#',
        );
    }

    println!("{}", grid);
}

pub fn step(elves: &HashSet<Coordinate>, order: &mut VecDeque<Direction>) -> HashSet<Coordinate> {
    let mut next = HashSet::new();
    let mut proposed: HashMap<Coordinate, Vec<Coordinate>> = HashMap::new();

    'elves: for elf in elves.iter() {
        if elf
            .moore_neighbors()
            .all(|neighbor| !elves.contains(&neighbor))
        {
            assert!(next.insert(*elf));
            continue;
        }

        for direction in order.iter() {
            let adj1 = *elf + *direction;
            let adj2 = *elf + direction.turn_left_45();
            let adj3 = *elf + direction.turn_right_45();

            if !elves.contains(&adj1) && !elves.contains(&adj2) && !elves.contains(&adj3) {
                proposed
                    .entry(adj1)
                    .and_modify(|x: &mut Vec<Coordinate>| x.push(*elf))
                    .or_insert(vec![*elf]);

                continue 'elves;
            }
        }

        assert!(next.insert(*elf));
    }

    for (adj, elves) in proposed.iter() {
        if elves.len() == 1 {
            next.insert(*adj);
        } else {
            next.extend(elves);
        }
    }

    order.rotate_left(1);

    assert_eq!(next.len(), elves.len());

    next
}

pub fn compute_bounding_box(elves: &HashSet<Coordinate>) -> (Coordinate, Coordinate) {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for elf in elves.iter() {
        min_x = min_x.min(elf.x);
        max_x = max_x.max(elf.x);
        min_y = min_y.min(elf.y);
        max_y = max_y.max(elf.y);
    }

    (Coordinate::new(min_x, min_y), Coordinate::new(max_x, max_y))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        ....#..
        ..###.#
        #...#.#
        .#...##
        #.###..
        ##.#.##
        .#..#..
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_eq!(input.cells.len(), 22);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "110");
    }
}
