use cached::proc_macro::cached;

use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let numpad = Keypad::new_numpad();
    let dirpad = Keypad::new_dirpad();

    let mut sum = 0;

    for code in input.codes.iter() {
        let solution = solve(code, &numpad, &dirpad, 2);

        let num = parse_uints(code)[0];
        sum += num as usize * solution;
    }

    sum.to_string()
}

// https://www.youtube.com/watch?v=dqzAaj589cM
pub fn solve(code: &str, numpad: &Keypad, dirpad: &Keypad, depth: usize) -> usize {
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
            length += compute_length(dirpad, (a, b), depth);
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
    for seq in dirpad.sequences.get(&pair).unwrap().iter() {
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
    pub pad: Grid2D<char>,
    pub sequences: HashMap<(char, char), Vec<String>>,
}

impl Keypad {
    pub fn new_numpad() -> Self {
        let grid = Grid2D::new(3, 4, '.');

        let mut numpad = Self {
            pad: grid,
            sequences: HashMap::new(),
        };

        numpad.pad.set((0, 0).into(), '7');
        numpad.pad.set((1, 0).into(), '8');
        numpad.pad.set((2, 0).into(), '9');
        numpad.pad.set((0, 1).into(), '4');
        numpad.pad.set((1, 1).into(), '5');
        numpad.pad.set((2, 1).into(), '6');
        numpad.pad.set((0, 2).into(), '1');
        numpad.pad.set((1, 2).into(), '2');
        numpad.pad.set((2, 2).into(), '3');
        numpad.pad.set((1, 3).into(), '0');
        numpad.pad.set((2, 3).into(), 'A');

        numpad.precalculate_movement_sequence();

        numpad
    }

    pub fn new_dirpad() -> Self {
        let grid = Grid2D::new(3, 2, '.');

        let mut dir_pad = Self {
            pad: grid,
            sequences: HashMap::new(),
        };

        dir_pad.pad.set((1, 0).into(), '^');
        dir_pad.pad.set((2, 0).into(), 'A');
        dir_pad.pad.set((0, 1).into(), '<');
        dir_pad.pad.set((1, 1).into(), 'v');
        dir_pad.pad.set((2, 1).into(), '>');

        dir_pad.precalculate_movement_sequence();

        dir_pad
    }

    pub fn shortest_paths(&self, input: &str) -> Vec<String> {
        let mut options = Vec::new();

        // We start with all robots pointing at the A button, so we need to
        // start by moving from A to the first button in the input, which is
        // easily accomplished by prepending A to the movement sequence.
        let input = std::iter::once('A').chain(input.chars());

        // Then we look at each pair of characters and record all possible paths
        // between them.
        for (a, b) in input.tuple_windows() {
            let seq = self.sequences.get(&(a, b)).unwrap_or_else(|| {
                panic!("No sequence found for {:?}", (a, b));
            });

            options.push(seq);
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
            let path = prod.into_iter().join("");
            result.push(path);
        }

        result
    }

    pub fn precalculate_movement_sequence(&mut self) {
        for x in 0..self.pad.width() {
            for y in 0..self.pad.height() {
                let position: Coordinate = (x as i32, y as i32).into();

                if !self.is_valid_position(position) {
                    continue;
                }

                for x2 in 0..self.pad.width() {
                    for y2 in 0..self.pad.height() {
                        let position2: Coordinate = (x2 as i32, y2 as i32).into();

                        if !self.is_valid_position(position2) {
                            continue;
                        }

                        let c1 = self.pad.get(position).unwrap();
                        let c2 = self.pad.get(position2).unwrap();

                        self.sequences
                            .insert((*c1, *c2), self.find_valid_paths(position, position2));
                    }
                }
            }
        }
    }

    fn find_valid_paths(&self, position: Coordinate, position2: Coordinate) -> Vec<String> {
        let dx = position2.x as isize - position.x as isize;
        let dy = position2.y as isize - position.y as isize;

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
            let mut cur = position;

            for dir in path {
                cur = cur.neighbor(*dir);

                if !self.is_valid_position(cur) {
                    return false;
                };
            }

            true
        };

        let num_moves = moves.len();

        moves
            .into_iter()
            // Generate all permutations of the moves.
            .permutations(num_moves)
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

    fn is_valid_position(&self, position: Coordinate) -> bool {
        self.pad.get(position).is_some() && self.pad.get(position).unwrap() != &'.'
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
