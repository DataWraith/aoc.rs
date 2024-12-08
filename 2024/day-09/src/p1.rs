use utility_belt::prelude::*;

use crate::parser::*;

#[tracing::instrument(skip(input))]
pub fn part1(input: &PuzzleInput) -> String {
    let mut files = Vec::with_capacity(input.disk.len() * 10);
    let mut free = Vec::with_capacity(input.disk.len() * 10);

    input.disk.iter().step_by(2).for_each(|&d| {
        files.push(d as usize);
    });

    dbg!(&files);

    input.disk.iter().skip(1).step_by(2).for_each(|&d| {
        free.push(d as usize);
    });

    let mut gaps = Vec::new();

    let mut free_idx = 0;
    let mut file_idx = files.len() - 1;

    while free_idx < file_idx {
        let mut cur_free = free[free_idx];
        let mut cur_file = files[file_idx];

        if cur_free == cur_file {
            gaps.push(cur_file);
            free_idx += 1;
            file_idx -= 1;
            continue;
        }

        if cur_free > cur_file {
            gaps.push(cur_file);
            cur_free -= cur_file;
            file_idx -= 1;
            continue;
        }

        if cur_free < cur_file {
            gaps.push(cur_free);
            cur_file -= cur_free;
            free_idx += 1;
            continue;
        }
    }

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utility_belt::prelude::*;

    const TEST_INPUT: &str = indoc! {"
        2333133121414131402
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO");
        assert_eq!(part1(&input), "022111222");
    }
}
