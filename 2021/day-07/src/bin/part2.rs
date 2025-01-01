fn main() {
    let puzzle_input = day_07::parser::part2(include_str!("../../input.txt"));
    println!("Part 2: {}", day_07::p2::part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part2() {
        let puzzle_input = day_07::parser::part2(include_str!("../../input.txt"));
        assert_eq!(day_07::p2::part2(&puzzle_input), "TODO");
    }
}
