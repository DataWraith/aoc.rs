use utility_belt::prelude::*;

use crate::{p1::breadth_first_search, parser::*};

pub fn part2(input: &PuzzleInput) -> String {
    let idx = bisect(0, input.bytes.len(), |i| {
        let mut memory = Grid2D::new(input.width, input.height, '.');

        for b in input.bytes.iter().take(*i) {
            memory.set(*b, '#');
        }

        let cost = breadth_first_search(&memory);

        cost.is_none()
    })
    .unwrap();

    format!("{},{}", input.bytes[idx].x, input.bytes[idx].y)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
"};

    #[test]
    fn test_part2_example() {
        let mut input = crate::parser::part2(TEST_INPUT);
        input.width = 7;
        input.height = 7;
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part2(&input), "6,1");
    }
}
