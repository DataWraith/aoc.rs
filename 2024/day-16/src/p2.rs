use std::iter::successors;
use utility_belt::prelude::*;

use crate::{
    p1::{follow_path, junctions, race, State},
    parser::*,
};

pub fn part2(input: &PuzzleInput) -> String {
    let mut tabu = HashSet::new();

    let mut best = race(input, &tabu);
    let mut coverage = dfs(input, best);

    coverage.to_string()
}

pub fn dfs(input: &PuzzleInput, best: State) -> usize {
    let junctions = junctions(&input.maze);

    let start = input.maze.iter().find(|(_, &c)| c == 'S').unwrap().0;
    let end = input.maze.iter().find(|(_, &c)| c == 'E').unwrap().0;

    let mut stack = VecDeque::new();
    let mut covered = input.maze.map(|_| false);
    //let mut visited = HashMap::new();

    stack.push_back(State {
        position: start,
        direction: Direction::Right,
        straight: 0,
        turn: 0,
        path: vec![start],
        heuristic: start.manhattan_distance(end) as usize,
    });

    while let Some(state) = stack.pop_back() {
        if state.score() == best.score() {
            dbg!(&state.turn, &state.straight);

            for path in state.path.windows(2) {
                let towards = path[0].towards(path[1]);
                covered.set(path[0], true);

                for c in successors(Some(path[0]), |&c| Some(c.neighbor(towards))) {
                    covered.set(c, true);
                    if c == path[1] {
                        break;
                    }
                }
            }
            continue;
        }

        if state.turn > best.turn {
            continue;
        }

        if state.straight > best.straight {
            continue;
        }

        //let cnt = visited
        //.entry((state.position, state.direction))
        //.or_insert(0);
        //*cnt += 1;

        //if *cnt > 1 {
        //continue;
        //}

        let (next, dir, len) =
            follow_path(state.position, state.direction, &input.maze, &junctions);

        let fwd_free = state.position != next;

        if fwd_free {
            let n = state.position.neighbor(state.direction);
            let mut path = state.path.clone();
            path.push(next);
            let mut fwd = State {
                position: next,
                straight: state.straight + len,
                heuristic: n.manhattan_distance(end) as usize,
                path,
                ..state.clone()
            };

            stack.push_back(fwd);
        }

        if junctions.contains(&state.position) || !fwd_free {
            let mut turn_left = State {
                direction: state.direction.turn_left_90(),
                turn: state.turn + 1,
                ..state.clone()
            };

            let n = turn_left.position.neighbor(turn_left.direction);
            if input.maze.get(n) == Some(&'.') || input.maze.get(n) == Some(&'S') {
                stack.push_back(turn_left);
            }

            let mut turn_right = State {
                direction: state.direction.turn_right_90(),
                turn: state.turn + 1,
                ..state.clone()
            };

            let n = turn_right.position.neighbor(turn_right.direction);
            if input.maze.get(n) == Some(&'.') || input.maze.get(n) == Some(&'S') {
                stack.push_back(turn_right);
            }
        }
    }

    println!(
        "{}",
        covered.map(|&c| if c { 'o' } else { '.' }).to_string()
    );

    return covered.iter().filter(|(_, &c)| c).count();
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "45");
    }
}
