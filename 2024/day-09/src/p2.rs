use utility_belt::prelude::*;

use crate::parser::*;

pub type FileId = u64;

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: u64,
    pub size: u64,
}

#[tracing::instrument(skip(input))]
pub fn part2(input: &PuzzleInput) -> String {
    let mut files: HashMap<FileId, Span> = HashMap::new();
    let mut blanks: Vec<Span> = vec![];

    let mut fid = 0;
    let mut cur = 0;

    for (i, &d) in input.disk.iter().enumerate() {
        if i % 2 == 0 {
            files.insert(
                fid,
                Span {
                    start: cur,
                    size: d,
                },
            );
            fid += 1;
        } else {
            blanks.push(Span {
                start: cur,
                size: d,
            });
        }

        cur += d;
    }

    while fid > 0 {
        fid -= 1;

        let cur_file = files.get_mut(&fid).unwrap();

        for (i, blank) in blanks.iter_mut().enumerate() {
            if blank.start > cur_file.start {
                break;
            }

            if blank.size == cur_file.size {
                cur_file.start = blank.start;
                blanks.remove(i);
                break;
            } else if blank.size > cur_file.size {
                cur_file.start = blank.start;
                blank.start += cur_file.size;
                blank.size -= cur_file.size;
                break;
            }
        }
    }

    let mut checksum = 0;

    for (fid, file) in files.iter() {
        for pos in file.start..(file.start + file.size) {
            checksum += pos * fid;
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
