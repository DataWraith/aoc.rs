use crate::structs::*;

pub fn parse2(input: &str) -> PuzzleInput {
    let mut input = input;
    let mut enabled = true;
    let mut muls = Vec::new();

    while !input.is_empty() {
        if input.starts_with("do()") {
            input = &input[4..];
            enabled = true;
            continue;
        }

        if input.starts_with("don't()") {
            input = &input[7..];
            enabled = false;
            continue;
        }

        if !enabled || !input.starts_with("mul(") {
            input = &input[1..];
            continue;
        }

        input = &input[4..];

        let Some((a, b)) = input.split_once(")") else {
            // Unclosed parenthesis, skip the rest of the input
            break;
        };

        if a.len() > 7 {
            // Too many digits (also generally caused by unclosed or mismatched parentheses)
            input = &input[1..];
            continue;
        }

        let Some((x, y)) = a.split_once(",") else {
            input = b;
            continue;
        };

        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();

        muls.push(x * y);

        input = b;
    }

    PuzzleInput { muls }
}

pub fn parse(input: &str) -> PuzzleInput {
    let mut input = input;
    let mut muls = Vec::new();

    while !input.is_empty() {
        if !input.starts_with("mul(") {
            input = &input[1..];
            continue;
        }

        input = &input[4..];

        let Some((a, b)) = input.split_once(")") else {
            break;
        };

        if a.len() > 7 {
            input = &input[1..];
            continue;
        }

        let Some((x, y)) = a.split_once(",") else {
            input = b;
            continue;
        };

        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();

        muls.push(x * y);

        input = b;
    }

    PuzzleInput { muls }
}
