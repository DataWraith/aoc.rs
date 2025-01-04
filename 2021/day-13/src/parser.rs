use utility_belt::prelude::*;

type Fold = (bool, i32);

#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub grid: HashSet<Coordinate>,
    pub folds: Vec<Fold>,
}

pub fn part1(input: &'static str) -> PuzzleInput {
    let (grid, folds) = input.split_once("\n\n").unwrap();

    let grid = create_grid(grid);
    let folds = parse_folds(folds);

    PuzzleInput { grid, folds }
}

fn create_grid(grid: &str) -> HashSet<Coordinate> {
    let grid_numbers = parse_ints(grid);

    grid_numbers
        .chunks(2)
        .map(|w| Coordinate::new(w[0] as i32, w[1] as i32))
        .collect()
}

fn parse_folds(folds: &str) -> Vec<Fold> {
    folds.lines().map(parse_fold).collect()
}

fn parse_fold(fold: &str) -> Fold {
    let (axis, value) = fold.split_once("=").unwrap();
    let axis = axis == "fold along y";
    let value = value.parse().unwrap();
    (axis, value)
}

pub fn part2(input: &'static str) -> PuzzleInput {
    part1(input)
}

pub const TEST_INPUT: &str = indoc! {"
    6,10
    0,14
    9,10
    0,3
    10,4
    4,11
    6,0
    6,12
    4,1
    0,13
    10,12
    3,4
    3,0
    8,4
    1,10
    2,14
    8,10
    9,0

    fold along y=7
    fold along x=5
"};
