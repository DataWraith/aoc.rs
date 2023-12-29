use crate::{
    part1::{find_start, walk},
    structs::*,
};



pub fn part2(input: &PuzzleInput) -> String {
    solve(input, 26501365).to_string()
}

pub fn solve(input: &PuzzleInput, num_steps: usize) -> usize {
    // First, notice that the grid polarity flips when we cross an edge.
    //
    // That means that we can work with a grid that is twice as large, and then
    // just ignore the polarity. So, in the actual puzzle input, we'd
    // conceptually be working with a (2 * 131) x (2 * 131) grid.
    //
    // If we assume a grid without obstacles, then the number of tiles we can
    // reach in x steps is simply (x + 1)². This is easy to see if we draw a
    // grid and count the tiles.
    //
    // This tells us that the relationship between the number of steps we take
    // and the number of reachable tiles is quadratic, so now we need to find
    // the actual polynomial of degree 2 that fits the data, and just hope that
    // there is one.
    //
    let size = input.grid.width(); // Assume a square input
    let start = find_start(input).x() as usize;

    // We need to take four samples, because we need to calculate the
    // second difference. We walk by (2 * size) steps, because we need
    // to account for the fact that the grid polarity flips when we
    // cross an edge.
    let x0 = walk(input, start);
    let x1 = walk(input, start + 2 * size);
    let x2 = walk(input, start + 4 * size);
    let x3 = walk(input, start + 6 * size);

    // Given four data points, we can subtract them from each other to get the
    // first and second differences. Since we assume a quadratic polynomial,
    // the second difference should be constant. If it isn't, then we know that
    // the data doesn't fit a quadratic polynomial. In that case we need to
    // try again later in the sequence, but I've taken too long to understand
    // this already, so I can't be bothered to make it work with the test input.
    // It works for the actual input.
    let first_diffs = [x1 - x0, x2 - x1, x3 - x2];
    let second_diffs = [
        first_diffs[1] - first_diffs[0],
        first_diffs[2] - first_diffs[1],
    ];
    assert_eq!(second_diffs[0], second_diffs[1]);

    // We can then use the second difference to find the coefficients of the
    // polynomial (nicely illustrated at
    // https://www.radfordmathematics.com/algebra/sequences-series/difference-method-sequences/quadratic-sequences.html):
    //
    //   2 * a  = sd[0]
    //   3a + b = fd[0]
    //   a + b + c = x0
    //
    // => a = sd[0] / 2
    //    b = fd[0] - 3a
    //    c = x0 - a - b

    let a = second_diffs[0] as i128 / 2;
    let b = first_diffs[0] as i128 - 3 * a;
    let c = x0 as i128 - a - b;

    // Now we need to account for the fact that we start at `start`, not at 1.
    let x = num_steps as i128;
    let x = x - start as i128;

    // Now we need to take into account that our polynomial is moving in steps of (2*size), not 1.
    let x = x / (2 * size as i128);

    // And we need to add 1, because a move of 0 steps will still reach a single tile;
    // recall that the formula for an obstacle-less plain was (x + 1)², not x², so that we
    // still reach a single tile when we take 0 steps.
    let x = x + 1;

    // Finally, we can evaluate the polynomial at x.
    (a * x * x + b * x + c) as usize
}
