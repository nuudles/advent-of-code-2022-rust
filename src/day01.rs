use itertools::Itertools;

use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|l| l.parse::<u64>().unwrap_or_default())
                .sum::<u64>()
        })
        .max()
        .unwrap_or_default()
        .print();
}

pub fn part2(input: String) {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|l| l.parse::<u64>().unwrap_or_default())
                .sum::<u64>()
        })
        .sorted()
        .rev()
        .take(3)
        .sum::<u64>()
        .print();
}
