use std::collections::HashSet;

use itertools::Itertools;

use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

fn print_by_layer(cubes: &HashSet<(i64, i64, i64)>) {
    let (min_x, max_x) = match cubes.iter().map(|c| c.0).minmax() {
        itertools::MinMaxResult::NoElements => (-1, -1),
        itertools::MinMaxResult::OneElement(x) => (x, x),
        itertools::MinMaxResult::MinMax(min, max) => (min, max),
    };
    let (min_y, max_y) = match cubes.iter().map(|c| c.1).minmax() {
        itertools::MinMaxResult::NoElements => (-1, -1),
        itertools::MinMaxResult::OneElement(x) => (x, x),
        itertools::MinMaxResult::MinMax(min, max) => (min, max),
    };
    for z in cubes.iter().map(|c| c.2).sorted().dedup() {
        println!(
            "===z={} - ({},{}) to ({},{})===",
            z, min_x, min_y, max_x, max_y
        );
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if cubes.contains(&(x, y, z)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

pub fn part1(input: String) {
    let cubes: HashSet<(i64, i64, i64)> = input
        .lines()
        .flat_map(|l| parse_nums(l).tuples().next())
        .collect();
    cubes
        .iter()
        .map(|c| {
            [
                (0, 0, 1),
                (0, 0, -1),
                (0, 1, 0),
                (0, -1, 0),
                (1, 0, 0),
                (-1, 0, 0),
            ]
            .iter()
            .filter(|d| !cubes.contains(&(c.0 + d.0, c.1 + d.1, c.2 + d.2)))
            .count()
        })
        .sum::<usize>()
        .print();
}

pub fn part2(input: String) {
    let cubes: HashSet<(i64, i64, i64)> = input
        .lines()
        .flat_map(|l| parse_nums(l).tuples().next())
        .collect();
    let (min_x, max_x) = match cubes.iter().map(|c| c.0).minmax() {
        itertools::MinMaxResult::NoElements => (-1, -1),
        itertools::MinMaxResult::OneElement(x) => (x, x),
        itertools::MinMaxResult::MinMax(min, max) => (min, max),
    };
    let (min_y, max_y) = match cubes.iter().map(|c| c.1).minmax() {
        itertools::MinMaxResult::NoElements => (-1, -1),
        itertools::MinMaxResult::OneElement(x) => (x, x),
        itertools::MinMaxResult::MinMax(min, max) => (min, max),
    };
    let (min_z, max_z) = match cubes.iter().map(|c| c.2).minmax() {
        itertools::MinMaxResult::NoElements => (-1, -1),
        itertools::MinMaxResult::OneElement(x) => (x, x),
        itertools::MinMaxResult::MinMax(min, max) => (min, max),
    };
    let mut next = vec![
        ((min_x - 1, min_y - 1, min_z - 1), (0, 0, 1)),
        ((min_x - 1, min_y - 1, min_z - 1), (0, 1, 0)),
        ((min_x - 1, min_y - 1, min_z - 1), (1, 0, 0)),
    ];
    let mut seen = HashSet::new();
    let mut count = 0;
    while let Some((c, d)) = next.pop() {
        if seen.contains(&(c, d)) {
            continue;
        }
        seen.insert((c, d));
        let n = (c.0 + d.0, c.1 + d.1, c.2 + d.2);
        if cubes.contains(&n) {
            count += 1;
        } else if (min_x - 1..=max_x + 1).contains(&n.0)
            && (min_y - 1..=max_y + 1).contains(&n.1)
            && (min_z - 1..=max_z + 1).contains(&n.2)
        {
            for d in [
                (0, 0, 1),
                (0, 0, -1),
                (0, 1, 0),
                (0, -1, 0),
                (1, 0, 0),
                (-1, 0, 0),
            ] {
                next.push((n, d));
            }
        }
    }
    println!("{}", count);
}
