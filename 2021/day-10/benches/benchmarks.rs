use day_10::p1;
use day_10::p2;
use day_10::parser;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    let puzzle_input = parser::part1(include_str!("../input.txt"));
    p1::part1(divan::black_box(&puzzle_input));
}

#[divan::bench]
fn part2() {
    let puzzle_input = parser::part2(include_str!("../input.txt"));
    p2::part2(divan::black_box(&puzzle_input));
}
