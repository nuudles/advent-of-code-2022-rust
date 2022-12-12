use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use pathfinding::prelude::dijkstra;

use crate::{point::Point, selfprint::SelfPrint};

fn fewest_steps(start: &Point<i64>, end: &Point<i64>, map: &HashMap<Point<i64>, u8>) -> u64 {
    let path = dijkstra(
        start,
        |p| {
            let current = map.get(p).expect("Height of current not found");
            p.neighbors()
                .iter()
                .filter_map(|n| {
                    if let Some(b) = map.get(n) {
                        if b < current || (b - current) <= 1 {
                            Some((*n, 1))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect_vec()
        },
        |p| p == end,
    );
    path.map(|p| p.1).unwrap_or(u64::MAX)
}

pub fn part1(input: String) {
    let mut start = Point { x: 0, y: 0 };
    let mut end = Point { x: 0, y: 0 };
    let mut a_points = HashSet::new();
    let map = input
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut map, (y, line)| {
            for (x, b) in line.bytes().enumerate() {
                let point = Point {
                    x: x as i64,
                    y: y as i64,
                };
                if b == b'S' {
                    start = point;
                    map.insert(point, b'a');
                } else if b == b'E' {
                    end = point;
                    map.insert(point, b'z');
                } else {
                    if b == b'a' {
                        a_points.insert(point);
                    }
                    map.insert(point, b);
                }
            }
            map
        });

    // Part 1
    fewest_steps(&start, &end, &map).print();

    // Part 2
    a_points
        .iter()
        .copied()
        .map(|start| fewest_steps(&start, &end, &map))
        .min()
        .expect("Min path could not be found")
        .print();
}
