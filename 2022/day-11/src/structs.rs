#[derive(Clone, Debug)]
pub struct PuzzleInput {
    pub monkeys: Vec<Monkey>,
}

#[derive(Clone, Debug, Copy)]
pub enum OperationType {
    Add(usize),
    Square,
    Mul(usize),
}

#[derive(Clone, Debug)]
pub struct Monkey {
    pub operation_type: OperationType,
    pub items: Vec<usize>,
    pub divisible_by: usize,
    pub true_monkey: usize,
    pub false_monkey: usize,
    pub inspections: usize,
}
