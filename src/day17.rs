use std::collections::HashSet;

use itertools::Itertools;

use crate::point::Point;

pub fn part1(input: String) {
    let mut directions = input.chars().cycle();
    let pieces = [
        vec![
            Point { x: 3i64, y: 0 },
            Point { x: 4, y: 0 },
            Point { x: 5, y: 0 },
            Point { x: 6, y: 0 },
        ],
        vec![
            Point { x: 4, y: 0 },
            Point { x: 3, y: -1 },
            Point { x: 4, y: -1 },
            Point { x: 5, y: -1 },
            Point { x: 4, y: -2 },
        ],
        vec![
            Point { x: 3, y: 0 },
            Point { x: 4, y: 0 },
            Point { x: 5, y: 0 },
            Point { x: 5, y: -1 },
            Point { x: 5, y: -2 },
        ],
        vec![
            Point { x: 3, y: 0 },
            Point { x: 3, y: -1 },
            Point { x: 3, y: -2 },
            Point { x: 3, y: -3 },
        ],
        vec![
            Point { x: 3, y: 0 },
            Point { x: 4, y: 0 },
            Point { x: 3, y: -1 },
            Point { x: 4, y: -1 },
        ],
    ];
    let mut piece_iter = pieces.iter().cycle();
    let mut last_min_y = 0;
    let mut last_cycle_min_y = i64::MAX;
    let mut last_cycle_i = i64::MAX;
    let mut min_y = 0;
    let mut rock: HashSet<Point<i64>> = HashSet::new();
    let mut deltas = vec![];
    for i in 0..1000000000 {
        let mut piece = piece_iter.next().expect("No piece found?!").clone();
        for part in piece.iter_mut() {
            part.y += min_y - 4;
        }
        // println!("{:?}", piece);
        let mut next = piece.clone();
        loop {
            // for y in min_y - 10..=0 {
            //     for x in 0..=8 {
            //         if y == 0 {
            //             print!("-");
            //         } else if x == 0 || x == 8 {
            //             print!("|");
            //         } else if rock.contains(&Point { x, y }) {
            //             print!("#");
            //         } else if next.contains(&Point { x, y }) {
            //             print!("@");
            //         } else {
            //             print!(".");
            //         }
            //     }
            //     println!();
            // }
            let flow = directions.next().expect("Direction not found!?");
            if next.iter().all(|p| {
                (flow == '<' && p.x > 1 && !rock.contains(&p.left()))
                    || (flow == '>' && p.x < 7 && !rock.contains(&p.right()))
            }) {
                for part in next.iter_mut() {
                    part.x += if flow == '<' { -1 } else { 1 };
                }
            }
            if next.iter().all(|p| !rock.contains(&p.down()) && p.y < -1) {
                for part in next.iter_mut() {
                    part.y += 1;
                }
            } else {
                for part in next {
                    rock.insert(part);
                    min_y = min_y.min(part.y);
                }
                break;
            }
        }
        if i == 2021 {
            println!("Part 1: {}", min_y.abs());
        }
        if i > 446 {
            // Manually played around with the i > value here in order to find when the
            // cycle starts to get the algorithm to work below
            deltas.push(last_min_y - min_y);
            if deltas.len() > 1735 * 2 {
                let first = deltas.iter().take(1735).collect_vec();
                let last = deltas.iter().rev().take(1735).rev().collect_vec();
                if first == last {
                    println!(
                        "Cycle detected! {} {} {}",
                        i,
                        last_min_y - min_y,
                        min_y.abs()
                    );
                    if last_cycle_i != i64::MAX {
                        let period = i - last_cycle_i;
                        println!("Period: {}", period);
                        let i = i + 1;
                        let desired = 1000000000000i64;
                        let repeats = (desired - i) / period;
                        let mut height =
                            min_y.abs() + (min_y.abs() - last_cycle_min_y.abs()) * repeats;
                        for delta in deltas
                            .iter()
                            .take((desired - (repeats * period + i)) as usize)
                        {
                            height += delta;
                        }
                        println!("Part 2: {}", height);
                        break;
                    } else {
                        last_cycle_i = i;
                        last_cycle_min_y = min_y;
                    }
                }
            }
        }
        last_min_y = min_y;
    }
}
