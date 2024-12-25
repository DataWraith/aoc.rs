fn main() {
    let puzzle_input = day_24::parser::part2(include_str!("../../input.txt"));
    println!("Part 2: {}", day_24::p2::part2(&puzzle_input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part2() {
        let puzzle_input = day_24::parser::part2(include_str!("../../input.txt"));
        assert_eq!(day_24::p2::part2(&puzzle_input), "bpt,fkp,krj,mfm,ngr,z06,z11,z31");
    }
}
