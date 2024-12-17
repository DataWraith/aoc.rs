use utility_belt::prelude::*;

use crate::{p1::Machine, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let a = 236539226447469u64;
        let mut machine = Machine {
            a: a,
            b: input.register_b,
            c: input.register_c,
            program: input.program.clone(),
            instr_ptr: 0,
            output: vec![],
        };

        println!("*********** {a} ***********");
        let mut count = 0;

        'outer: while let Some(_) = machine.step() {
            let mut xnput = String::new();
            std::io::stdin().read_line(&mut xnput).unwrap();

            for (i, o) in machine.output.iter().enumerate() {
                if o != &input.program[i] {
                    break 'outer;
                }

                if i == 15 && machine.a == 0 {
                    println!("SOLVED");
                }
            }
        }

    todo!()
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
