use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    let mut file_tree = BTreeMap::new();
    let mut current_directory = PathBuf::from("/");

    // Register the root directory
    file_tree.insert(current_directory.clone(), 0);

    for line in input.lines().skip(1) {
        // Ignore `ls` input, we just need the file lists
        if line == "$ ls" {
            continue;
        }

        // Change the current directory
        if &line[0..4] == "$ cd" {
            let cd = &line[5..];

            if cd == ".." {
                current_directory.pop();
            } else {
                current_directory.push(cd);
            }

            continue;
        }

        // Register a directory from `ls` output
        if &line[0..4] == "dir " {
            let dir = line[4..].to_string();
            let path = current_directory.join(&dir);

            file_tree.insert(path, 0);

            continue;
        }

        // Register a file
        let split = line.split(' ').collect::<Vec<_>>();
        let size = split[0].parse::<usize>().unwrap();
        let name = split[1].to_string();
        let path = current_directory.join(&name);

        file_tree.insert(path, size);
    }

    PuzzleInput {
        filesystem: file_tree,
    }
}
