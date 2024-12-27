use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use utility_belt::prelude::*;

use crate::parser::*;

#[derive(Clone, Debug)]
pub struct Well {
    pub lines: VecDeque<[bool; 7]>,
    pub rocks: Rc<Vec<BoolGrid2D>>,
    pub jets: Rc<Vec<char>>,
    pub rock_idx: usize,
    pub jet_idx: usize,
    pub base_height: i32,
    pub max_height: i32,
}

impl PartialEq for Well {
    fn eq(&self, other: &Self) -> bool {
        self.lines == other.lines
            && self.rock_idx == other.rock_idx
            && self.jet_idx == other.jet_idx
    }
}

impl Eq for Well {}

impl Hash for Well {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lines.hash(state);
        self.rock_idx.hash(state);
        self.jet_idx.hash(state);
    }
}

impl Well {
    pub fn new(rocks: Vec<BoolGrid2D>, jets: Vec<char>) -> Self {
        let mut lines = VecDeque::new();
        lines.push_back([false; 7]);
        lines.push_back([false; 7]);
        lines.push_back([false; 7]);

        Self {
            lines,
            rocks: Rc::new(rocks),
            jets: Rc::new(jets),
            rock_idx: 0,
            jet_idx: 0,
            max_height: 0,
            base_height: 0,
        }
    }

    pub fn get(&self, coordinate: Coordinate) -> bool {
        if coordinate.x < 0 || coordinate.x >= 7 {
            return true;
        }

        if coordinate.y < self.base_height {
            return true;
        }

        let y = coordinate.y - self.base_height;

        if self.lines.len() <= y as usize {
            return false;
        }

        self.lines[y as usize][coordinate.x as usize]
    }

    pub fn shrink(&mut self) {
        let start_y = self.lines.len().max(3) - 3;
        let end_y = self.lines.len() - 1;

        for y in (start_y..=end_y).rev() {
            let mut count = 0;

            for x in 0..7 {
                if self.lines[y][x] {
                    count += 1;
                }
            }

            if count >= 7 {
                self.base_height += y as i32;

                for _ in 0..y {
                    self.lines.pop_front();
                }

                break;
            }
        }
    }

    pub fn set(&mut self, coordinate: Coordinate) {
        let y = coordinate.y - self.base_height;

        while self.lines.len() <= y as usize {
            self.lines.push_back([false; 7]);
        }

        self.lines[y as usize][coordinate.x as usize] = true;
        self.max_height = self.max_height.max(coordinate.y);
    }

    pub fn initial_coordinate(&self) -> Coordinate {
        Coordinate::new(2, self.max_height + 4)
    }

    pub fn collides(&self, rock: &BoolGrid2D, coordinate: Coordinate) -> bool {
        if coordinate.y <= 0 {
            return true;
        }

        rock.iter().any(|(coord, bool)| {
            if *bool {
                self.get(coord + coordinate)
            } else {
                false
            }
        })
    }

    pub fn drop_rock(&self) -> Self {
        let mut well = self.clone();

        let mut coordinate = well.initial_coordinate();
        let rock = well.rocks[well.rock_idx].clone();
        well.rock_idx = (well.rock_idx + 1) % well.rocks.len();

        loop {
            let jet = well.jets[well.jet_idx];
            well.jet_idx = (well.jet_idx + 1) % well.jets.len();

            if jet == '<' {
                if !well.collides(&rock, coordinate.neighbor(Direction::Left)) {
                    coordinate += Direction::Left;
                }
            } else if jet == '>' {
                if !well.collides(&rock, coordinate.neighbor(Direction::Right)) {
                    coordinate += Direction::Right;
                }
            } else {
                panic!("Invalid jet: {}", jet);
            }

            if well.collides(&rock, coordinate.neighbor(Direction::Up)) {
                rock.iter().for_each(|(coord, bool)| {
                    let x = coord.x + coordinate.x;
                    let y = coord.y + coordinate.y;
                    let c = Coordinate::new(x, y);

                    if *bool {
                        well.set(c);
                    }
                });
                break;
            }

            coordinate = coordinate.neighbor(Direction::Up);
        }

        well.shrink();
        well
    }
}

impl Display for Well {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (self.base_height..self.max_height + 1).rev() {
            for x in 0..7 {
                write!(
                    f,
                    "{}",
                    if self.get(Coordinate::new(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part1(input: &PuzzleInput) -> String {
    let mut well = Well::new(input.rocks.clone().into(), input.jets.clone().into());

    for _ in 0..2022 {
        well = well.drop_rock();
    }

    return well.max_height.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        >>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "3068");
    }
}
