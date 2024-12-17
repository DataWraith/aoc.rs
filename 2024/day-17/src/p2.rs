use utility_belt::prelude::*;

use crate::{p1::Machine, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let mut machine = Machine {
        a: input.register_a,
        b: input.register_b,
        c: input.register_c,
        program: input.program.clone(),
        instr_ptr: 0,
        output: vec![],
    };

    let mut sequence = vec![vec![]; input.program.len()];
    let mut prev = vec![0; input.program.len()];
    let mut found = vec![0];
    let mut move_by = 1;
    let target = 0;

    'outer: for i in (0..100_000).step_by(move_by) {
        let mut m = machine.clone();
        m.a = i;

        'middle: while let Some(_) = m.step() {
            for (j, target) in input.program.iter().enumerate() {
                if j >= m.output.len() {
                    continue 'middle;
                }

                if m.output[j] == *target {
                    sequence[j].push(i);
                    prev[j] = i;
                    continue 'outer;
                }

                if m.output[j] != *target {
                    continue 'outer;
                }
            }

            return i.to_string();
        }
    }

    let x = 0;
    dbg!(&sequence[x].first());
    sequence[x].dedup_by(|a, b| (*b - *a) == 0 || (*b - *a) == move_by as u64);
    dbg!(&sequence[x][0..100]);

    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,3,5,4,3,0
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "117440");
    }
}
