use crate::structs::*;

pub fn part1(input: &PuzzleInput) -> String {
    input
        .sequence
        .iter()
        .map(|s| hash(&s.to_string()) as usize)
        .sum::<usize>()
        .to_string()
}

pub fn hash(s: &str) -> u8 {
    let mut cur = 0u8;

    for char in s.chars() {
        cur = cur.wrapping_add(char as u8);
        cur = cur.wrapping_mul(17);
    }

    cur
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::rstest::*;
    use utility_belt::prelude::*;

    #[rstest]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn test_hash(#[case] s: String, #[case] expected: u8) {
        assert_eq!(hash(&s), expected);
    }
}
