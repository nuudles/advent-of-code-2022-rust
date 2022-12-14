use std::collections::HashSet;

use itertools::{iproduct, Itertools};

use crate::{parse_nums::parse_nums, point::Point, selfprint::SelfPrint};

pub fn part1(input: String) {
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (i64::MAX, i64::MIN, i64::MAX, i64::MIN);
    let walls = input.lines().fold(HashSet::new(), |mut set, line| {
        for ((x1, y1), (x2, y2)) in parse_nums::<i64>(line).tuples().tuple_windows() {
            for (x, y) in iproduct!(x1.min(x2)..=x1.max(x2), y1.min(y2)..=y1.max(y2)) {
                set.insert(Point { x, y });
            }
            min_x = min_x.min(x1).min(x2);
            max_x = max_x.max(x1).max(x2);
            min_y = min_y.min(y1).min(y2);
            max_y = max_y.max(y1).max(y2);
        }
        set
    });
    let mut sand = HashSet::new();
    loop {
        let mut particle = Point { x: 500i64, y: 0i64 };
        'outer: while particle.y <= max_y {
            for next in [
                particle.down(),
                particle.down().left(),
                particle.down().right(),
            ] {
                if !walls.contains(&next) && !sand.contains(&next) {
                    particle = next;
                    continue 'outer;
                }
            }
            sand.insert(particle);
            break;
        }
        if !sand.contains(&particle) {
            break;
        }
    }
    sand.len().print();
}

pub fn part2(input: String) {
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (i64::MAX, i64::MIN, i64::MAX, i64::MIN);
    let walls = input.lines().fold(HashSet::new(), |mut set, line| {
        for ((x1, y1), (x2, y2)) in parse_nums::<i64>(line).tuples().tuple_windows() {
            for (x, y) in iproduct!(x1.min(x2)..=x1.max(x2), y1.min(y2)..=y1.max(y2)) {
                set.insert(Point { x, y });
            }
            min_x = min_x.min(x1).min(x2);
            max_x = max_x.max(x1).max(x2);
            min_y = min_y.min(y1).min(y2);
            max_y = max_y.max(y1).max(y2);
        }
        set
    });
    let mut sand = HashSet::new();
    while !sand.contains(&Point { x: 500, y: 0 }) {
        let mut particle = Point { x: 500i64, y: 0i64 };
        'outer: loop {
            for next in [
                particle.down(),
                particle.down().left(),
                particle.down().right(),
            ] {
                if !(walls.contains(&next) || sand.contains(&next) || next.y >= max_y + 2) {
                    particle = next;
                    continue 'outer;
                }
            }
            sand.insert(particle);
            break;
        }
    }
    sand.len().print();
}
