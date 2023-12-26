use std::collections::BTreeMap;

pub struct PuzzleInput {
    pub instructions: String,
    pub nodes: BTreeMap<String, (String, String)>,
}

impl PuzzleInput {
    pub fn instructions(&self) -> impl Iterator<Item = char> + '_ {
        self.instructions.chars().clone().cycle()
    }

    pub fn path_to_end(&self, start: String) -> impl Iterator<Item = String> + '_ {
        let mut current = start;
        let mut instrs = self.instructions();

        std::iter::once(current.clone()).chain(std::iter::from_fn(move || {
            if current == "ZZZ" {
                return None;
            }

            let (left, right) = self.nodes.get(&current).unwrap();
            let go_left = instrs.next().unwrap() == 'L';
            current = if go_left { left.clone() } else { right.clone() };
            Some(current.clone())
        }))
    }
}

#[derive(Debug, Clone)]
pub struct GraphNode {
    pub node: String,
    pub index: usize,
}

impl PartialEq for GraphNode {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

impl Eq for GraphNode {}
