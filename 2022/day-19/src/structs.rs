use std::ops::{Index, IndexMut};

use derive_more::{Add, AddAssign, Mul, Sub, SubAssign};

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub blueprints: Vec<Blueprint>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct State {
    pub time_remaining: u8,
    pub pack: Pack,
}

impl State {
    pub fn new(time_limit: u8) -> Self {
        State {
            time_remaining: time_limit,
            pack: Pack {
                resources: Resources::default(),
                robots: Resources {
                    ore: 1,
                    clay: 0,
                    obsidian: 0,
                    geodes: 0,
                },
            },
        }
    }

    pub fn tick(&self) -> State {
        State {
            time_remaining: self.time_remaining - 1,
            pack: Pack {
                resources: self.pack.resources + self.pack.robots,
                robots: self.pack.robots.clone(),
            },
        }
    }

    pub fn buy_robot(mut self, robot: usize, cost: Resources) -> Self {
        self.pack.resources -= cost;
        self.pack.robots[robot] += 1;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Pack {
    pub resources: Resources,
    pub robots: Resources,
}

impl Pack {
    pub fn can_afford(&self, cost: &Resources) -> bool {
        self.resources.ore >= cost.ore
            && self.resources.clay >= cost.clay
            && self.resources.obsidian >= cost.obsidian
            && self.resources.geodes >= cost.geodes
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Blueprint {
    pub number: usize,
    pub robot_costs: [Resources; 4],
    pub max_resources: Resources,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Add, AddAssign, Sub, SubAssign, Mul)]
pub struct Resources {
    pub ore: u8,
    pub clay: u8,
    pub obsidian: u8,
    pub geodes: u8,
}

impl Index<usize> for Resources {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.ore,
            1 => &self.clay,
            2 => &self.obsidian,
            3 => &self.geodes,
            _ => panic!("Invalid index"),
        }
    }
}

impl IndexMut<usize> for Resources {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.ore,
            1 => &mut self.clay,
            2 => &mut self.obsidian,
            3 => &mut self.geodes,
            _ => panic!("Invalid index"),
        }
    }
}
