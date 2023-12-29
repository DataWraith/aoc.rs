use crate::structs::*;

pub fn part1(input: &PuzzleInput) -> String {
    input
        .numbers
        .iter()
        .map(|row| extrapolate(row.clone()))
        .sum::<isize>()
        .to_string()
}

pub fn extrapolate(report: Vec<isize>) -> isize {
    fn pyramid(report: Vec<isize>) -> Vec<Vec<isize>> {
        fn differences(history: &[isize]) -> Vec<isize> {
            history
                .windows(2)
                .map(|window| window[1] - window[0])
                .collect()
        }

        let mut difference_pyramid = vec![report];

        loop {
            let last = difference_pyramid.last().unwrap();
            let next = differences(last);
            let first = next.first().cloned().unwrap();

            let stop = next.iter().all(|&n| n == first);

            difference_pyramid.push(next);

            if stop {
                break;
            };
        }

        difference_pyramid
    }

    fn next_value(pyramid: Vec<Vec<isize>>) -> isize {
        let mut current = *pyramid.last().unwrap().last().unwrap();

        for row in pyramid.iter().rev().skip(1) {
            current += row.last().unwrap();
        }

        current
    }

    next_value(pyramid(report))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extrapolate() {
        assert_eq!(extrapolate(vec![0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(extrapolate(vec![1, 3, 6, 10, 15, 21]), 28);
        assert_eq!(extrapolate(vec![10, 13, 16, 21, 30, 45]), 68);
        assert_eq!(extrapolate(vec![15, 12, 9, 6, 3, 0]), -3);
        assert_eq!(extrapolate(vec![21, 15, 10, 6, 3, 1]), 0);
        assert_eq!(extrapolate(vec![45, 30, 21, 16, 13, 10]), 5);
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&crate::parser::parse(
                "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45"
            )),
            "114"
        );
    }
}
