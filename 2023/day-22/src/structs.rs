use crate::bvh::BBox;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub bricks: Vec<BBox>,
}
