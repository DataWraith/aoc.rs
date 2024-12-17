use utility_belt::prelude::*;

use crate::{p1::Machine, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let mut stack = VecDeque::new();
    stack.push_front(0);

    for k in 0.. {
        let a = stack.pop_front().unwrap();

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

            println!("*********** {register} {x} ***********");

            while let Some(_) = machine.step() {}

            println!("{}", machine.output.iter().copied().join(", "));

            for (a, b) in machine.output.iter().rev().zip(input.program.iter().rev()) {
                if a != b {
                    continue 'bits;
                }
            }

            if machine.output.len() == 16 {
                return register.to_string();
            }

            println!("FOUND");
            stack.push_back(register);
        }
    }

    unreachable!();
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
