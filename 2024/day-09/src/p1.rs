use crate::parser::*;

pub fn make_disk(input: &PuzzleInput) -> Vec<u64> {
    let mut disk: Vec<u64> = vec![];

    for (i, &d) in input.disk.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..d {
                disk.push(i as u64 / 2);
            }
        } else {
            for _ in 0..d {
                disk.push(u64::MAX);
            }
        }
    }

    disk
}

pub fn part1(input: &PuzzleInput) -> String {
    let disk = make_disk(input);

    let mut fwd = 0;
    let mut bwd = disk.len() - 1;
    let mut defrag = Vec::with_capacity(disk.len());

    while fwd <= bwd {
        if disk[fwd] == u64::MAX {
            defrag.push(disk[bwd]);
            fwd += 1;

            loop {
                bwd -= 1;

                if disk[bwd] != u64::MAX {
                    break;
                }
            }
        } else {
            defrag.push(disk[fwd]);
            fwd += 1;
        }
    }

    checksum(&defrag).to_string()
}

fn checksum(defrag: &[u64]) -> u64 {
    defrag.iter().enumerate().map(|(i, &d)| d * i as u64).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = utility_belt::prelude::indoc! {"
        2333133121414131402
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO");
        assert_eq!(part1(&input), "1928");
    }
}
