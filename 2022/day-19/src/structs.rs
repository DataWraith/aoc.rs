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
    pub ore: i16,
    pub clay: i16,
    pub obsidian: i16,
    pub geodes: i16,
}

impl Index<usize> for Resources {
    type Output = i16;

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
            ore: self.ore * rhs as i16,
            clay: self.clay * rhs as i16,
            obsidian: self.obsidian * rhs as i16,
            geodes: self.geodes * rhs as i16,
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
