use std::{cmp::Reverse, collections::BinaryHeap};

use crate::parser::*;

pub type FileId = u64;

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: u64,
    pub size: u64,
}

pub fn part2(input: &PuzzleInput) -> String {
    let mut files: Vec<Span> = vec![];
    let mut blanks = vec![BinaryHeap::new(); 10];

    let mut cur = 0;

    for (i, &d) in input.disk.iter().enumerate() {
        if i % 2 == 0 {
            files.push(Span {
                start: cur,
                size: d,
            });
        } else {
            blanks[d as usize].push(Reverse(cur));
        }

        cur += d;
    }

    for cur_file in files.iter_mut().rev() {
        let mut blank_pos = cur_file.start;
        let mut blank_size = usize::MAX;

        // Find the earliest blank that can fit the file
        for size in (cur_file.size as usize)..10 {
            if let Some(Reverse(blank_start)) = blanks[size].peek() {
                if *blank_start < blank_pos {
                    blank_pos = *blank_start;
                    blank_size = size;
                }
            }
        }

        // If no blank can fit the file, skip it
        if blank_size == usize::MAX {
            continue;
        }

        // Remove the blank from the heap
        let _ = blanks[blank_size].pop();

        // Move the file to the blank
        cur_file.start = blank_pos;

        // If the blank has space left, add it back to the heap
        let new_size = blank_size - cur_file.size as usize;
        let new_start = blank_pos + cur_file.size;

        if new_size > 0 {
            blanks[new_size].push(Reverse(new_start));
        }
    }

    let mut checksum = 0;
    for (fid, file) in files.iter().enumerate() {
        for pos in file.start..(file.start + file.size) {
            checksum += pos * fid as u64;
        }
    }

    checksum.to_string()
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
