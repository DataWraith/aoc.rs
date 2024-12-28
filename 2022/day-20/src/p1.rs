use utility_belt::prelude::*;

use crate::parser::*;

pub struct CircularList {
    pub ring: Vec<(usize, i64)>,
}

impl CircularList {
    pub fn new(orig: &[i64]) -> Self {
        Self {
            ring: orig.iter().cloned().enumerate().collect(),
        }
    }

    pub fn mix(&mut self, orig: &[i64]) {
        for (i, value) in orig.iter().enumerate() {
            let index = self.ring.iter().position(|&x| x.0 == i).unwrap();
            self.ring.remove(index);
            let new_index = (index as i64 + *value).rem_euclid(self.ring.len() as i64) as usize;
            self.ring.insert(new_index, (i, *value));
        }
    }

    pub fn get(&self, index: usize) -> i64 {
        let zero_index = self.ring.iter().position(|&x| x.1 == 0).unwrap();
        let index = (zero_index + index) % self.ring.len();
        self.ring[index].1
    }
}

pub fn part1(input: &PuzzleInput) -> String {
    let mut list = CircularList::new(&input.sequence);

    list.mix(&input.sequence);

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
