mod parser;
mod part1;
mod part2;
mod structs;

fn main() {
    let puzzle_input = parser::parse(include_str!("../input.txt"));
    println!("Part 1: {}", part1::part1(&puzzle_input));
    println!("Part 2: {}", part2::part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let puzzle_input = parser::parse(include_str!("../input.txt"));
        //assert_eq!(part1::part1(&puzzle_input), "3637");
    }

    #[test]
    fn test_part2() {
        let puzzle_input = parser::parse(include_str!("../input.txt"));
        assert_eq!(part2::part2(&puzzle_input), "TODO");
    }
}
