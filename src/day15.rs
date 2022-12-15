use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

use itertools::Itertools;

use crate::{parse_nums::parse_nums, point::Point, selfprint::SelfPrint};

pub fn part1(input: String) {
    let (mut min_x, mut max_x) = (i64::MAX, i64::MIN);
    let mut beacons = HashSet::new();
    let covered = input.lines().fold(HashSet::new(), |mut set, line| {
        let (x1, y1, x2, y2) = parse_nums::<i64>(line)
            .tuples()
            .next()
            .expect("Parse error");
        let sensor = Point { x: x1, y: y1 };
        let beacon = Point { x: x2, y: y2 };
        beacons.insert(beacon);
        let d = sensor.manhattan_distance(&Point { x: x2, y: y2 });
        min_x = min_x.min(sensor.x - d);
        max_x = max_x.max(sensor.x + d);
        set.insert((sensor, d));
        set
    });
    (min_x..=max_x)
        .filter(|x| {
            let point = Point { x: *x, y: 2000000 };
            !beacons.contains(&point)
                && covered
                    .iter()
                    .any(|(s, d)| point.manhattan_distance(s) <= *d)
        })
        .count()
        .print();
}

pub fn part2(input: String) {
    let rows = input.lines().fold(
        HashMap::<i64, HashSet<RangeInclusive<i64>>>::new(),
        |mut map, line| {
            let (x1, y1, x2, y2) = parse_nums::<i64>(line)
                .tuples()
                .next()
                .expect("Error parsing");
            let sensor = Point { x: x1, y: y1 };
            let beacon = Point { x: x2, y: y2 };
            let d = sensor.manhattan_distance(&beacon);
            for i in 0..=d {
                map.entry(sensor.y - i)
                    .or_default()
                    .insert((sensor.x - d + i)..=(sensor.x + d - i));
                map.entry(sensor.y + i)
                    .or_default()
                    .insert((sensor.x - d + i)..=(sensor.x + d - i));
            }
            map
        },
    );
    for (y, ranges) in rows {
        if !(0..=4000000).contains(&y) {
            continue;
        }
        let mut max_x = i64::MIN;
        for range in ranges.iter().sorted_by(|a, b| a.start().cmp(b.start())) {
            if max_x != i64::MIN && max_x < *range.start() - 1 {
                println!("{}", (max_x + 1) * 4000000 + y);
                return;
            }
            max_x = max_x.max(*range.end());
        }
    }
}
