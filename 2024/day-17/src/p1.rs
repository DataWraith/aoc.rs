use core::num;

use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let machine = run_machine(input);
    machine.output.iter().join(",").to_string()
}

pub fn run_machine(input: &PuzzleInput) -> Machine {
    let mut machine = Machine {
        a: input.register_a,
        b: input.register_b,
        c: input.register_c,
        program: input.program.clone(),
        instr_ptr: 0,
        output: vec![],
    };

    while let Some(_) = machine.step() {}

    machine
}

#[derive(Debug, Clone)]
pub struct Machine {
    pub a: u64,
    pub b: u64,
    pub c: u64,

    pub program: Vec<u64>,
    pub instr_ptr: usize,
    pub output: Vec<u64>,
}

impl Machine {
    pub fn step(&mut self) -> Option<()> {
        let instr: Instruction = self.read()?.into();

        match instr {
            Instruction::Adv => self.adv(),
            Instruction::Bxl => self.bxl(),
            Instruction::Bst => self.bst(),
            Instruction::Jnz => self.jnz(),
            Instruction::Bxc => self.bxc(),
            Instruction::Out => self.out(),
            Instruction::Bdv => self.bdv(),
            Instruction::Cdv => self.cdv(),
        };

        Some(())
    }

    pub fn read(&mut self) -> Option<u64> {
        if self.instr_ptr >= self.program.len() {
            return None;
        }

        let val = self.program[self.instr_ptr];
        self.instr_ptr += 1;
        Some(val)
    }

    pub fn adv(&mut self) -> Option<()> {
        let numerator = self.a as f64;
        let denominator = (1 << self.combo_operand()?) as f64;
        self.a = (numerator / denominator).trunc() as u64;
        Some(())
    }

    pub fn bxl(&mut self) -> Option<()> {
        let operand = self.literal_operand()?;
        self.b ^= operand;
        Some(())
    }

    pub fn bst(&mut self) -> Option<()> {
        let operand = self.combo_operand()?;
        self.b = operand % 8;
        Some(())
    }

    pub fn jnz(&mut self) -> Option<()> {
        let o = self.literal_operand()?;

        if self.a == 0 {
            return Some(());
        }

        self.instr_ptr = o as usize;
        Some(())
    }

    pub fn bxc(&mut self) -> Option<()> {
        let _ = self.literal_operand()?;
        let r = self.b ^ self.c;
        self.b = r;
        Some(())
    }

    pub fn out(&mut self) -> Option<()> {
        let o = self.combo_operand()? % 8;
        self.output.push(o);
        Some(())
    }

    pub fn bdv(&mut self) -> Option<()> {
        let numerator = self.a as f64;
        let denominator = (1 << self.combo_operand()?) as f64;
        self.b = (numerator / denominator).trunc() as u64;
        Some(())
    }

    pub fn cdv(&mut self) -> Option<()> {
        let numerator = self.a as f64;
        let denominator = (1 << self.combo_operand()?) as f64;
        self.c = (numerator / denominator).trunc() as u64;
        Some(())
    }

    pub fn literal_operand(&mut self) -> Option<u64> {
        let x = *self.program.get(self.instr_ptr)?;
        self.instr_ptr += 1;
        Some(x)
    }

    pub fn combo_operand(&mut self) -> Option<u64> {
        let x = self.program.get(self.instr_ptr)?;
        self.instr_ptr += 1;

        match x {
            0..=3 => return Some(*x),
            4 => return Some(self.a),
            5 => return Some(self.b),
            6 => return Some(self.c),
            7 => unreachable!("Reserved operand"),
            _ => panic!("Invalid operand"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u64> for Instruction {
    fn from(value: u64) -> Self {
        match value {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!("Invalid instruction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "4,6,3,5,6,3,5,2,1,0");
    }

    const TEST_INPUT2: &str = indoc! {"
        Register A: 10
        Register B: 0
        Register C: 0

        Program: 5,0,5,1,5,4
    "};

    #[test]
    fn test_part1_example_2() {
        let input = crate::parser::part1(TEST_INPUT2);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "0,1,2");
    }

    const TEST_INPUT3: &str = indoc! {"
        Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    "};

    #[test]
    fn test_part1_example_3() {
        let input = crate::parser::part1(TEST_INPUT3);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "4,2,5,6,7,7,7,7,3,1,0");
    }

    const TEST_INPUT4: &str = indoc! {"
        Register A: 0
        Register B: 29
        Register C: 0

        Program: 1,7
    "};

    #[test]
    fn test_part1_example_4() {
        let input = crate::parser::part1(TEST_INPUT4);
        assert_ne!(TEST_INPUT, "TODO\n");
        let m = run_machine(&input);
        assert_eq!(m.b, 26);
    }

    const TEST_INPUT5: &str = indoc! {"
        Register A: 0
        Register B: 0
        Register C: 9

        Program: 2,6
    "};

    #[test]
    fn test_part1_example_5() {
        let input = crate::parser::part1(TEST_INPUT5);
        assert_ne!(TEST_INPUT, "TODO\n");
        let m = run_machine(&input);
        assert_eq!(m.b, 1);
    }

    const TEST_INPUT6: &str = indoc! {"
        Register A: 0
        Register B: 2024
        Register C: 43690

        Program: 4,0
    "};

    #[test]
    fn test_part1_example_6() {
        let input = crate::parser::part1(TEST_INPUT6);
        assert_ne!(TEST_INPUT, "TODO\n");
        let m = run_machine(&input);
        assert_eq!(m.b, 44354);
    }
}
