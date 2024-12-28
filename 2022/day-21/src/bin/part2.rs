fn main() {
    let puzzle_input = day_21::parser::part2(include_str!("../../input.txt"));
    println!("Part 2: {}", day_21::p2::part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part2() {
        let puzzle_input = day_21::parser::part2(include_str!("../../input.txt"));

        // -3887609737915 is wrong
        assert_eq!(day_21::p2::part2(&puzzle_input), "3887609741189");
    }
}
