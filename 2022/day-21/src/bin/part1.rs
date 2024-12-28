fn main() {
    let puzzle_input = day_21::parser::part1(include_str!("../../input.txt"));
    println!("Part 1: {}", day_21::p1::part1(&puzzle_input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let puzzle_input = day_21::parser::part1(include_str!("../../input.txt"));
        assert_eq!(day_21::p1::part1(&puzzle_input), "TODO");
    }
}
