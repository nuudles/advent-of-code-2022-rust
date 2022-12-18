use core::fmt;
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    vec,
};

use itertools::Itertools;
use lazy_static::lazy_static;
use pathfinding::prelude::dijkstra;
use regex::Regex;

use crate::selfprint::SelfPrint;

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Valve {
    name: String,
    rate: u64,
    tunnels: Vec<String>,
}

impl Valve {
    fn from(string: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"[A-Z]{2}|\d+").expect("Invalid Regex");
        }

        let mut matches = RE.find_iter(string).map(|m| m.as_str());
        Some(Valve {
            name: matches.next()?.to_string(),
            rate: matches.next()?.parse().ok()?,
            tunnels: matches.map(|s| s.to_string()).collect_vec(),
        })
    }
}

impl fmt::Debug for Valve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn step<'a>(
    states: Vec<(u64, HashSet<Valve>, &'a str, u64)>,
    valves: &'a HashMap<String, Valve>,
    costs: &'a HashMap<(&'a Valve, &'a Valve), u64>,
) -> Vec<(u64, HashSet<Valve>, &'a str, u64)> {
    states
        .iter()
        .flat_map(|(time_left, open, current, pressure)| {
            if time_left == &0 {
                return vec![(*time_left, open.clone(), *current, *pressure)];
            }
            if open.len() == valves.values().filter(|v| v.rate > 0).count() {
                // All valves open
                return vec![(
                    0,
                    open.clone(),
                    *current,
                    pressure + open.iter().map(|v| v.rate).sum::<u64>() * time_left,
                )];
            }
            let valve = valves.get(*current).expect("Valve not found");
            let states = valves
                .values()
                .filter(|v| v.rate > 0 && !open.contains(v))
                .map(|n| {
                    let cost = costs.get(&(valve, n)).expect("Cost not found");
                    let mut open = open.clone();
                    let build_up = open.iter().map(|v| v.rate).sum::<u64>();
                    open.insert(n.clone());
                    (
                        time_left.saturating_sub(cost + 1),
                        open,
                        n.name.as_str(),
                        pressure + build_up * (cost + 1).min(*time_left),
                    )
                })
                .collect_vec();
            step(states, valves, costs)
        })
        .collect_vec()
}

fn step_with_elephant<'a>(
    states: Vec<(u64, BTreeSet<Valve>, (Valve, u64, Valve, u64), u64)>,
    valves: &'a HashMap<String, Valve>,
    costs: &'a HashMap<(&'a Valve, &'a Valve), u64>,
    max: &mut u64,
) -> Vec<(u64, BTreeSet<Valve>, (Valve, u64, Valve, u64), u64)> {
    states
        .iter()
        .flat_map(|(time_left, open, current, pressure)| {
            // println!("{} {}", time_left, open.len());
            let (a, a_cost, b, b_cost) = current;
            if time_left == &0 {
                if *pressure > *max {
                    *max = *pressure;
                    println!("Current Max: {}", pressure);
                }
                return vec![(*time_left, open.clone(), current.clone(), *pressure)];
            }
            if open.len() == valves.values().filter(|v| v.rate > 0).count() {
                // All valves open
                let pressure = pressure + open.iter().map(|v| v.rate).sum::<u64>() * time_left;
                if pressure > *max {
                    *max = pressure;
                    println!("Current Max: {}", pressure);
                }
                return vec![(0, open.clone(), current.clone(), pressure)];
            }
            let mut open = open.clone();
            if a_cost == &0 && a.rate > 0 {
                open.insert(a.clone());
            }
            if b_cost == &0 && b.rate > 0 {
                open.insert(b.clone());
            }
            let build_up = open.iter().map(|v| v.rate).sum::<u64>();
            let possible = valves
                .values()
                .filter(|v| !open.contains(v) && v.rate > 0 && v != &a && v != &b)
                .collect::<HashSet<_>>();
            if a_cost == &0 && b_cost == &0 && possible.len() > 1 {
                let mut next = vec![];
                for (next_a, next_b) in possible.iter().tuple_combinations() {
                    let a_to_a = costs.get(&(a, next_a)).expect("Cost not found") + 1;
                    let a_to_b = costs.get(&(a, next_b)).expect("Cost not found") + 1;
                    let b_to_a = costs.get(&(b, next_a)).expect("Cost not found") + 1;
                    let b_to_b = costs.get(&(b, next_b)).expect("Cost not found") + 1;

                    let min_cost = a_to_a.min(b_to_b);
                    next.push((
                        time_left.saturating_sub(min_cost),
                        open.clone(),
                        (
                            (*next_a).clone(),
                            a_to_a - min_cost,
                            (*next_b).clone(),
                            b_to_b - min_cost,
                        ),
                        pressure + build_up * min_cost.min(*time_left),
                    ));
                    let min_cost = a_to_b.min(b_to_a);
                    next.push((
                        time_left.saturating_sub(min_cost),
                        open.clone(),
                        (
                            (*next_b).clone(),
                            a_to_b - min_cost,
                            (*next_a).clone(),
                            b_to_a - min_cost,
                        ),
                        pressure + build_up * min_cost.min(*time_left),
                    ));
                }
                step_with_elephant(next, valves, costs, max)
            } else if a_cost == &0 && !possible.is_empty() {
                let mut next = vec![];
                for next_a in possible {
                    let cost = costs.get(&(a, next_a)).expect("Cost not found") + 1;
                    let min_cost = if b_cost == &0 {
                        cost
                    } else {
                        cost.min(*b_cost)
                    };
                    next.push((
                        time_left.saturating_sub(min_cost),
                        open.clone(),
                        (
                            next_a.clone(),
                            cost - min_cost,
                            b.clone(),
                            b_cost.saturating_sub(min_cost),
                        ),
                        pressure + build_up * min_cost.min(*time_left),
                    ));
                }
                step_with_elephant(next, valves, costs, max)
            } else if b_cost == &0 && !possible.is_empty() {
                let mut next = vec![];
                for next_b in possible {
                    let cost = costs.get(&(b, next_b)).expect("Cost not found") + 1;
                    let min_cost = if b_cost == &0 {
                        cost
                    } else {
                        cost.min(*b_cost)
                    };
                    next.push((
                        time_left.saturating_sub(min_cost),
                        open.clone(),
                        (
                            a.clone(),
                            a_cost.saturating_sub(min_cost),
                            next_b.clone(),
                            cost - min_cost,
                        ),
                        pressure + build_up * min_cost.min(*time_left),
                    ));
                }
                step_with_elephant(next, valves, costs, max)
            } else {
                let min_cost = if a_cost > &0 && b_cost > &0 {
                    a_cost.min(b_cost)
                } else {
                    a_cost.max(b_cost)
                };
                step_with_elephant(
                    vec![(
                        time_left.saturating_sub(*min_cost),
                        open.clone(),
                        (
                            a.clone(),
                            a_cost.saturating_sub(*min_cost),
                            b.clone(),
                            b_cost.saturating_sub(*min_cost),
                        ),
                        pressure + build_up * min_cost.min(time_left),
                    )],
                    valves,
                    costs,
                    max,
                )
            }
        })
        .collect()
}

/*
fn step_with_elephant<'a>(
    states: Vec<(u64, Vec<Valve>, (Valve, u64, Valve, u64), u64)>,
    valves: &'a HashMap<String, Valve>,
    costs: &'a HashMap<(&'a Valve, &'a Valve), u64>,
) -> Vec<(u64, Vec<Valve>, (Valve, u64, Valve, u64), u64)> {
    let next = states
        .iter()
        .flat_map(
            |(time_left, open, (target, target_cost, elephant, elephant_cost), pressure)| {
                if open.len() == valves.values().filter(|v| v.rate > 0).count() {
                    // if open.iter().map(|v| v.name.to_string()).collect_vec()
                    //     == vec!["DD", "JJ", "BB", "HH", "CC", "EE"]
                    // {
                    //     println!("LUUU: {} {:?}", time_left, pressure);
                    // }
                    // All valves open
                    return vec![(
                        0,
                        open.clone(),
                        (
                            target.clone(),
                            *target_cost,
                            elephant.clone(),
                            *elephant_cost,
                        ),
                        pressure + open.iter().map(|v| v.rate).sum::<u64>() * time_left,
                    )];
                }
                let mut states = vec![];
                let build_up = open.iter().map(|v| v.rate).sum::<u64>();
                let mut open = open.clone();
                if target_cost == &0 && target.rate > 0 && !open.contains(target) {
                    open.push((*target).clone());
                }
                if elephant_cost == &0 && elephant.rate > 0 && !open.contains(elephant) {
                    open.push((*elephant).clone());
                }

                let remaining = valves
                    .values()
                    .filter(|v| v.rate > 0 && !open.contains(v) && v != &target && v != &elephant)
                    .collect::<HashSet<_>>();

                if target_cost == &0 && elephant_cost == &0 && remaining.len() > 1 {
                    // Both have reached their destination
                    states.extend(remaining.iter().tuple_combinations().flat_map(|(&a, &b)| {
                        let target_a_cost =
                            costs.get(&(target, a)).expect("target_a_cost not found");
                        let target_b_cost =
                            costs.get(&(target, b)).expect("target_b_cost not found");
                        let elephant_a_cost = costs
                            .get(&(elephant, a))
                            .expect("elephant_a_cost not found");
                        let elephant_b_cost = costs
                            .get(&(elephant, b))
                            .expect("elephant_b_cost not found");
                        [
                            (
                                0,
                                open.clone(),
                                (a.clone(), target_a_cost - 1, b.clone(), elephant_b_cost - 1),
                                pressure + build_up,
                            ),
                            (
                                0,
                                open.clone(),
                                (b.clone(), target_b_cost - 1, a.clone(), elephant_a_cost - 1),
                                pressure + build_up,
                            ),
                        ]
                    }))
                } else if target_cost == &0 && !remaining.is_empty() {
                    states.extend(remaining.iter().map(|&v| {
                        let cost = costs.get(&(target, v)).expect("Cost not found");
                        (
                            0,
                            open.clone(),
                            (
                                v.clone(),
                                cost - 1,
                                elephant.clone(),
                                elephant_cost.saturating_sub(1),
                            ),
                            pressure + build_up,
                        )
                    }))
                } else if elephant_cost == &0 && !remaining.is_empty() {
                    states.extend(remaining.iter().map(|&v| {
                        let cost = costs.get(&(elephant, v)).expect("Cost not found");
                        (
                            0,
                            open.clone(),
                            (
                                target.clone(),
                                target_cost.saturating_sub(1),
                                v.clone(),
                                cost - 1,
                            ),
                            pressure + build_up,
                        )
                    }))
                } else {
                    states.push((
                        0,
                        open.clone(),
                        (
                            target.clone(),
                            target_cost.saturating_sub(1),
                            elephant.clone(),
                            elephant_cost.saturating_sub(1),
                        ),
                        pressure + build_up,
                    ));
                }
                states
            },
        )
        .collect_vec()
}

fn build_possibilities<'a>(
    valves: &'a HashMap<String, Valve>,
    costs: &'a HashMap<(&'a Valve, &'a Valve), u64>,
) {
    let start = valves.get("AA").expect("AA not found");
    let prospects = valves
        .values()
        .filter(|v| v.rate > 0)
        .cloned()
        .collect::<BTreeSet<_>>();
    let path = dijkstra(
        &(start.clone(), 0, start.clone(), 0, prospects),
        |(a, a_cost, b, b_cost, remaining)| {
            let mut next = vec![];
            let mut remaining = remaining.clone();
            if a_cost == &0 {
                remaining.remove(a);
            }
            if b_cost == &0 {
                remaining.remove(b);
            }
            let possible = remaining
                .iter()
                .filter(|v| v != &a && v != &b)
                .collect_vec();
            if a_cost == &0 && b_cost == &0 && possible.len() > 1 {
                for (next_a, next_b) in possible.iter().tuple_combinations() {
                    let a_to_a = costs.get(&(a, next_a)).expect("Cost not found");
                    let a_to_b = costs.get(&(a, next_b)).expect("Cost not found");
                    let b_to_a = costs.get(&(b, next_a)).expect("Cost not found");
                    let b_to_b = costs.get(&(b, next_b)).expect("Cost not found");

                    let min_cost = a_to_a.min(b_to_b);
                    next.push((
                        (
                            (*next_a).clone(),
                            a_to_a - min_cost,
                            (*next_b).clone(),
                            b_to_b - min_cost,
                            remaining.clone(),
                        ),
                        *min_cost,
                    ));
                    let min_cost = a_to_b.min(b_to_a);
                    next.push((
                        (
                            (*next_b).clone(),
                            a_to_b - min_cost,
                            (*next_a).clone(),
                            b_to_a - min_cost,
                            remaining.clone(),
                        ),
                        *min_cost,
                    ));
                }
            } else if a_cost == &0 && !possible.is_empty() {
                for next_a in possible {
                    let cost = costs.get(&(a, next_a)).expect("Cost not found");
                    let min_cost = if b_cost == &0 { cost } else { cost.min(b_cost) };
                    next.push((
                        (
                            next_a.clone(),
                            cost - min_cost,
                            b.clone(),
                            b_cost.saturating_sub(*min_cost),
                            remaining.clone(),
                        ),
                        *min_cost,
                    ));
                }
            } else if b_cost == &0 && !possible.is_empty() {
                for next_b in possible {
                    let cost = costs.get(&(b, next_b)).expect("Cost not found");
                    let min_cost = if a_cost == &0 { cost } else { cost.min(a_cost) };
                    next.push((
                        (
                            a.clone(),
                            a_cost.saturating_sub(*min_cost),
                            next_b.clone(),
                            cost - min_cost,
                            remaining.clone(),
                        ),
                        *min_cost,
                    ));
                }
            } else {
                let min_cost = a_cost.min(b_cost);
                next.push((
                    (
                        a.clone(),
                        a_cost.saturating_sub(*min_cost),
                        b.clone(),
                        b_cost.saturating_sub(*min_cost),
                        remaining.clone(),
                    ),
                    *min_cost,
                ));
            }
            next
        },
        |t| t.4.is_empty(),
    );
    println!("{:?}", path);
    // [AA, JJ, BB, CC]
    // [AA, DD, HH, EE]
}
*/

pub fn part1(input: String) {
    let valves: HashMap<String, Valve> = input
        .lines()
        .flat_map(Valve::from)
        .map(|v| (v.name.clone(), v))
        .collect();
    let costs = valves
        .values()
        .tuple_combinations()
        .fold(HashMap::new(), |mut map, (a, b)| {
            let cost = dijkstra(
                a,
                |v| {
                    v.tunnels
                        .iter()
                        .filter_map(|n| valves.get(n).map(|v| (v.clone(), 1)))
                        .collect_vec()
                },
                |v| v == b,
            )
            .expect("Path not found");
            map.insert((a, b), cost.1);
            map.insert((b, a), cost.1);
            map
        });
    println!("Part 1");
    step(vec![(30, HashSet::new(), "AA", 0)], &valves, &costs)
        .iter()
        .map(|s| s.3)
        .max()
        .expect("Max pressure not found")
        .print();

    // Part 2
    let aa = valves.get("AA").expect("AA valve not found");
    let mut max = 0;
    step_with_elephant(
        vec![(26, BTreeSet::new(), (aa.clone(), 0, aa.clone(), 0), 0)],
        &valves,
        &costs,
        &mut max,
    )
    .iter()
    .map(|s| s.3)
    .max()
    .expect("Max pressure not found")
    .print();

    /*
    I went through a lot of different iterations of solutions. Dynamic Programming is
    not a strong suit. Ultimately, I had the above `step_with_elephant` function, which
    works with the sample, but runs out of memory with the actual input. So I added a print
    with the "Current Max" that displays the current max as it runs through its calculations.
    Interestingly, the program runs through random paths each time since I'm using sets and
    iterating combinations to figure out the next interesting valve to visit for each visitor
    so I kept running the program until it ran out of memory multiple times keeping track of a
    global max until I found the answer. The different approaches I had involved trying to find
    a minimum spanning tree, but that didn't quite work for the example, since it passes a valve
    instead of opening it. I left all that work commented out above.
     */
}
