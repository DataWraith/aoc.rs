use pathfinding;
use utility_belt::prelude::*;

use crate::parser::*;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct State {
    position: Coordinate,
    time: usize,
}

pub fn part1(input: &PuzzleInput) -> String {
    let (cycle_length, _, cycle_start) =
        pathfinding::directed::cycle_detection::brent(make_blizzard_grid(input), |grid| {
            blizzard_step(&grid)
        });

    assert_eq!(cycle_start, 0);

    let blizzard_grids = std::iter::successors(Some(make_blizzard_grid(input)), |grid| {
        Some(blizzard_step(grid))
    })
    .take(cycle_length)
    .map(|grid| grid.map(|ds| !ds.is_empty()))
    .map(|grid| grid.into())
    .collect::<Vec<BoolGrid2D>>();

    let start = State {
        position: start_coord(input),
        time: 1,
    };

    let end = end_coord(input);

    let successors = |state: &State| {
        let blizzard_grid = &blizzard_grids[state.time % blizzard_grids.len()];

        let mut successors = Vec::new();

        for n in state.position.neighbors() {
            if n != end
                && (n.x == 0
                    || n.x == input.grid.width() as i32 - 1
                    || n.y <= 0 // Can walk off the top edge...
                    || n.y == input.grid.height() as i32 - 1)
            {
                continue;
            }

            if blizzard_grid[n.into()] {
                continue;
            }

            successors.push((
                State {
                    position: n,
                    time: state.time + 1,
                },
                1,
            ));
        }

        if !blizzard_grid[state.position.into()] {
            successors.push((
                State {
                    position: state.position,
                    time: state.time + 1,
                },
                1,
            ));
        }

        successors
    };

    let path = pathfinding::directed::dijkstra::dijkstra(&start, successors, |state| {
        state.position == end
    });

    (path.unwrap().1).to_string()
}

fn start_coord(input: &PuzzleInput) -> Coordinate {
    for x in 0..input.grid.width() {
        if input.grid.get((x as i32, 0i32).into()).unwrap() == &'.' {
            return Coordinate::new(x as i32, 0);
        }
    }

    unreachable!()
}

fn end_coord(input: &PuzzleInput) -> Coordinate {
    for x in (0..input.grid.width()).rev() {
        if input
            .grid
            .get((x as i32, input.grid.height() as i32 - 1).into())
            .unwrap()
            == &'.'
        {
            return Coordinate::new(x as i32, input.grid.height() as i32 - 1);
        }
    }

    unreachable!()
}

pub fn blizzard_step(grid: &Grid2D<DirectionSet>) -> Grid2D<DirectionSet> {
    let mut new_grid = Grid2D::new(grid.width(), grid.height(), DirectionSet::empty());

    for y in 1..(grid.height() - 1) {
        for x in 1..(grid.width() - 1) {
            let cell = grid[(x as i32, y as i32).into()];

            for direction in cell.iter() {
                let mut new_coord = Coordinate::new(x as i32, y as i32) + direction;

                if new_coord.x == 0
                    || new_coord.x == grid.width() as i32 - 1
                    || new_coord.y == 0
                    || new_coord.y == grid.height() as i32 - 1
                {
                    new_coord = new_coord + 2 * direction;
                    new_coord %= grid.dims();
                }

                new_grid[new_coord.into()].insert(direction);
            }
        }
    }

    new_grid
}

pub fn make_blizzard_grid(input: &PuzzleInput) -> Grid2D<DirectionSet> {
    let mut grid = Grid2D::new(
        input.grid.width(),
        input.grid.height(),
        DirectionSet::empty(),
    );

    for y in 0..input.grid.height() {
        for x in 0..input.grid.width() {
            let cell = input.grid.get((x as i32, y as i32).into()).unwrap();

            match cell {
                '^' => grid[(x as i32, y as i32).into()] = Direction::Up.into(),
                'v' => grid[(x as i32, y as i32).into()] = Direction::Down.into(),
                '<' => grid[(x as i32, y as i32).into()] = Direction::Left.into(),
                '>' => grid[(x as i32, y as i32).into()] = Direction::Right.into(),
                _ => {}
            }
        }
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "18");
    }
}
