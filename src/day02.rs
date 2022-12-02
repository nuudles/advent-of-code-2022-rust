use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    input
        .lines()
        .map(|round| {
            let mut chars = round.chars();
            let (x, y) = (
                chars.next().unwrap_or_default(),
                chars.nth(1).unwrap_or_default(),
            );
            (y as u64 - 'X' as u64 + 1)
                + match (x, y) {
                    ('A', 'Y') => 6,
                    ('A', 'Z') => 0,
                    ('B', 'X') => 0,
                    ('B', 'Z') => 6,
                    ('C', 'X') => 6,
                    ('C', 'Y') => 0,
                    _ => 3,
                }
        })
        .sum::<u64>()
        .print();
}

pub fn part2(input: String) {
    input
        .lines()
        .map(|round| {
            let mut chars = round.chars();
            let (x, y) = (
                chars.next().unwrap_or_default(),
                chars.nth(1).unwrap_or_default(),
            );
            let p = match (x, y) {
                ('A', 'X') => 'C',
                ('A', 'Z') => 'B',
                ('B', 'X') => 'A',
                ('B', 'Z') => 'C',
                ('C', 'X') => 'B',
                ('C', 'Z') => 'A',
                _ => x,
            };
            (p as u64 - 'A' as u64 + 1)
                + match y {
                    'X' => 0,
                    'Y' => 3,
                    'Z' => 6,
                    _ => 0,
                }
        })
        .sum::<u64>()
        .print();
}
