use std::ops::{Add, Sub};

use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub blueprints: Vec<Blueprint>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Action {
    OreRobot = 0,
    ClayRobot = 1,
    ObsidianRobot = 2,
    GeodeRobot = 3,
    Wait = 4,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct State {
    pub time: usize,
    pub resources: Resources,
    pub robots: Resources,
}

impl State {
    pub fn atoms(&self) -> [isize; 9] {
        [
            self.time as isize,
            self.resources.ore,
            self.resources.clay,
            self.resources.obsidian,
            self.resources.geodes,
            self.robots.ore,
            self.robots.clay,
            self.robots.obsidian,
            self.robots.geodes,
        ]
    }
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
