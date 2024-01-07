use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub packets: Vec<(List, List)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum List {
    Nil,
    Digit(u32),
    List(Vec<Box<List>>),
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (List::Nil, List::Nil) => std::cmp::Ordering::Equal,
            (List::Nil, _) => std::cmp::Ordering::Less,
            (_, List::Nil) => std::cmp::Ordering::Greater,

            (List::List(a), List::List(b)) => a.cmp(b),
            (List::Digit(a), List::Digit(b)) => a.cmp(b),

            (List::Digit(a), other) => List::List(vec![Box::new(List::Digit(*a))]).cmp(other),
            (other, List::Digit(b)) => other.cmp(&List::List(vec![Box::new(List::Digit(*b))])),
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
