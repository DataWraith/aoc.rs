use utility_belt::prelude::*;

use crate::parser::*;

pub fn part1(input: &PuzzleInput) -> String {
    let mut octopi = input.octopi.clone();
    let mut flash_count = 0;

    for _ in 0..100 {
        flash_count += step(&mut octopi);
    }

    flash_count.to_string()
}

pub fn step(octopi: &mut Grid2D<u16>) -> usize {
    let mut flash_count = 0;
    let mut energy_levels = octopi.map(|&octopus| octopus + 1);

    let mut flash_queue: VecDeque<Coordinate> = energy_levels
        .iter()
        .filter(|&(_, octopus)| *octopus > 9)
        .map(|(c, _)| c)
        .collect();

    while let Some(coord) = flash_queue.pop_front() {
        if energy_levels[coord] < 10 {
            continue;
        }

        flash_count += 1;

        for neighbor in coord.moore_neighbors() {
            if !octopi.contains(neighbor) {
                continue;
            }

            energy_levels[neighbor] += 1;

            if energy_levels[neighbor] == 10 {
                flash_queue.push_back(neighbor);
            }
        }
    }

    *octopi = energy_levels.map(|&octopus| if octopus > 9 { 0 } else { octopus });

    flash_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn test_part1_example() {
        let input = parser::part1(parser::TEST_INPUT);
        assert_ne!(parser::TEST_INPUT.trim(), "TODO");
        assert_eq!(part1(&input), "1656");
    }
}
