use std::collections::HashMap;

use itertools::Itertools;

use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

fn load_stacks(input: &str) -> HashMap<usize, Vec<char>> {
    input
        .lines()
        .rev()
        .skip(1)
        .fold(HashMap::<usize, Vec<char>>::new(), |mut map, line| {
            for (i, c) in line.chars().enumerate() {
                if !c.is_alphabetic() {
                    continue;
                }
                map.entry(i).or_default().push(c);
            }
            map
        })
}

pub fn part1(input: String) {
    let mut split = input.split("\n\n");
    let mut stacks = load_stacks(split.next().unwrap_or_default());
    let indices = stacks.keys().sorted().copied().collect_vec();
    for (count, src, dest) in split
        .next()
        .unwrap_or_default()
        .lines()
        .filter_map(|l| parse_nums::<usize>(l).tuples().next())
    {
        for _ in 0..count {
            let c = stacks
                .entry(indices[src - 1])
                .or_default()
                .pop()
                .expect("None found");
            stacks.entry(indices[dest - 1]).or_default().push(c);
        }
    }
    indices
        .iter()
        .map(|i| {
            stacks
                .entry(*i)
                .or_default()
                .last()
                .copied()
                .unwrap_or_default()
        })
        .join("")
        .print();
}

pub fn part2(input: String) {
    let mut split = input.split("\n\n");
    let mut stacks = load_stacks(split.next().unwrap_or_default());
    let indices = stacks.keys().sorted().copied().collect_vec();
    for (count, src, dest) in split
        .next()
        .unwrap_or_default()
        .lines()
        .filter_map(|l| parse_nums::<usize>(l).tuples().next())
    {
        let mut to_add = vec![];
        for _ in 0..count {
            let c = stacks
                .entry(indices[src - 1])
                .or_default()
                .pop()
                .expect("None found");
            to_add.push(c);
        }
        for c in to_add.iter().rev() {
            stacks.entry(indices[dest - 1]).or_default().push(*c);
        }
    }
    indices
        .iter()
        .map(|i| {
            stacks
                .entry(*i)
                .or_default()
                .last()
                .copied()
                .unwrap_or_default()
        })
        .join("")
        .print();
}
