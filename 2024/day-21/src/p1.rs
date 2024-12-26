use std::collections::{BTreeMap, BTreeSet};

use cached::proc_macro::cached;

use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut sum = 0;

    for code in input.codes.iter() {
        let solution = solve(code, 2);

        let num = parse_uints(code)[0];
        sum += num as usize * solution;
    }

    sum.to_string()
}

// https://www.youtube.com/watch?v=dqzAaj589cM
pub fn solve(code: &str, depth: usize) -> usize {
    let numpad = Keypad::new_numpad();
    let dirpad = Keypad::new_dirpad();

    // First, we find all possible paths between the buttons of the number pad.
    let candidates = numpad.shortest_paths(code);

    let mut best_length = usize::MAX;

    // Then we need to compute the length of the shortest sequence of movements
    // for each candidate. The shortest sequence is the one that minimizes the
    // sum of movement costs between each pair of buttons adjacent in the
    // sequence (starting with our initial position at the A button).
    for seq in candidates.iter() {
        let mut length = 0;

        for (a, b) in std::iter::once('A').chain(seq.chars()).tuple_windows() {
            length += compute_length(&dirpad, (a, b), depth);
        }

        best_length = best_length.min(length);
    }

    best_length
}

#[cached(key = "(char, char, usize)", convert = "{ (pair.0, pair.1, depth) }")]
fn compute_length(dirpad: &Keypad, pair: (char, char), depth: usize) -> usize {
    // We're at the last pad (the one we're controlling manually), so each button
    // press only costs 1 -- the cost of pressing the button itself.
    if depth == 0 {
        return 1;
    }

    let mut optimal = usize::MAX;

    // Otherwise we, again, need to check all possible paths by splitting them
    // up into pairs of buttons and solving for the length of the path from the
    // first button to the second one recursively, then sum up the costs and
    // find the cheapest one.
    for seq in dirpad.find_valid_paths(dirpad.keys[&pair.0], dirpad.keys[&pair.1]) {
        let mut length = 0;

        for (a, b) in std::iter::once('A').chain(seq.chars()).tuple_windows() {
            length += compute_length(dirpad, (a, b), depth - 1);
        }

        optimal = optimal.min(length);
    }

    optimal
}

#[derive(Clone, Debug)]
pub struct Keypad {
    pub keys: BTreeMap<char, Coordinate>,
    pub coordinates: BTreeSet<Coordinate>,
}

impl Keypad {
    pub fn new_numpad() -> Self {
        let mut numpad = Self {
            keys: BTreeMap::new(),
            coordinates: BTreeSet::new(),
        };

        numpad.keys.insert('7', (0, 0).into());
        numpad.keys.insert('8', (1, 0).into());
        numpad.keys.insert('9', (2, 0).into());
        numpad.keys.insert('4', (0, 1).into());
        numpad.keys.insert('5', (1, 1).into());
        numpad.keys.insert('6', (2, 1).into());
        numpad.keys.insert('1', (0, 2).into());
        numpad.keys.insert('2', (1, 2).into());
        numpad.keys.insert('3', (2, 2).into());
        numpad.keys.insert('0', (1, 3).into());
        numpad.keys.insert('A', (2, 3).into());

        for (_key, position) in numpad.keys.iter() {
            numpad.coordinates.insert(*position);
        }

        numpad
    }

    pub fn new_dirpad() -> Self {
        let mut dir_pad = Self {
            keys: BTreeMap::new(),
            coordinates: BTreeSet::new(),
        };

        dir_pad.keys.insert('^', (1, 0).into());
        dir_pad.keys.insert('A', (2, 0).into());
        dir_pad.keys.insert('<', (0, 1).into());
        dir_pad.keys.insert('v', (1, 1).into());
        dir_pad.keys.insert('>', (2, 1).into());

        for (_key, position) in dir_pad.keys.iter() {
            dir_pad.coordinates.insert(*position);
        }

        dir_pad
    }

    pub fn shortest_paths(&self, input: &str) -> Vec<String> {
        let mut options = Vec::new();

        // We start with all robots pointing at the A button, so we need to
        // start by moving from A to the first button in the input, which is
        // easily accomplished by prepending A to the input code.
        let input = std::iter::once('A').chain(input.chars());

        // Then we look at each pair of characters and record all possible paths
        // between them.
        for (a, b) in input.tuple_windows() {
            options.push(self.find_valid_paths(self.keys[&a], self.keys[&b]));
        }

        let mut result = Vec::new();

        // Then we look at all possible combinations of paths between the
        // buttons. Thankfully this doesn't blow up, but we *do* need to check
        // all combinations, otherwise we might miss the shortest path.
        //
        // I tried only looking at more-likely candidates (those with many
        // consecutive moves in the same direction), but that wasn't any
        // faster when benchmarking.
        for prod in options.iter().map(|v| v.iter()).multi_cartesian_product() {
            result.push(prod.into_iter().join(""));
        }

        result
    }

    fn find_valid_paths(&self, a: Coordinate, b: Coordinate) -> Vec<String> {
        let dx = b.x as isize - a.x as isize;
        let dy = b.y as isize - a.y as isize;

        // We need to find all valid paths between the two positions. We know we need to move
        // abs(dx) steps in the x direction and abs(dy) steps in the y direction, but we don't
        // know the order in which to do so, so we need to check all permutations.
        let mut moves = Vec::new();

        if dx > 0 {
            moves.extend(std::iter::repeat_n(Direction::Right, dx.abs_diff(0)));
        } else if dx < 0 {
            moves.extend(std::iter::repeat_n(Direction::Left, dx.abs_diff(0)));
        }

        if dy > 0 {
            moves.extend(std::iter::repeat_n(Direction::Down, dy.abs_diff(0)));
        } else if dy < 0 {
            moves.extend(std::iter::repeat_n(Direction::Up, dy.abs_diff(0)));
        }

        // Since we don't know which paths are valid (some will pass over the
        // forbidden blank spot), we can just check all of them. The following
        // closure simulates the path and checks if it's valid.
        //
        // Doing it this way is twice as fast as using a breadth-first search,
        // and it's also fewer lines of code.
        let valid_path = |path: &[Direction]| {
            let mut cur = a;

            for dir in path {
                cur = cur.neighbor(*dir);

                if !self.coordinates.contains(&cur) {
                    return false;
                }
            }

            true
        };

        let num_moves = moves.len();

        moves
            .into_iter()
            // Generate all permutations of the moves.
            .permutations(num_moves)
            // Only keep unique paths -- this is an approx. 4x speedup.
            .unique()
            // And check if they are valid
            .filter(|perm| valid_path(perm))
            // Convert the directions to characters
            .map(|perm| {
                perm.iter()
                    .map(|dir| ['^', '>', 'v', '<'][*dir as usize])
                    .collect::<String>()
            })
            // Don't forget to press the A button at the end of the sequence
            .map(|path| path + "A")
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
