use utility_belt::prelude::*;

use crate::bvh::{AABB, BVH};

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub bricks: Vec<AABB>,
}
