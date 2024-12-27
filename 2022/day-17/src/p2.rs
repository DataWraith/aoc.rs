use crate::{p1::Well, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let well = Well::new(input.rocks.clone(), input.jets.clone());

    let (cycle_length, mut well, start) =
        pathfinding::directed::cycle_detection::brent(well, |well| well.drop_rock());

    let start_height = well.max_height as usize;

    for _ in 0..cycle_length {
        well = well.drop_rock();
    }

    let end_height = well.max_height as usize;
    let difference = end_height - start_height;

    let cycles_to_run = (1000000000000 - start) / cycle_length;
    let cycle_height = cycles_to_run * difference;

    let preliminary = cycle_height + start_height - difference;

    let remaining = (1000000000000 - start - cycle_length) % cycle_length;
    let w = well.max_height as usize;

    for _ in 0..remaining {
        well = well.drop_rock();
    }

    let w2 = well.max_height as usize;

    (preliminary + w2 - w).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
       >>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
   "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "1514285714288");
    }
}
