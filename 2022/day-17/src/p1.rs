use std::fmt::Display;

use utility_belt::prelude::*;

use crate::parser::*;

pub struct Well {
    lines: VecDeque<[bool; 7]>,
    max_height: i32,
}

impl Well {
    pub fn new() -> Self {
        let mut lines = VecDeque::new();
        lines.push_back([false; 7]);
        lines.push_back([false; 7]);
        lines.push_back([false; 7]);

        Self {
            lines,
            max_height: 0,
        }
    }

    pub fn get(&self, coordinate: Coordinate) -> bool {
        if coordinate.x < 0 || coordinate.x >= 7 {
            return true;
        }

        if self.lines.len() <= coordinate.y as usize {
            return false;
        }

        self.lines[coordinate.y as usize][coordinate.x as usize]
    }

    pub fn set(&mut self, coordinate: Coordinate) {
        while self.lines.len() <= coordinate.y as usize {
            self.lines.push_back([false; 7]);
        }

        self.lines[coordinate.y as usize][coordinate.x as usize] = true;
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
}

impl Display for Well {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.max_height + 1).rev() {
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

fn drop_rock(well: &mut Well, rock: &BoolGrid2D, mut jets: impl Iterator<Item = char>) {
    let mut coordinate = well.initial_coordinate();

    loop {
        let jet = jets.next().unwrap();

        if jet == '<' {
            if !well.collides(rock, coordinate.neighbor(Direction::Left)) {
                coordinate += Direction::Left;
            }
        } else if jet == '>' {
            if !well.collides(rock, coordinate.neighbor(Direction::Right)) {
                coordinate += Direction::Right;
            }
        } else {
            panic!("Invalid jet: {}", jet);
        }

        if well.collides(rock, coordinate.neighbor(Direction::Up)) {
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
}

pub fn part1(input: &PuzzleInput) -> String {
    let mut well = Well::new();
    let mut rocks = input.rocks.iter().cycle();
    let mut jets = input.jets.clone().into_iter().cycle();

    for i in 0..2022 {
        drop_rock(&mut well, rocks.next().unwrap(), &mut jets);
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
