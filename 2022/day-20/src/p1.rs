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

    pub fn mix(&mut self, orig: &[i64]) {
        for (i, value) in orig.iter().enumerate() {
            // Linear search for the index of the current element. This is
            // faster than rotating the list to the left until the current
            // element is at the front because the built-in rotate_left will
            // actually rotate right if the distance is shorter that way.
            let index = self.ring.iter().position(|&x| x.0 == i).unwrap();
            self.ring.rotate_left(index);

            // Move the element by popping it, rotating the list, and then
            // pushing it back.
            let _ = self.ring.pop_front();
            let distance = value.rem_euclid(self.ring.len() as i64) as usize;
            self.ring.rotate_left(distance);
            self.ring.push_front((i, *value));
        }
    }

    pub fn reset(&mut self) {
        while self.ring.front().unwrap().1 != 0 {
            self.ring.rotate_left(1);
        }
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
