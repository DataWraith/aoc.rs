#!/usr/bin/env -S cargo +nightly -Zscript
---cargo
[dependencies]
chrono = "0.4"
reqwest = { version = "0.12", features = ["blocking"] }
---

use chrono::{self, Datelike};
use reqwest;

fn main() {
    let session = get_session();

    if std::path::Path::new("input.txt").exists() {
        std::process::exit(0);
    }

    let (year, day) = get_year_and_day();
    let puzzle_input = get_puzzle_input(year, day, &session);

    std::fs::write("input.txt", puzzle_input).expect("Failed to write input to file");
}

fn get_year_and_day() -> (u32, u32) {
    let cwd = std::env::current_dir().expect("Failed to get current directory");

    let split = format!("{}", cwd.display())
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.parse::<u32>()
                .expect("Failed to parse workinng directory")
        })
        .collect::<Vec<_>>();

    let year = *split.get(0).expect("Failed to parse year");
    let day = *split.get(1).expect("Failed to parse day");

    if year < 2015 || year > chrono::Utc::now().year_ce().1 {
        eprintln!("Year is out of range");
        std::process::exit(1);
    }

    if day < 1 || day > 25 {
        eprintln!("Day is out of range");
        std::process::exit(1);
    }

    (year, day)
}

fn get_session() -> String {
    std::env::var("AOC_SESSION").expect("AOC_SESSION is not set")
}

fn get_puzzle_input(year: u32, day: u32, session: &str) -> String {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    reqwest::blocking::Client::new()
        .get(url)
        .header("Cookie", format!("session={session}"))
        .send()
        .expect("Failed to get input")
        .text()
        .expect("Failed to convert HTTP response into text")
        .to_string()
}
