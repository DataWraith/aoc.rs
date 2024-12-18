use utility_belt::prelude::*;

use crate::parser::*;

pub fn breadth_first_search(memory: &Grid2D<char>) -> Option<usize> {
    let mut memory = memory.clone();

    let mut q = VecDeque::new();
    q.push_back((Coordinate::new(0, 0), 0));

    while let Some((c, cost)) = q.pop_front() {
        if c == Coordinate::new(memory.width as i32 - 1, memory.height as i32 - 1) {
            return Some(cost);
        }

        for nc in c.neighbors() {
            if memory.get(nc) == Some(&'.') {
                memory.set(nc, 'o');
                q.push_back((nc, cost + 1));
            }
        }
    }

    None
}

pub fn part1(input: &PuzzleInput) -> String {
    let mut memory = Grid2D::new(input.width, input.height, '.');

    // Mark the fallen bytes
    for c in input.bytes.iter().take(input.to_simulate) {
        memory.set(*c, '#');
    }

    // Find the shortest path to the bottom right
    let cost = breadth_first_search(&memory);

    cost.unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::indoc;

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
