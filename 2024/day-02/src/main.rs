use tracing_subscriber::fmt;
use tracing_subscriber::fmt::format::FmtSpan;

mod p1;
mod p2;
mod parser;
mod structs;

fn main() {
    fmt::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .with_target(false)
        .with_level(false)
        .init();

    let puzzle_input = parser::parse(include_str!("../input.txt"));

    #[cfg(feature = "p1")]
    println!("Part 1: {}", p1::part1(&puzzle_input));

    #[cfg(feature = "p2")]
    println!("Part 2: {}", p2::part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let puzzle_input = parser::parse(include_str!("../input.txt"));
        assert_eq!(p1::part1(&puzzle_input), "321");
    }

    #[test]
    fn test_part2() {
        let puzzle_input = parser::parse(include_str!("../input.txt"));
        assert_eq!(p2::part2(&puzzle_input), "386");
    }
}
