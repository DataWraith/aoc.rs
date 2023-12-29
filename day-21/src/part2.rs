use crate::{part1::walk, structs::*};

use gomez::{Domain, Problem, SolverDriver, System};
use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    //let polynomial = solve(input, (500..=500).collect::<Vec<_>>().as_slice());
    let polynomial = solve(input, &[65, 65 + 131, 65 + 2 * 131]);

    let a = polynomial[0];
    let b = polynomial[1];
    let c = polynomial[2];
    let x = 26501300.0;

    let result = a * x * x + b * x + c;

    dbg!(result);

    result.to_string()
}

pub fn solve(input: &PuzzleInput, num_steps: &[usize]) -> [f64; 3] {
    let mut dataset = Vec::new();

    for steps in num_steps {
        dbg!(steps);
        let result = walk(input, *steps, true);
        dataset.push((1 + *steps, result));
    }

    opt(dataset.clone())
}

pub fn opt(dataset: Vec<(usize, usize)>) -> [f64; 3] {
    let problem = ReachableTilesProblem { dataset };
    let mut solver = SolverDriver::builder(&problem).build();
    let mut x = [0.0; 3];

    while let Ok((cur_x, rx)) = solver.next() {
        dbg!(&cur_x);
        dbg!(&rx);

        x[0] = cur_x[0];
        x[1] = cur_x[1];
        x[2] = cur_x[2];
    }

    x
}

struct ReachableTilesProblem {
    dataset: Vec<(usize, usize)>,
}

impl Problem for ReachableTilesProblem {
    type Field = f64;

    fn domain(&self) -> Domain<Self::Field> {
        Domain::unconstrained(self.dataset.len() + 3)
    }
}

impl System for ReachableTilesProblem {
    fn eval<Sx, Srx>(
        &self,
        x: &gomez::nalgebra::Vector<Self::Field, gomez::nalgebra::Dyn, Sx>,
        rx: &mut gomez::nalgebra::Vector<Self::Field, gomez::nalgebra::Dyn, Srx>,
    ) where
        Sx: gomez::nalgebra::Storage<Self::Field, gomez::nalgebra::Dyn>
            + gomez::nalgebra::IsContiguous,
        Srx: gomez::nalgebra::StorageMut<Self::Field, gomez::nalgebra::Dyn>,
    {
        let a = x[0];
        let b = x[1];
        let c = x[2];

        //let a = 14688.0;
        //let b = 14750.0;
        //let c = 3699.0;

        for (i, (x_i, y_i)) in self.dataset.iter().enumerate() {
            let x_i = *x_i as f64;
            let y_i = *y_i as f64;

            rx[i] = (a * x_i * x_i + b * x_i + c - y_i);
        }

        rx[self.dataset.len()] = (a - a.round()).abs();
        rx[self.dataset.len() + 1] = (b - b.round()).abs();
        rx[self.dataset.len() + 2] = (c - c.round()).abs();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    "};

    const MINIGRID: &str = indoc! {"
        ...
        .S.
        ...
    "};

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(
            solve(&input, (1..100).collect_vec().as_slice()),
            [1.0, 1.0, 1.0]
        );
    }

    #[test]
    fn test_gomez() {
        let input = crate::parser::parse(MINIGRID);
        let mut dataset = Vec::new();

        for step in 1..=50 {
            dbg!(step);
            let tiles = walk(&input, step, true);
            dataset.push((1 + step, tiles));
        }

        dbg!(&dataset);

        opt(dataset);
    }
}
