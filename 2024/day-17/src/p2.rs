use utility_belt::prelude::*;

use crate::{p1::Machine, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let mut q = VecDeque::new();
    q.push_front(0);

    loop {
        let a = q.pop_front().unwrap();

        'bits: for x in 0..8 {
            let register = (a << 3) + x;

            let mut machine = Machine {
                a: register,
                b: input.register_b,
                c: input.register_c,
                program: input.program.clone(),
                instr_ptr: 0,
                output: vec![],
            };

            while machine.step().is_some() {}

            for (a, b) in machine.output.iter().rev().zip(input.program.iter().rev()) {
                if a != b {
                    continue 'bits;
                }
            }

            if machine.output.len() == 16 {
                return register.to_string();
            }

            q.push_back(register);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

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
