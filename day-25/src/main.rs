mod parser;
mod part1;
mod structs;

fn main() {
    let puzzle_input = parser::parse(include_str!("../input.txt"));
    println!("Part 1: {}", part1::part1(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let puzzle_input = parser::parse(include_str!("../input.txt"));
        assert_eq!(part1::part1(&puzzle_input), "612945");
    }
}
