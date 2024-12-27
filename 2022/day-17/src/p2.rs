use crate::{p1::Well, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let well = Well::new(input.rocks.clone(), input.jets.clone());

    // Find the cycle start, cycle length, and the well state at the start of the first cycle
    let (cycle_length, mut well, start) =
        pathfinding::directed::cycle_detection::brent(well, |well| well.drop_rock());

    // Then, compute the height difference between the start and end of the cycle
    let start_height = well.max_height as usize;

    for _ in 0..cycle_length {
        well = well.drop_rock();
    }

    let end_height = well.max_height as usize;
    let difference = end_height - start_height;

    // Compute the number of cycles to run.
    //
    // We subtract `start` and the cycle length because we already ran `start`
    // steps in order to find the cycle, and then `cycle_length` steps in order
    // to find the height-difference of a cycle.
    let cycles_to_run = (1000000000000 - start - cycle_length) / cycle_length;

    // This gives us the additional height from the cycles
    let cycle_height = cycles_to_run * difference;

    // And of course, we need to add the start height back in
    let mut total_height = start_height + cycle_height;

    // Now we need to compute the remaining cycles to reach the 1000000000000-th
    // step, again subtracting the steps we already ran.
    let remaining = (1000000000000 - start - cycle_length) % cycle_length;

    // And actually run the remaining cycles and get the height difference
    let h_before = well.max_height as usize;

    for _ in 0..remaining {
        well = well.drop_rock();
    }

    let h_after = well.max_height as usize;

    // Add the height difference from the remaining steps to get our final answer
    total_height += h_after - h_before;

    total_height.to_string()
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
