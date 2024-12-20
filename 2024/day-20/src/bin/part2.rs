fn main() {
    let puzzle_input = day_20::parser::part2(include_str!("../../input.txt"));
    println!("Part 2: {}", day_20::p2::part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let puzzle_input = day_20::parser::part2(include_str!("../../input.txt"));
        // Too high
        // 1116799
        // 1116795
        // Tool low:
        // 985728
        assert_eq!(day_20::p2::part2(&puzzle_input), "TODO");
    }
}
