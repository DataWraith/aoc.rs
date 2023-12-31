use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub guide: Vec<(Rps, Rps)>,
}

#[derive(Clone, Copy, Debug)]
pub enum Rps {
    R,
    P,
    S,
    X,
    Y,
    Z,
}

impl From<char> for Rps {
    fn from(c: char) -> Self {
        match c {
            'A' => Rps::R,
            'B' => Rps::P,
            'C' => Rps::S,
            'X' => Rps::X,
            'Y' => Rps::Y,
            'Z' => Rps::Z,
            _ => panic!("Invalid RPS char"),
        }
    }
}

impl Rps {
    pub fn convert(self) -> Rps {
        match self {
            Rps::R => Rps::R,
            Rps::P => Rps::P,
            Rps::S => Rps::S,
            Rps::X => Rps::R,
            Rps::Y => Rps::P,
            Rps::Z => Rps::S,
        }
    }

    pub fn outcome_score(self, other: Self) -> usize {
        match (self, other) {
            (Rps::R, Rps::S) => 6,
            (Rps::R, Rps::R) => 3,
            (Rps::R, Rps::P) => 0,
            (Rps::P, Rps::R) => 6,
            (Rps::P, Rps::P) => 3,
            (Rps::P, Rps::S) => 0,
            (Rps::S, Rps::P) => 6,
            (Rps::S, Rps::S) => 3,
            (Rps::S, Rps::R) => 0,
            _ => panic!("Invalid RPS game"),
        }
    }

    pub fn choice_score(self) -> usize {
        match self {
            Rps::R => 1,
            Rps::P => 2,
            Rps::S => 3,
            _ => panic!("Invalid RPS choice"),
        }
    }

    pub fn score_vs(self, other: Self) -> usize {
        let myself = self.convert();
        let opponent = other.convert();

        myself.outcome_score(opponent) + myself.choice_score() 
    }
}
