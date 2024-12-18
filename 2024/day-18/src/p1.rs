use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut memory = Grid2D::new(input.width, input.height, '.');

    for c in input.bytes.iter().take(input.to_simulate) {
        memory.set(*c, '#');
    }

    let mut q = VecDeque::new();
    q.push_back((Coordinate::new(0, 0), 0));

    while let Some(c) = q.pop_front() {
        if c.0 == Coordinate::new(input.width as i32 - 1, input.height as i32 - 1) {
            return c.1.to_string();
        }

        for d in Direction::cardinal() {
            let nc = c.0 + d;
            if memory.get(nc) == Some(&'.') {
                memory.set(nc, 'o');
                q.push_back((nc, c.1 + 1));
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

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
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        let input = PuzzleInput {
            bytes: input.bytes,
            width: 7,
            height: 7,
            to_simulate: 12,
        };

        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "22");
    }
}
