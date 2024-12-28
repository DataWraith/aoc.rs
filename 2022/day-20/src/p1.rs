use utility_belt::prelude::*;

use crate::parser::*;

pub struct CircularList {
    pub ring: VecDeque<(usize, i64)>,
}

impl CircularList {
    pub fn new(orig: &[i64]) -> Self {
        Self {
            ring: orig.iter().cloned().enumerate().collect(),
        }
    }

    pub fn rotate_left(&mut self, amount: usize) {
        for _ in 0..amount {
            let value = self.ring.pop_front().unwrap();
            self.ring.push_back(value);
        }
    }

    pub fn rotate_right(&mut self, amount: usize) {
        for _ in 0..amount {
            let value = self.ring.pop_back().unwrap();
            self.ring.push_front(value);
        }
    }

    pub fn mix(&mut self, orig: &[i64]) {
        for (i, value) in orig.iter().enumerate() {
            if value == &0 {
                continue;
            }

            let index = self.ring.iter().position(|&x| x.0 == i).unwrap();
            self.rotate_left(index);

            let v = self.ring.pop_front().unwrap();
            assert_eq!(v.1, *value);

            if *value < 0 {
                self.rotate_right((-value % self.ring.len() as i64) as usize);
            } else {
                self.rotate_left((*value % self.ring.len() as i64) as usize);
            }

            self.ring.push_front((i, *value));
        }
    }

    pub fn reset(&mut self) {
        let zero_index = self.ring.iter().position(|&x| x.1 == 0).unwrap();
        self.rotate_left(zero_index);
    }

    pub fn get(&self, index: usize) -> i64 {
        self.ring[index % self.ring.len()].1
    }
}

pub fn part1(input: &PuzzleInput) -> String {
    let mut list = CircularList::new(&input.sequence);

    list.mix(&input.sequence);
    list.reset();

    let mut sum = 0;
    for i in 1..=3 {
        sum += list.get(1000 * i);
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = indoc! {"
        1
        2
        -3
        3
        -2
        0
        4
    "};

    #[test]
    fn test_part1_example() {
        let input = crate::parser::part1(TEST_INPUT);
        assert_ne!(TEST_INPUT, "TODO\n");
        assert_eq!(part1(&input), "3");
    }
}
