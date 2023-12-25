use std::{iter::Sum, ops::Add};

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub games: Vec<Game>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Cubes {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl Add for Cubes {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Cubes {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sum for Cubes {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Default::default(), |acc, cubes| acc + cubes)
    }
}

impl Cubes {
    pub fn is_possible(&self, query: Cubes) -> bool {
        self.red <= query.red && self.green <= query.green && self.blue <= query.blue
    }

    pub fn max(&self, other: Self) -> Self {
        Cubes {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    pub fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

#[derive(Clone, Debug)]
pub struct Game {
    pub id: usize,
    pub cubes: Vec<Cubes>,
}

impl Game {
    pub fn is_possible(&self, query: Cubes) -> bool {
        self.cubes.iter().all(|cubes| cubes.is_possible(query))
    }
}
