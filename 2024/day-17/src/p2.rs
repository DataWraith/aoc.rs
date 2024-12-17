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

    'outer: for i in 0.. {
        dbg!(i);

        let mut m = machine.clone();
        m.a = i;

        'middle: while let Some(_) = m.step() {
            for (j, target) in input.program.iter().enumerate() {
                if j >= m.output.len() {
                    continue 'middle;
                }

                if m.output[j] != *target {
                    continue 'outer;
                }
            }

            return i.to_string();
        }
    }

    unreachable!()
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
