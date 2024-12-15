fn main() {
    tracing_subscriber::fmt::init();

    let puzzle_input = day_12::parser::part2(include_str!("../../input.txt"));
    println!("Part 2: {}", day_12::p2::part2(&puzzle_input));
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_part2() {
        let puzzle_input = day_12::parser::part2(include_str!("../../input.txt"));
        // 847472
        // 839842
        assert_eq!(day_12::p2::part2(&puzzle_input), "844132");
    }
}
