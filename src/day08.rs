use std::collections::{HashMap, HashSet};

use crate::{point::Point, selfprint::SelfPrint};

fn find_visible(
    point: Point<i64>,
    delta: Point<i64>,
    trees: &HashMap<Point<i64>, u8>,
    visible: &mut HashSet<Point<i64>>,
) {
    let mut point = point;
    visible.insert(point);

    let mut tallest = trees.get(&point).expect("Tree not found");
    loop {
        point = point + delta;
        if let Some(next) = trees.get(&point) {
            if next > tallest {
                visible.insert(point);
                tallest = next;
            }
        } else {
            break;
        }
    }
}

pub fn part1(input: String) {
    let mut max = (i64::MIN, i64::MIN);
    let trees = input
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut map, (y, line)| {
            line.bytes().enumerate().for_each(|(x, b)| {
                map.insert(
                    Point {
                        x: x as i64,
                        y: y as i64,
                    },
                    b - b'0',
                );
            });
            max.0 = (line.len() - 1) as i64;
            max.1 = y as i64;
            map
        });
    let mut visible = HashSet::new();
    for x in 0..=max.0 {
        find_visible(
            Point { x, y: 0 },
            Point { x: 0, y: 1 },
            &trees,
            &mut visible,
        );
        find_visible(
            Point { x, y: max.1 },
            Point { x: 0, y: -1 },
            &trees,
            &mut visible,
        );
    }
    for y in 0..=max.1 {
        find_visible(
            Point { x: 0, y },
            Point { x: 1, y: 0 },
            &trees,
            &mut visible,
        );
        find_visible(
            Point { x: max.0, y },
            Point { x: -1, y: 0 },
            &trees,
            &mut visible,
        );
    }
    println!("{}", visible.len());
}

fn scenic_score(point: &Point<i64>, trees: &HashMap<Point<i64>, u8>) -> u64 {
    let mut score = 1;
    let current = trees.get(point).expect("Tree not found");
    for delta in [
        Point { x: 0, y: -1 },
        Point { x: 0, y: 1 },
        Point { x: -1, y: 0 },
        Point { x: 1, y: 0 },
    ] {
        let mut neighbor = *point + delta;
        let mut distance = 0;
        while let Some(next) = trees.get(&neighbor) {
            distance += 1;
            if next >= current {
                break;
            }
            neighbor = neighbor + delta;
        }
        score *= distance;
    }
    score
}

pub fn part2(input: String) {
    let trees = input
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut map, (y, line)| {
            line.bytes().enumerate().for_each(|(x, b)| {
                map.insert(
                    Point {
                        x: x as i64,
                        y: y as i64,
                    },
                    b - b'0',
                );
            });
            map
        });
    trees
        .keys()
        .map(|t| scenic_score(t, &trees))
        .max()
        .expect("No max found")
        .print();
}
