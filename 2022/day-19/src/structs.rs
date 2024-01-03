use std::ops::{Add, Mul, Sub};

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

impl Mul<usize> for Resources {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Resources {
            ore: self.ore * rhs as isize,
            clay: self.clay * rhs as isize,
            obsidian: self.obsidian * rhs as isize,
            geodes: self.geodes * rhs as isize,
        }
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
