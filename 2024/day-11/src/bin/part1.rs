fn main() {
    tracing_subscriber::fmt::init();

    let puzzle_input = day_11::parser::part1(include_str!("../../input.txt"));
    println!("Part 1: {}", day_11::p1::part1(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let puzzle_input = day_11::parser::part1(include_str!("../../input.txt"));
        assert_eq!(day_11::p1::part1(&puzzle_input), "TODO");
    }
}
