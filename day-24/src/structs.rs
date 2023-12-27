use utility_belt::prelude::*;

use glam::I64Vec3;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub hailstones: Vec<Hailstone>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Hailstone {
    pub position: I64Vec3,
    pub velocity: I64Vec3,
}
