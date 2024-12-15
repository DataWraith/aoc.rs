mod p1;
mod p2;
mod parser;
mod structs;

fn main() {
    #[cfg(feature = "p1")]
    {
        let puzzle_input = parser::parse(include_str!("../input.txt"));
        println!("Part 1: {}", p1::part1(&puzzle_input));
    }

    #[cfg(feature = "p2")]
    {
        let puzzle_input = parser::parse2(include_str!("../input.txt"));
        println!("Part 2: {}", p2::part2(&puzzle_input));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let puzzle_input = parser::parse(include_str!("../input.txt"));
        assert_eq!(p1::part1(&puzzle_input), "173731097");
    }

    #[test]
    fn test_part2() {
        let puzzle_input = parser::parse2(include_str!("../input.txt"));
        assert_eq!(p2::part2(&puzzle_input), "93729253");
    }
}
