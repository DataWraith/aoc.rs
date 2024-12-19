use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    assemble_pattern(input).to_string()
}

pub fn assemble_pattern(input: &PuzzleInput) -> usize {
    let mut c = 0;
    let mut current_patterns = Vec::new();
    let mut next_patterns = Vec::new();

    let mut remaining_designs = input.desired_designs.clone();

    while !remaining_designs.is_empty() {
        for p1 in input.patterns.iter() {
            'outer: for p2 in current_patterns.iter() {
                let prepend = format!("{}{}", p1, p2);
                let append = format!("{}{}", p2, p1);

                for (i, design) in remaining_designs.iter().enumerate() {
                    if design.contains(&prepend) {
                        next_patterns.push(prepend.clone());

                        if prepend.len() == design.len() {
                            c += 1;
                            remaining_designs.swap_remove(i);
                            continue 'outer;
                        }
                    }
                }

                for (i, design) in remaining_designs.iter().enumerate() {
                    if design.contains(&append) {
                        c += 1;
                        next_patterns.push(append.clone());
                        remaining_designs.swap_remove(i);
                        continue 'outer;
                    }
                }
            }
        }

        current_patterns = next_patterns;
        next_patterns = Vec::new();
    }

    c
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
"};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "6");
    }
}
