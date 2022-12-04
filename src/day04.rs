use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    input
        .lines()
        .filter(|line| {
            let mut numbers = line
                .split(|c: char| !c.is_numeric())
                .filter_map(|n| n.parse::<u64>().ok());
            let a = numbers.next().unwrap_or_default()..=numbers.next().unwrap_or_default();
            let b = numbers.next().unwrap_or_default()..=numbers.next().unwrap_or_default();
            a.contains(b.start()) && a.contains(b.end())
                || b.contains(a.start()) && b.contains(a.end())
        })
        .count()
        .print();
}

pub fn part2(input: String) {
    input
        .lines()
        .filter(|line| {
            let mut numbers = line
                .split(|c: char| !c.is_numeric())
                .filter_map(|n| n.parse::<u64>().ok());
            let a = numbers.next().unwrap_or_default()..=numbers.next().unwrap_or_default();
            let b = numbers.next().unwrap_or_default()..=numbers.next().unwrap_or_default();
            a.contains(b.start())
                || a.contains(b.end())
                || b.contains(a.start())
                || b.contains(a.end())
        })
        .count()
        .print();
}
