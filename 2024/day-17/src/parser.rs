use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub register_a: u64,
    pub register_b: u64,
    pub register_c: u64,
    pub program: Vec<u64>,
}

pub fn part1(input: &str) -> PuzzleInput {
    let (registers, program) = input.split_once("\n\n").unwrap();
    let registers: Vec<_> = registers.lines().collect();

    let a = parse_uints(registers[0])[0];
    let b = parse_uints(registers[1])[0];
    let c = parse_uints(registers[2])[0];

    let program = parse_uints(program);

    PuzzleInput {
        register_a: a,
        register_b: b,
        register_c: c,
        program,
    }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        TODO
    "};

    #[test]
    fn test_parse() {
        assert_ne!(TEST_INPUT, "TODO");
        assert!(trace("Puzzle", winnow_parser).parse_next(&mut TEST_INPUT.clone()).is_ok());
    }
}
*/
