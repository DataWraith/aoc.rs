use utility_belt::prelude::*;

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    // Remember to make the fields pub
    pub numbers: Vec<i64>,
    pub boards: Vec<Grid2D<i64>>,
    pub marked: Vec<BoolGrid2D>,
    pub ongoing: HashSet<usize>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let (numbers, boards) = input.split_once("\n\n").unwrap();
    let numbers = parse_ints(numbers);

    let boards: Vec<Grid2D<i64>> = parse_ints(boards)
        .chunks(25)
        .map(|chunk| Grid2D::from_shape_vec(5, 5, chunk.to_vec()))
        .collect();

    let marked = boards
        .iter()
        .map(|grid| grid.map(|_| false).into())
        .collect_vec();

    PuzzleInput {
        numbers,
        boards,
        ongoing: HashSet::from_iter(0..marked.len()),
        marked,
    }
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}
