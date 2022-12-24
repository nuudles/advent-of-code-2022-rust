use std::collections::{BTreeMap, BTreeSet, VecDeque};

use itertools::Itertools;
use pathfinding::prelude::astar;

use crate::point::Point;

type Blizzard = BTreeMap<Point<i64>, BTreeSet<char>>;

fn next_blizzard(blizzards: &Blizzard, walls: &BTreeSet<Point<i64>>) -> Blizzard {
    blizzards
        .iter()
        .fold(BTreeMap::new(), |mut map, (point, storms)| {
            for &c in storms {
                let mut next = match c {
                    '>' => point.right(),
                    '<' => point.left(),
                    '^' => point.up(),
                    _ => point.down(),
                };
                if walls.contains(&next) {
                    next = match c {
                        '>' => walls
                            .iter()
                            .filter(|w| w.y == point.y)
                            .sorted_by_key(|w| w.x)
                            .next()
                            .expect("NO RIGHT WALL FOUND!?!?! EEEK")
                            .right(),
                        '<' => walls
                            .iter()
                            .filter(|w| w.y == point.y)
                            .sorted_by_key(|w| w.x)
                            .rev()
                            .next()
                            .expect("NO LEFT WALL FOUND!?!?! ZORP")
                            .left(),
                        '^' => walls
                            .iter()
                            .filter(|w| w.x == point.x)
                            .sorted_by_key(|w| w.y)
                            .rev()
                            .next()
                            .expect("NO UP WALL FOUND!?!?! FLEEP")
                            .up(),
                        _ => walls
                            .iter()
                            .filter(|w| w.x == point.x)
                            .sorted_by_key(|w| w.y)
                            .next()
                            .expect("NO DOWN WALL FOUND!?!?! KHOI")
                            .down(),
                    }
                }
                map.entry(next).or_default().insert(c);
            }
            map
        })
}

pub fn part1(input: String) {
    let mut walls = BTreeSet::new();
    let mut blizzards = BTreeMap::new();
    let mut spaces = BTreeSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let point = Point {
                x: x as i64,
                y: y as i64,
            };
            if c == '#' {
                walls.insert(point);
            } else if c == '.' {
                spaces.insert(point);
            } else {
                blizzards.insert(point, BTreeSet::from([c]));
            }
        }
    }

    let (start, end) = match spaces
        .iter()
        .filter(|p| walls.contains(&p.left()) && walls.contains(&p.right()))
        .sorted_by_key(|p| p.y)
        .minmax()
    {
        itertools::MinMaxResult::NoElements => panic!("HUH?!?!"),
        itertools::MinMaxResult::OneElement(_) => panic!("WHAT??!?!?!"),
        itertools::MinMaxResult::MinMax(min, max) => (min, max),
    };

    let mut all_blizzards = VecDeque::from([blizzards.clone()]);
    loop {
        let next_blizzard = next_blizzard(&blizzards, &walls);
        if next_blizzard == all_blizzards[0] {
            break;
        }
        blizzards = next_blizzard.clone();
        all_blizzards.push_back(next_blizzard);
    }

    let mut total = 0;

    let path = astar(
        &(*start, total),
        |(elf, b_index)| {
            let next_blizzards = &all_blizzards[(b_index + 1) % all_blizzards.len()];
            let mut next = vec![];
            for p in [*elf, elf.up(), elf.down(), elf.right(), elf.left()] {
                if !walls.contains(&p) && !&next_blizzards.contains_key(&p) && p.y > -1 {
                    next.push(((p, b_index + 1), 1));
                }
            }
            next
        },
        |(elf, _)| elf.manhattan_distance(end),
        |(elf, _)| elf == end,
    )
    .expect("Path not found... zounds...");
    println!("Part 1: {}", path.1);

    total += path.1 as usize;

    let path = astar(
        &(*end, total),
        |(elf, b_index)| {
            let next_blizzards = &all_blizzards[(b_index + 1) % all_blizzards.len()];
            let mut next = vec![];
            for p in [*elf, elf.up(), elf.down(), elf.right(), elf.left()] {
                if !walls.contains(&p) && !&next_blizzards.contains_key(&p) && p.y > -1 {
                    next.push(((p, b_index + 1), 1));
                }
            }
            next
        },
        |(elf, _)| elf.manhattan_distance(start),
        |(elf, _)| elf == start,
    )
    .expect("Path not found... zounds...");

    total += path.1 as usize;

    let path = astar(
        &(*start, total),
        |(elf, b_index)| {
            let next_blizzards = &all_blizzards[(b_index + 1) % all_blizzards.len()];
            let mut next = vec![];
            for p in [*elf, elf.up(), elf.down(), elf.right(), elf.left()] {
                if !walls.contains(&p) && !&next_blizzards.contains_key(&p) && p.y > -1 {
                    next.push(((p, b_index + 1), 1));
                }
            }
            next
        },
        |(elf, _)| elf.manhattan_distance(end),
        |(elf, _)| elf == end,
    )
    .expect("Path not found... zounds...");

    total += path.1 as usize;

    println!("Part 2: {}", total);
}
