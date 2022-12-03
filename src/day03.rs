use std::collections::HashSet;

use itertools::Itertools;

use crate::selfprint::SelfPrint;

fn priority(c: char) -> u64 {
    if c.is_lowercase() {
        c as u64 - 'a' as u64 + 1
    } else {
        c as u64 - 'A' as u64 + 27
    }
}

pub fn part1(input: String) {
    input
        .lines()
        .map(|line| {
            let set = line.chars().take(line.len() / 2).collect::<HashSet<_>>();
            let c = line
                .chars()
                .rev()
                .find(|c| set.contains(c))
                .expect("No duplicate found");
            priority(c)
        })
        .sum::<u64>()
        .print();
}

pub fn part2(input: String) {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|group| {
            let sets = group
                .map(|l| l.chars().collect::<HashSet<_>>())
                .collect_vec();
            let intersection = sets[0]
                .intersection(&sets[1])
                .copied()
                .collect::<HashSet<_>>();
            let c = intersection
                .intersection(&sets[2])
                .next()
                .expect("No duplicate found");
            priority(*c)
        })
        .sum::<u64>()
        .print();
}
