use std::collections::HashMap;

use itertools::Itertools;

use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

#[derive(Debug)]
enum Entry {
    Directory(Vec<String>),
    File(usize),
}

impl Entry {
    fn from(string: &str, current: &[String]) -> Self {
        if string.starts_with("dir") {
            let mut next = current.to_vec();
            next.push(
                string
                    .split_whitespace()
                    .last()
                    .expect("Directory listing not found")
                    .to_string(),
            );
            Self::Directory(next)
        } else {
            let size = parse_nums::<usize>(string)
                .next()
                .expect("No file size found");
            Self::File(size)
        }
    }
}

pub fn part1(input: String) {
    let mut contents = HashMap::<Vec<String>, Vec<Entry>>::new();
    let mut current = vec![];
    for line in input.lines().skip(1) {
        if line.starts_with('$') {
            if line == "$ cd .." {
                _ = current.pop();
            } else if line.starts_with("$ cd") {
                current.push(
                    line.split_whitespace()
                        .last()
                        .expect("No directory found")
                        .to_string(),
                );
            }
        } else {
            contents
                .entry(current.clone())
                .or_default()
                .push(Entry::from(line, &current));
        }
    }
    let mut sizes = HashMap::<Vec<String>, usize>::new();
    for (directory, entries) in contents.iter().sorted_by_key(|(d, _)| d.len()).rev() {
        let size = entries
            .iter()
            .map(|e| match e {
                Entry::Directory(sub_dir) => {
                    sizes.get(sub_dir).expect("Subdirectory size not found")
                }
                Entry::File(file_size) => file_size,
            })
            .sum::<usize>();
        sizes.insert(directory.clone(), size);
    }

    // Part 1
    sizes
        .values()
        .filter(|s| s <= &&100000)
        .sum::<usize>()
        .print();

    // Part 2
    let remaining = 70000000 - sizes.get(&vec![]).expect("Root directory size not found");
    sizes
        .values()
        .sorted()
        .find(|s| s > &&(30000000 - remaining))
        .expect("No appropriate directory found")
        .print();
}
