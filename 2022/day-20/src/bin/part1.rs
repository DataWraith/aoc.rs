fn main() {
    let puzzle_input = day_20::parser::part1(include_str!("../../input.txt"));
    println!("Part 1: {}", day_20::p1::part1(&puzzle_input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let puzzle_input = day_20::parser::part1(include_str!("../../input.txt"));
        // -284 is wrong
        // 16569 is too high
        assert_eq!(day_20::p1::part1(&puzzle_input), "TODO");
    }
}
