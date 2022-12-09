use std::collections::HashSet;

use itertools::Itertools;

use crate::{point::Point, selfprint::SelfPrint};

pub fn part1(input: String) {
    let mut seen = HashSet::new();
    let mut h = Point { x: 0i64, y: 0i64 };
    let mut t = Point { x: 0i64, y: 0i64 };
    seen.insert(t);

    for (direction, count) in input.lines().flat_map(|l| {
        l.split(' ')
            .tuples()
            .map(|(d, c)| (d, c.parse::<usize>().unwrap_or_default()))
            .next()
    }) {
        let delta = match direction {
            "R" => Point { x: 1, y: 0 },
            "L" => Point { x: -1, y: 0 },
            "U" => Point { x: 0, y: -1 },
            "D" => Point { x: 0, y: 1 },
            _ => Point { x: 0, y: 0 },
        };
        for _ in 0..count {
            h = h + delta;
            if t.neighbors_with_diagonals().contains(&h) || t == h {
                continue;
            }
            if t.x == h.x || t.y == h.y {
                t.x = (t.x + h.x) / 2;
                t.y = (t.y + h.y) / 2;
            } else {
                t.x = t.x + if t.x < h.x { 1 } else { -1 };
                t.y = t.y + if t.y < h.y { 1 } else { -1 };
            }
            seen.insert(t);
        }
    }
    seen.len().print();
}

pub fn part2(input: String) {
    let mut seen = HashSet::new();
    let mut knots = (0..10).map(|_| Point { x: 0i64, y: 0i64 }).collect_vec();
    seen.insert(*knots.first().unwrap());

    for (direction, count) in input.lines().flat_map(|l| {
        l.split(' ')
            .tuples()
            .map(|(d, c)| (d, c.parse::<usize>().unwrap_or_default()))
            .next()
    }) {
        let delta = match direction {
            "R" => Point { x: 1, y: 0 },
            "L" => Point { x: -1, y: 0 },
            "U" => Point { x: 0, y: -1 },
            "D" => Point { x: 0, y: 1 },
            _ => Point { x: 0, y: 0 },
        };
        for _ in 0..count {
            knots[0] = knots[0] + delta;

            for i in 1..10 {
                let h = knots[i - 1];
                let mut t = knots[i];
                if t.neighbors_with_diagonals().contains(&h) || t == h {
                    continue;
                }
                if t.x == h.x || t.y == h.y {
                    t.x = (t.x + h.x) / 2;
                    t.y = (t.y + h.y) / 2;
                } else {
                    t.x = t.x + if t.x < h.x { 1 } else { -1 };
                    t.y = t.y + if t.y < h.y { 1 } else { -1 };
                }
                knots[i] = t;
            }
            seen.insert(knots[9]);
        }
    }
    seen.len().print();
}
