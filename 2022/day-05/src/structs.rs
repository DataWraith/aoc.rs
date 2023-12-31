use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub crates: Vec<Vec<char>>,
    pub instructions : Vec<(usize, usize, usize)>
}
