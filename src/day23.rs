use std::collections::{HashMap, HashSet, VecDeque};

use crate::point::Point;

pub fn part1(input: String) {
    let mut elves = input
        .lines()
        .enumerate()
        .fold(HashSet::new(), |mut set, (y, line)| {
            for (x, _) in line.chars().enumerate().filter(|(_, c)| c == &'#') {
                set.insert(Point {
                    x: x as i64,
                    y: y as i64,
                });
            }
            set
        });
    let mut deltas = VecDeque::from([
        (
            [
                Point { x: -1i64, y: -1i64 },
                Point { x: 0, y: -1 },
                Point { x: 1, y: -1 },
            ],
            0,
        ),
        (
            [
                Point { x: -1, y: 1 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
            ],
            1,
        ),
        (
            [
                Point { x: -1, y: -1 },
                Point { x: -1, y: 0 },
                Point { x: -1, y: 1 },
            ],
            2,
        ),
        (
            [
                Point { x: 1, y: -1 },
                Point { x: 1, y: 0 },
                Point { x: 1, y: 1 },
            ],
            3,
        ),
    ]);
    for i in 0..10000000 {
        let mut proposals: HashMap<Point<i64>, HashSet<Point<i64>>> = HashMap::new();
        for elf in &elves {
            let has_neighbor = elf
                .neighbors_with_diagonals()
                .iter()
                .any(|n| elves.contains(n));
            if !has_neighbor {
                proposals.entry(*elf).or_default().insert(*elf);
            } else if let Some((_, direction)) = deltas
                .iter()
                .find(|a| a.0.iter().all(|n| !elves.contains(&(*n + *elf))))
            {
                proposals
                    .entry(match direction {
                        0 => elf.up(),
                        1 => elf.down(),
                        2 => elf.left(),
                        _ => elf.right(),
                    })
                    .or_default()
                    .insert(*elf);
            } else {
                proposals.entry(*elf).or_default().insert(*elf);
            }
        }
        let next = proposals
            .iter()
            .fold(HashSet::new(), |mut set, (point, candidates)| {
                if candidates.len() == 1 {
                    set.insert(*point);
                } else {
                    set.extend(candidates);
                }
                set
            });
        if elves == next {
            println!("Part 2: {}", i + 1);
            break;
        }
        elves = next;
        deltas.rotate_left(1);

        if i == 9 {
            let (mut min_x, mut max_x, mut min_y, mut max_y) =
                (i64::MAX, i64::MIN, i64::MAX, i64::MIN);
            for elf in &elves {
                min_x = min_x.min(elf.x);
                max_x = max_x.max(elf.x);
                min_y = min_y.min(elf.y);
                max_y = max_y.max(elf.y);
            }
            println!(
                "Part 1: {}",
                (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i64
            );
        }
    }
}
