fn main() {
    let puzzle_input = day_16::parser::part1(include_str!("../../input.txt"));
    println!("Part 1: {}", day_16::p1::part1(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let puzzle_input = day_16::parser::part1(include_str!("../../input.txt"));
        //assert_eq!(day_16::p1::part1(&puzzle_input), "TODO");
    }
}
