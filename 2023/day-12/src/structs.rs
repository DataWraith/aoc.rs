#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub records: Vec<(String, Vec<usize>)>,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum State {
    ColumnStart(usize),
    Column((usize, usize)),
    Final,
}

pub struct Automaton {
    columns: Vec<usize>,
}

impl Automaton {
    pub fn new(columns: &[usize]) -> Self {
        let mut columns = columns.to_vec();
        columns.push(1);

        Automaton { columns }
    }

    pub fn transition(&self, s: &State, c: char) -> Vec<State> {
        use State::*;

        assert!(c == '.' || c == '?' || c == '#' || c == 'x');

        match (s, c) {
            // There can be arbitrary many '.'s at the start of a column
            (ColumnStart(i), '.') => vec![ColumnStart(*i)],

            // A broken spring transitions to the beginning of a column
            (ColumnStart(i), '#') => vec![Column((*i, 1))],

            // A ? may do either
            (ColumnStart(i), '?') => vec![ColumnStart(*i), Column((*i, 1))],

            // If we're in the last column, when an 'x' input occurs, we transition to the final state
            (ColumnStart(i), 'x') => {
                if *i == self.columns.len() - 1 {
                    vec![Final]
                } else {
                    vec![]
                }
            }

            // When a broken spring appears, we advance in the column
            (Column((i, j)), '#') => {
                if self.columns[*i] > *j {
                    vec![Column((*i, *j + 1))]
                } else {
                    vec![]
                }
            }

            // When a working spring appears, we advance to the next column if possible.
            (Column((i, j)), '.') => {
                if self.columns[*i] == *j && self.columns.len() > *i + 1 {
                    vec![ColumnStart(*i + 1)]
                } else {
                    vec![]
                }
            }

            // When a '?' appears, either transition may be possible
            (Column((i, j)), '?') => {
                let mut result = Vec::new();

                if self.columns[*i] > *j {
                    result.push(Column((*i, *j + 1)));
                }

                if self.columns[*i] == *j && self.columns.len() > *i + 1 {
                    result.push(ColumnStart(*i + 1));
                }

                result
            }

            (Final, _) => vec![Final],

            _ => unreachable!(),
        }
    }
}
