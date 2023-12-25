use crate::{part1::split_grid, structs::*};

use utility_belt::prelude::HashSet;

pub fn part2(input: &PuzzleInput) -> String {
    let (symbols, numbers) = split_grid(&input.grid);

    let mut sum = 0;

    symbols.iter().for_each(|(coord, c)| {
        if *c == '*' {
            let mut nums = Vec::new();
            let mut found = HashSet::new();

            for n in coord.moore_neighbors() {
                if let Some((number, id)) = numbers.get(n) {
                    if *id != usize::MAX && !found.contains(id) {
                        found.insert(*id);
                        nums.push(*number);
                    }
                }
            }

            if nums.len() == 2 {
                sum += nums.iter().product::<usize>();
            }
        }
    });

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    "};

    #[test]
    fn test_part2() {
        let input = crate::parser::parse(TEST_INPUT);
        assert_eq!(part2(&input), "467835");
    }
}
