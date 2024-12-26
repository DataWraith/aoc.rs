use z3::{ast::Int, Context, SatResult};

use utility_belt::prelude::*;

use crate::parser::*;

fn manhattan_distance<'ctx>(
    ctx: &'ctx Context,
    x1: &'ctx Int,
    y1: &'ctx Int,
    x2: &'ctx Int,
    y2: &'ctx Int,
) -> Int<'ctx> {
    // Calculate |x1 - x2| + |y1 - y2|
    let diff_x = x1 - x2;
    let diff_y = y1 - y2;

    let b = diff_x.lt(&Int::from_i64(&ctx, 0));
    let x = b.ite(&diff_x.unary_minus(), &diff_x);

    let b = diff_y.lt(&Int::from_i64(&ctx, 0));
    let y = b.ite(&diff_y.unary_minus(), &diff_y);

    x + y
}

pub fn part2(input: &PuzzleInput) -> String {
    let (signal_x, signal_y) = {
        let cfg = z3::Config::new();
        let ctx = z3::Context::new(&cfg);
        let solver = z3::Solver::new(&ctx);

        let signal_x = Int::new_const(&ctx, "signal_x");
        let signal_y = Int::new_const(&ctx, "signal_y");

        // Bound the search space
        solver.assert(&signal_x.ge(&Int::from_i64(&ctx, 0)));
        solver.assert(&signal_x.le(&Int::from_i64(&ctx, 4000000)));
        solver.assert(&signal_y.ge(&Int::from_i64(&ctx, 0)));
        solver.assert(&signal_y.le(&Int::from_i64(&ctx, 4000000)));

        let sensor_xs = input
            .sensors
            .iter()
            .map(|s| Int::from_i64(&ctx, s.0.x as i64))
            .collect::<Vec<_>>();

        let sensor_ys = input
            .sensors
            .iter()
            .map(|s| Int::from_i64(&ctx, s.0.y as i64))
            .collect::<Vec<_>>();

        for i in 0..input.sensors.len() {
            let dist = manhattan_distance(&ctx, &signal_x, &signal_y, &sensor_xs[i], &sensor_ys[i]);

            let (sensor, beacon) = input.sensors[i];
            let sensor_range = sensor.manhattan_distance(beacon);
            let sensor_range = Int::from_i64(&ctx, sensor_range as i64);

            solver.assert(&dist.gt(&sensor_range));
        }

        let result = solver.check();

        if result == SatResult::Unsat {
            panic!("No solution found");
        }

        let signal_x = solver
            .get_model()
            .unwrap()
            .eval(&signal_x, true)
            .unwrap()
            .as_i64()
            .unwrap();

        let signal_y = solver
            .get_model()
            .unwrap()
            .eval(&signal_y, true)
            .unwrap()
            .as_i64()
            .unwrap();

        (signal_x as i32, signal_y as i32)
    };

    let coord = Coordinate::new(signal_x as i32, signal_y as i32);

    tuning_frequency(coord).to_string()
}

fn tuning_frequency(coord: Coordinate) -> u64 {
    coord.x as u64 * 4000000 + coord.y as u64
}
