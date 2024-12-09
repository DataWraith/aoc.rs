use crate::{p1::make_disk, parser::*};

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let disk = make_disk(input);
    let mut contiguous = vec![];
    let mut current_run = vec![];

    for i in 0..(disk.len() - 1) {
        current_run.push(disk[i]);
        if disk[i] != disk[i + 1] {
            contiguous.push(current_run);
            current_run = vec![];
        }
    }

    current_run.push(disk[disk.len() - 1]);

    if !current_run.is_empty() {
        contiguous.push(current_run);
    }

    let mut result = vec![];

    loop {
        if contiguous.is_empty() {
            break;
        }

        let xs = contiguous.remove(0);

        if xs.is_empty() {
            continue;
        }

        if xs[0] != u64::MAX {
            result.push(xs);
            continue;
        }

        let mut found = false;

        for idx in (0..contiguous.len()).rev() {
            let ks = &contiguous[idx];

            if ks.is_empty() {
                continue;
            }

            if ks[0] == u64::MAX {
                continue;
            }

            if ks.len() == xs.len() {
                result.push(ks.clone());
                contiguous[idx] = vec![u64::MAX; ks.len()];
                found = true;
                break;
            }

            if ks.len() < xs.len() {
                let diff = xs.len() - ks.len();
                result.push(ks.clone());
                contiguous[idx] = vec![u64::MAX; ks.len()];
                contiguous.insert(0, vec![u64::MAX; diff]);
                found = true;
                break;
            }
        }

        if !found {
            result.push(xs);
        }
    }

    result
        .iter()
        .flatten()
        .enumerate()
        .fold(0, |acc, (i, x)| {
            if *x != u64::MAX {
                acc + i * *x as usize
            } else {
                acc
            }
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = utility_belt::prelude::indoc! {"
        2333133121414131402
    "};

    #[test]
    fn test_part2_example() {
        let input = crate::parser::part2(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO");
        assert_eq!(part2(&input), "2858");
    }
}
