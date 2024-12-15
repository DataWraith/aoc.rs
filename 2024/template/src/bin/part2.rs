fn main() {
    let puzzle_input = {{crate_name}}::parser::part2(include_str!("../../input.txt"));
    println!("Part 2: {}", {{crate_name}}::p2::part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let puzzle_input = {{crate_name}}::parser::part2(include_str!("../../input.txt"));
        assert_eq!({{crate_name}}::p2::part2(&puzzle_input), "TODO");
    }
}
