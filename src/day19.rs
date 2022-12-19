use std::collections::HashSet;

use cached::proc_macro::cached;
use itertools::Itertools;

use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Blueprint {
    id: u64,
    ore_robot_ore_cost: u64,
    clay_robot_ore_cost: u64,
    obsidian_robot_ore_cost: u64,
    obsidian_robot_clay_cost: u64,
    geode_robot_ore_cost: u64,
    geode_robot_obsidian_cost: u64,
}

impl Blueprint {
    fn from(line: &str) -> Self {
        let mut nums = parse_nums(line);
        Blueprint {
            id: nums.next().expect("id not found"),
            ore_robot_ore_cost: nums.next().expect("ore_robot_ore_cost not found"),
            clay_robot_ore_cost: nums.next().expect("clay_robot_ore_cost not found"),
            obsidian_robot_ore_cost: nums.next().expect("obsidian_robot_ore_cost not found"),
            obsidian_robot_clay_cost: nums.next().expect("obsidian_robot_clay_cost not found"),
            geode_robot_ore_cost: nums.next().expect("geode_robot_ore_cost not found"),
            geode_robot_obsidian_cost: nums.next().expect("geode_robot_obsidian_cost not found"),
        }
    }
}

// (ore, clay, obsidian, geode)
type SupplyTypes = (u64, u64, u64, u64);

#[cached(
    key = "String",
    convert = r#"{ format!("{}: {:?}", blueprint.id, supplies) }"#
)]
fn possible_robots(
    supplies: SupplyTypes,
    blueprint: &Blueprint,
) -> HashSet<(SupplyTypes, SupplyTypes)> {
    let (ore, clay, obsidian, geode) = supplies;

    // If we can build a geode robot, just build the geode robot
    if ore >= blueprint.geode_robot_ore_cost && obsidian >= blueprint.geode_robot_obsidian_cost {
        return HashSet::from([(
            (0, 0, 0, 1),
            (
                ore - blueprint.geode_robot_ore_cost,
                clay,
                obsidian - blueprint.geode_robot_obsidian_cost,
                geode,
            ),
        )]);
    }

    let mut possible = HashSet::new();
    if ore >= blueprint.ore_robot_ore_cost {
        let mut supplies = supplies;
        supplies.0 -= blueprint.ore_robot_ore_cost;
        possible.insert(((1, 0, 0, 0), supplies));
    }
    if ore >= blueprint.clay_robot_ore_cost {
        let mut supplies = supplies;
        supplies.0 -= blueprint.clay_robot_ore_cost;
        possible.insert(((0, 1, 0, 0), supplies));
    }
    if ore >= blueprint.obsidian_robot_ore_cost && clay >= blueprint.obsidian_robot_clay_cost {
        let mut supplies = supplies;
        supplies.0 -= blueprint.obsidian_robot_ore_cost;
        supplies.1 -= blueprint.obsidian_robot_clay_cost;
        possible.insert(((0, 0, 1, 0), supplies));
    }

    // Add the possibility of doing nothing
    possible.insert(((0, 0, 0, 0), supplies));

    possible
}

#[cached(
    key = "String",
    convert = r#"{ format!("{}: {} {:?} {:?}", blueprint.id, time_left, supplies, robots) }"#
)]
fn max_geode_opened(
    time_left: u64,
    supplies: SupplyTypes,
    robots: SupplyTypes,
    blueprint: &Blueprint,
) -> u64 {
    if time_left == 0 {
        return supplies.3;
    }

    let mut max = 0;

    let (ore_robot, clay_robot, obsidian_robot, geode_robot) = robots;

    for ((ore_built, clay_built, obsidian_built, geode_built), mut supplies) in
        possible_robots(supplies, blueprint)
    {
        supplies.0 += ore_robot;
        supplies.1 += clay_robot;
        supplies.2 += obsidian_robot;
        supplies.3 += geode_robot;

        let robots = (
            ore_robot + ore_built,
            clay_robot + clay_built,
            obsidian_robot + obsidian_built,
            geode_robot + geode_built,
        );

        if robots.0
            > blueprint
                .ore_robot_ore_cost
                .max(blueprint.clay_robot_ore_cost)
                .max(blueprint.geode_robot_ore_cost)
        {
            // We don't need more ore robots
            continue;
        }
        if robots.1 > blueprint.obsidian_robot_clay_cost {
            // We don't need more clay robots
            continue;
        }
        if robots.2 > blueprint.geode_robot_obsidian_cost {
            // We don't need more obsidian robots
            continue;
        }

        let time_left = time_left - 1;
        max = max.max(max_geode_opened(time_left, supplies, robots, blueprint));
    }

    max
}

pub fn part1(input: String) {
    let blueprints = input.lines().map(Blueprint::from).collect_vec();
    blueprints
        .iter()
        .map(|b| max_geode_opened(24, (0, 0, 0, 0), (1, 0, 0, 0), b) * b.id)
        .sum::<u64>()
        .print();
}

pub fn part2(input: String) {
    let blueprints = input.lines().map(Blueprint::from).collect_vec();
    blueprints
        .iter()
        .take(3)
        .map(|b| max_geode_opened(32, (0, 0, 0, 0), (1, 0, 0, 0), b))
        .product::<u64>()
        .print();
}
