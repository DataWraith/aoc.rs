use std::collections::LinkedList;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    // Remember to make the fields pub
    pub stones: LinkedList<String>,
}

pub fn part1(input: &str) -> PuzzleInput {
    let mut list = LinkedList::new();
    for stone in input.split_ascii_whitespace() {
        list.push_back(String::from(stone));
    }
    PuzzleInput { stones: list }
}

pub fn part2(input: &str) -> PuzzleInput {
    part1(input)
}
