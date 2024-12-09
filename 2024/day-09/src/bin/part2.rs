fn main() {
    tracing_subscriber::fmt::init();

    let puzzle_input = day_09::parser::part2(include_str!("../../input.txt"));
    println!("Part 2: {}", day_09::p2::part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    

    #[test]
    fn test_part1() {
        let puzzle_input = day_09::parser::part2(include_str!("../../input.txt"));
        assert_eq!(day_09::p2::part2(&puzzle_input), "6488291456470");
    }
}
