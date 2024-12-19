use utility_belt::prelude::*;

use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut cache = HashMap::new();
    let mut c = 0;

    for design in input.desired_designs.iter() {
        c += count_possibilities(input, design, &mut cache);
    }

    c.to_string()
}

pub fn count_possibilities(
    input: &PuzzleInput,
    design: &str,
    cache: &mut HashMap<String, usize>,
) -> usize {
    if let Some(c) = cache.get(design) {
        return *c;
    }

    let mut count = 0;

    for pattern in input.patterns.iter() {
        if pattern == design {
            count += 1;
            continue;
        }

        if design.starts_with(pattern) {
            count += count_possibilities(input, &design[pattern.len()..], cache);
        }
    }

    cache.insert(design.to_string(), count);

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

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
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "16");
    }
}
