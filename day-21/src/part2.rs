use crate::{part1::walk, structs::*};

use gomez::{Domain, Problem, SolverDriver, System};
use utility_belt::prelude::*;

pub fn part2(input: &PuzzleInput) -> String {
    solve(
        input,
        ((65..(65 + 131 * 15)).step_by(131))
            .collect::<Vec<_>>()
            .as_slice(),
    );
    todo!();
}

pub fn solve(input: &PuzzleInput, num_steps: &[usize]) -> usize {
    let mut dataset = Vec::new();

    for steps in num_steps {
        dbg!(steps);
        let result = walk(input, *steps, true);
        dataset.push((*steps, result));
    }

    opt(dataset.clone());

    dbg!(&dataset);

    todo!();
}

pub fn opt(dataset: Vec<(usize, usize)>) {
    let problem = ReachableTilesProblem { dataset };
    let mut solver = SolverDriver::builder(&problem).build();

    let (x, norm) = solver
        .find(|s| {
            dbg!(s.x());
            dbg!(s.rx());
            dbg!(s.norm()) < 1e-6
        })
        .unwrap();

    dbg!(x, norm);
}

struct ReachableTilesProblem {
    dataset: Vec<(usize, usize)>,
}

impl Problem for ReachableTilesProblem {
    type Field = f64;

    fn domain(&self) -> Domain<Self::Field> {
        Domain::unconstrained(self.dataset.len())
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
            let x_i = *x_i as f64 / 131.0;
            let y_i = *y_i as f64;

            rx[i] = (a * x_i * x_i + b * x_i + c - y_i)
        }
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

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(solve(&input, &[5, 5 + 11, 5 + 22]), usize::MAX);
    }

    #[test]
    fn test_gomez() {
        let dataset = vec![
            (6, 16),
            (10, 50),
            (50, 1594),
            (100, 6536),
            (500, 167004),
            (1000, 668697),
        ];

        opt(dataset)
    }
}
