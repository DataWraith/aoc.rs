use std::ops::{Add, Index, Mul, Sub};

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub blueprints: Vec<Blueprint>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct State {
    pub time_remaining: usize,
    pub resources: Resources,
    pub robots: Resources,
}

#[derive(Clone, Debug)]
pub struct Blueprint {
    pub number: usize,
    pub robot_costs: [Resources; 4],
    pub max_resources: Resources,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Resources {
    pub ore: isize,
    pub clay: isize,
    pub obsidian: isize,
    pub geodes: isize,
}

impl Index<usize> for Resources {
    type Output = isize;

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
