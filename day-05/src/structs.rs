use std::ops::Range;

use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub seeds: Vec<isize>,
    pub maps: Vec<Vec<RangeMap>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct RangeMap {
    pub source: Range<isize>,
    pub destination: Range<isize>,
}

impl RangeMap {
    pub fn lookup(&self, number: isize) -> isize {
        if !self.source.contains(&number) {
            return number;
        }

        self.destination.start + number - self.source.start
    }

    pub fn range_split(&self, other_range: Range<isize>) -> ArrayVec<[Range<isize>; 3]> {
        fn range_intersects(r1: &Range<isize>, r2: &Range<isize>) -> bool {
            r1.start >= r2.start && r1.start < r2.end || r2.start >= r1.start && r2.start < r1.end
        }

        let mut result = ArrayVec::default();

        if !range_intersects(&self.source, &other_range) {
            result.push(other_range);
            return result;
        }

        // Left segment
        let s = self.source.start.min(other_range.start);
        let e = self.source.start.max(other_range.start);

        if s == other_range.start && s != self.source.start {
            result.push(s..e);
        }

        // Middle segment
        let s = self.source.start.max(other_range.start);
        let e = self.source.end.min(other_range.end);

        let shift = self.lookup(s) - s;
        result.push((s + shift)..(e + shift));

        // Right segment
        let s = self.source.end.min(other_range.end);
        let e = self.source.end.max(other_range.end);

        if e == other_range.end && e != self.source.end {
            result.push(s..e);
        }

        result.into_iter().filter(|r| !r.is_empty()).collect()
    }
}
