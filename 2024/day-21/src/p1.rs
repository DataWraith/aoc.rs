use std::collections::BTreeMap;

use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let codepad = CodePad::new_codepad();
    let dirpad = CodePad::new_dirpad();

    let mut sum = 0;

    for code in input.codes.iter() {
        let solution = solve_codepad(code, &codepad, &dirpad);

        let num = parse_uints(&code)[0];
        sum += num as usize * solution.len();
    }

    sum.to_string()
}

fn solve_codepad(code: &str, codepad: &CodePad, dirpad: &CodePad) -> String {
    let codepad_solutions = codepad.solve(code);

    let mut min_length = usize::MAX;
    let mut best_solution = String::new();

    for codepad_solution in codepad_solutions {
        for dirpad_solution in solve_dirpad(codepad_solution, dirpad, 1) {
            if dirpad_solution.len() < min_length {
                min_length = dirpad_solution.len();
                best_solution = dirpad_solution;
            }
        }
    }

    best_solution
}

fn solve_dirpad(code: String, dirpad: &CodePad, depth: usize) -> Vec<String> {
    if depth == 0 {
        return dirpad.solve(&code);
    }

    let mut best_length = usize::MAX;
    let mut best_solution = String::new();

    for dirpad_solution in dirpad.solve(&code) {
        for rec_soln in solve_dirpad(dirpad_solution, dirpad, depth - 1) {
            if rec_soln.len() < best_length {
                best_length = rec_soln.len();
                best_solution = rec_soln;
            }
        }
    }

    vec![best_solution]
}

#[derive(Clone, Debug)]
pub struct CodePad {
    pub pad: Grid2D<char>,
    pub positions: BTreeMap<char, Coordinate>,
    pub sequences: BTreeMap<(char, char), Vec<String>>,
}

impl CodePad {
    // https://www.youtube.com/watch?v=dqzAaj589cM
    pub fn solve(&self, input: &str) -> Vec<String> {
        let mut options = Vec::new();

        // We start with all robots pointing at the A button, so we need to add that to the input --
        // otherwise we'll have no way to press A at the start.
        let input = std::iter::once('A').chain(input.chars());

        for (a, b) in input.tuple_windows() {
            let seq = self.sequences.get(&(a, b)).unwrap_or_else(|| {
                panic!("No sequence found for {:?}", (a, b));
            });

            options.push(seq);
        }

        let mut result = Vec::new();

        for prod in options.iter().map(|v| v.iter()).multi_cartesian_product() {
            let path = prod.into_iter().join("");
            result.push(path);
        }

        result
    }

    pub fn set(&mut self, position: Coordinate, value: char) {
        self.pad.set(position, value);
        self.positions.insert(value, position);
    }

    pub fn precalculate_movement_sequence(&mut self) {
        for x in 0..self.pad.width() {
            for y in 0..self.pad.height() {
                for x2 in 0..self.pad.width() {
                    for y2 in 0..self.pad.height() {
                        let position = Coordinate::new(x as i32, y as i32);
                        let position2 = Coordinate::new(x2 as i32, y2 as i32);

                        if !self.is_valid_position(position) || !self.is_valid_position(position2) {
                            continue;
                        }

                        let c1 = self.pad.get(position).unwrap();
                        let c2 = self.pad.get(position2).unwrap();

                        if c1 == c2 {
                            self.sequences.insert((*c1, *c2), vec!["A".to_string()]);
                            continue;
                        }

                        // BFS to find the shortest path
                        let mut q = VecDeque::new();
                        q.push_back((position, String::new()));
                        let mut best_len = usize::MAX;

                        while let Some((current, path)) = q.pop_front() {
                            if current == position2 {
                                best_len = path.len();

                                let mut path = path.clone();
                                path.push('A');

                                self.sequences
                                    .entry((*c1, *c2))
                                    .or_insert(vec![])
                                    .push(path);
                                continue;
                            }

                            if path.len() > best_len {
                                break;
                            }

                            for d in Direction::cardinal() {
                                let next = current.neighbor(d);
                                let mut next_path = path.clone();

                                match d {
                                    Direction::Up => next_path.push('^'),
                                    Direction::Down => next_path.push('v'),
                                    Direction::Left => next_path.push('<'),
                                    Direction::Right => next_path.push('>'),
                                    _ => unreachable!(),
                                }

                                if self.is_valid_position(next) {
                                    q.push_back((next, next_path));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn new_codepad() -> Self {
        let mut grid = Grid2D::new(3, 4, '.');

        let mut codepad = Self {
            pad: grid,
            positions: BTreeMap::new(),
            sequences: BTreeMap::new(),
        };

        codepad.set((0, 0).into(), '7');
        codepad.set((1, 0).into(), '8');
        codepad.set((2, 0).into(), '9');
        codepad.set((0, 1).into(), '4');
        codepad.set((1, 1).into(), '5');
        codepad.set((2, 1).into(), '6');
        codepad.set((0, 2).into(), '1');
        codepad.set((1, 2).into(), '2');
        codepad.set((2, 2).into(), '3');
        codepad.set((1, 3).into(), '0');
        codepad.set((2, 3).into(), 'A');

        codepad.precalculate_movement_sequence();

        codepad
    }

    fn new_dirpad() -> Self {
        let grid = Grid2D::new(3, 2, '.');

        let mut dir_pad = Self {
            pad: grid,
            positions: BTreeMap::new(),
            sequences: BTreeMap::new(),
        };

        dir_pad.set((1, 0).into(), '^');
        dir_pad.set((2, 0).into(), 'A');
        dir_pad.set((0, 1).into(), '<');
        dir_pad.set((1, 1).into(), 'v');
        dir_pad.set((2, 1).into(), '>');

        dir_pad.precalculate_movement_sequence();

        dir_pad
    }

    fn is_valid_position(&self, position: Coordinate) -> bool {
        self.pad.get(position).is_some() && self.pad.get(position).unwrap() != &'.'
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        029A
        980A
        179A
        456A
        379A
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "126384");
    }
}
