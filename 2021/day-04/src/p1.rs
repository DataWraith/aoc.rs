use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut state = input.clone();

    for number in state.numbers {
        for (i, board) in state.boards.iter().enumerate() {
            if let Some(coord) = board
                .iter()
                .find(|(_, x)| *x == &number)
                .map(|(coord, _)| coord)
            {
                state.marked[i].set(coord, true);

                for row in state.marked[i].row_iter() {
                    if row.iter().all(|x| *x) {
                        let unmarked_sum = board
                            .iter()
                            .filter(|(coord, _)| !state.marked[i][*coord])
                            .map(|(_, x)| x)
                            .sum::<i64>();

                        return format!("{}", unmarked_sum * number);
                    }
                }

                for col in state.marked[i].col_iter() {
                    if col.iter().all(|x| *x) {
                        let unmarked_sum = board
                            .iter()
                            .filter(|(coord, _)| !state.marked[i][*coord])
                            .map(|(_, x)| x)
                            .sum::<i64>();

                        return format!("{}", unmarked_sum * number);
                    }
                }
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

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
