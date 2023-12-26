use crate::{part1::part1, structs::*};

pub fn part2(input: &PuzzleInput) -> String {
    let numbers = input
        .numbers
        .iter()
        .map(|row| {
            let mut r = row.clone();
            r.reverse();
            r
        })
        .collect::<Vec<_>>();

    part1(&PuzzleInput { numbers })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&crate::parser::parse(
                "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45"
            )),
            "2"
        )
    }
}
