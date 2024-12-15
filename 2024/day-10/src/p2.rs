use crate::parser::*;

pub fn part2(input: &PuzzleInput) -> String {
    let mut ratings = input
        .map
        .map(|&square| if square == 9 { 1 } else { 0usize });

    let mut terrain = vec![Vec::new(); 9];

    for (c, &square) in input.map.iter() {
        if square != 9 {
            terrain[square as usize].push(c);
        }
    }

    for height in (0..9).rev() {
        for c in terrain[height].iter() {
            let mut sum = 0;

            for neighbor in c.neighbors() {
                if let Some(&neighbor_height) = input.map.get(neighbor) {
                    if neighbor_height == height as u8 + 1 {
                        sum += ratings[neighbor];
                    }
                }
            }

            ratings[*c] = sum;
        }
    }

    let mut sum = 0;
    for c in terrain[0].iter() {
        sum += ratings[*c];
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = utility_belt::prelude::indoc! {"
        89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO");
        assert_eq!(part2(&input), "81");
    }
}
