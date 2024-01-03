use std::ops::{Add, Sub};

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub blueprints: Vec<Blueprint>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct State {
    pub time: usize,
    pub resources: Resources,
    pub robots: Resources,
}

#[derive(Clone, Debug)]
pub struct Blueprint {
    pub number: usize,
    pub ore_robot_cost: Resources,
    pub clay_robot_cost: Resources,
    pub obsidian_robot_cost: Resources,
    pub geode_robot_cost: Resources,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Resources {
    pub ore: isize,
    pub clay: isize,
    pub obsidian: isize,
    pub geodes: isize,
}

impl Resources {
    pub fn can_afford(&self, other: &Self) -> bool {
        self.ore >= other.ore
            && self.clay >= other.clay
            && self.obsidian >= other.obsidian
            && self.geodes >= other.geodes
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Resources {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geodes: self.geodes + rhs.geodes,
        }
    }
}

impl Sub for Resources {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Resources {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geodes: self.geodes - rhs.geodes,
        }
    }
}
