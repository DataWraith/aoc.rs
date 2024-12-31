use utility_belt::prelude::*;

use crate::parser::*;

#[derive(Clone, Debug)]
pub struct Board {
    pub numbers: Grid2D<i64>,
    pub marked: BoolGrid2D,
}

impl Board {
    pub fn mark(&mut self, number: i64) -> bool {
        if let Some(coord) = self
            .numbers
            .iter()
            .find(|(_, x)| *x == &number)
            .map(|(coord, _)| coord)
        {
            self.marked.set(coord, true);
            return self.is_winner();
        }

        false
    }

    pub fn score(&self) -> i64 {
        self.numbers
            .iter()
            .filter(|(coord, _)| !self.marked[*coord])
            .map(|(_, x)| x)
            .sum::<i64>()
    }

    pub fn is_winner(&self) -> bool {
        for row in self.marked.row_iter() {
            if row.iter().all(|x| *x) {
                return true;
            }
        }

        for col in self.marked.col_iter() {
            if col.iter().all(|x| *x) {
                return true;
            }
        }

        false
    }
}

pub fn part1(input: &PuzzleInput) -> String {
    let mut state = input.clone();

    for number in state.numbers {
        for board in state.boards.iter_mut() {
            if board.mark(number) {
                return format!("{}", board.score() * number);
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "4512");
    }
}
