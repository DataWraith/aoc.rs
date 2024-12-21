use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut sum = 0;

    for code in input.codes.iter() {
        let buttons = input_code(code.clone());
        let len = buttons.len();
        let num = parse_uints(code)[0];
        dbg!(num as usize, len);
        sum += num as usize * len;
    }

    sum.to_string()
}

fn input_code(code: String) -> String {
    let mut pads = vec![
        CodePad::new_dirpad(),
        CodePad::new_dirpad(),
        CodePad::new_codepad(),
    ];

    let mut chars = code.chars().collect::<Vec<char>>();
    let mut code: Vec<char> = vec![];

    'outer: for (depth, pad) in pads.iter_mut().enumerate().rev() {
        for c in chars.iter() {
            let (buttons_pressed, position) = pad.find_path(*c, depth);
            code.extend(buttons_pressed);
            code.push('A');
            pad.position = position;

            if code.iter().join("") == "v<<A^>>AvA^A" {
                //break 'outer;
            }
        }

        dbg!(&code.iter().join(""));
        chars = code.clone();
        code.clear();
    }

    //dbg!(&pads);

    chars.iter().join("")
}

#[derive(Clone, Debug)]
pub struct CodePad {
    pub pad: Grid2D<char>,
    pub position: Coordinate,
}

impl CodePad {
    fn new_codepad() -> Self {
        let mut grid = Grid2D::new(3, 4, '.');
        grid.set((0, 0).into(), '7');
        grid.set((1, 0).into(), '8');
        grid.set((2, 0).into(), '9');
        grid.set((0, 1).into(), '4');
        grid.set((1, 1).into(), '5');
        grid.set((2, 1).into(), '6');
        grid.set((0, 2).into(), '1');
        grid.set((1, 2).into(), '2');
        grid.set((2, 2).into(), '3');
        grid.set((1, 3).into(), '0');
        grid.set((2, 3).into(), 'A');

        let position = (2, 3).into();

        Self {
            pad: grid,
            position,
        }
    }

    fn new_dirpad() -> Self {
        let mut grid = Grid2D::new(3, 2, '.');
        grid.set((1, 0).into(), '^');
        grid.set((2, 0).into(), 'A');
        grid.set((0, 1).into(), '<');
        grid.set((1, 1).into(), 'v');
        grid.set((2, 1).into(), '>');

        Self {
            pad: grid,
            position: (2, 0).into(),
        }
    }

    fn is_valid_position(&self, position: Coordinate) -> bool {
        self.pad.get(position).is_some() && self.pad.get(position).unwrap() != &'.'
    }

    pub fn find_path(&self, c: char, depth: usize) -> (Vec<char>, Coordinate) {
        #[derive(Clone, Debug, Eq, PartialEq, Hash)]
        struct State {
            position: Coordinate,
            direction: Option<Direction>,
        }

        let pos = self.position;
        let start = State {
            position: pos,
            direction: None,
        };

        let mut successors = |state: &State| {
            let mut successors = vec![];

            for direction in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                let new_pos = state.position + direction;
                let cost = if state.direction.is_some() && state.direction.unwrap() == direction {
                    1
                } else {
                    2
                };

                if self.is_valid_position(new_pos) {
                    let new_state = State {
                        position: new_pos,
                        direction: Some(direction),
                    };

                    successors.push((new_state, cost));
                }
            }

            successors
        };

        let path = pathfinding::directed::dijkstra::dijkstra(&start, successors, |p| {
            self.pad.get(p.position) == Some(&c)
        });
        let mut buttons_pressed: Vec<char> = Vec::new();
        let new_position = path
            .clone()
            .unwrap()
            .clone()
            .0
            .last()
            .unwrap()
            .clone()
            .position;
        let mut last_direction: Option<Direction> = None;

        for w in path.unwrap().0.windows(2) {
            let direction = w[0].position.towards(w[1].position);

            match direction {
                Direction::Up => buttons_pressed.push('^'),
                Direction::Down => buttons_pressed.push('v'),
                Direction::Left => buttons_pressed.push('<'),
                Direction::Right => buttons_pressed.push('>'),
                _ => panic!("Invalid direction: {:?}", direction),
            }

            last_direction = Some(direction);
        }

        (buttons_pressed, new_position)
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
