use std::collections::BTreeMap;

use utility_belt::prelude::*;

use crate::structs::*;

pub fn parse(input: &str) -> PuzzleInput {
    let mut file_tree = BTreeMap::new();
    let mut current_directory = "/".to_string();

    for line in input.lines().skip(1) {
        if line == "$ ls" {
            continue;
        }

        if &line[0..4] == "$ cd" {
            let cd = &line[5..];

            if cd == ".." {
                current_directory.pop();
                let last_slash = current_directory.rfind('/').unwrap();
                current_directory = current_directory[0..last_slash + 1].to_string();
            } else {
                current_directory.push_str(cd);
                current_directory.push('/');
            }
        } else if &line[0..4] == "dir " {
            let dir = line[4..].to_string();
            let path = format!("{}{}", current_directory, dir);
            file_tree.insert(path, 0);
        } else {
            let split = line.split(" ").collect::<Vec<_>>();
            let size = split[0].parse::<usize>().unwrap();
            let name = split[1].to_string();
            let path = format!("{}{}", current_directory, name);
            file_tree.insert(path, 1 + size);
        }
    }

    PuzzleInput {
        filesystem: file_tree,
    }
}
