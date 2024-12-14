fn main() {
    tracing_subscriber::fmt::init();

    let puzzle_input = day_14::parser::part2(include_str!("../../input.txt"));
    println!("Part 2: {}", day_14::p2::part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let puzzle_input = day_14::parser::part2(include_str!("../../input.txt"));
        assert_eq!(day_14::p2::part2(&puzzle_input), "TODO");
    }
}