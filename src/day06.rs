use std::{collections::HashSet, ops::Add};

use itertools::Itertools;

use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    input
        .as_bytes()
        .windows(4)
        .find_position(|w| w.iter().collect::<HashSet<_>>().len() == 4)
        .expect("No position found")
        .0
        .add(4)
        .print();
}

pub fn part2(input: String) {
    input
        .as_bytes()
        .windows(14)
        .find_position(|w| w.iter().collect::<HashSet<_>>().len() == 14)
        .expect("No position found")
        .0
        .add(14)
        .print();
}
