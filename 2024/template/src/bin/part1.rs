fn main() {
    let puzzle_input = {{crate_name}}::parser::part1(include_str!("../../input.txt"));
    println!("Part 1: {}", {{crate_name}}::p1::part1(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let puzzle_input = {{crate_name}}::parser::part1(include_str!("../../input.txt"));
        assert_eq!({{crate_name}}::p1::part1(&puzzle_input), "TODO");
    }
}
