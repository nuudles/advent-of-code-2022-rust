use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

use crate::point::Point;

pub fn part1(input: String) {
    let (map, instructions) = input.split_once("\n\n").expect("Could not split input");

    let mut position = Point {
        x: i64::MIN,
        y: i64::MIN,
    };
    let tiles = map
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut map, (y, line)| {
            for (x, c) in line.chars().enumerate().filter(|(_, c)| c != &' ') {
                if position.x == i64::MIN {
                    position = Point {
                        x: x as i64 + 1,
                        y: y as i64 + 1,
                    };
                }
                map.insert(
                    Point {
                        x: x as i64 + 1,
                        y: y as i64 + 1,
                    },
                    c,
                );
            }
            map
        });
    let regex = Regex::new(r#"(\d+)|(L|R)"#).expect("Invalid regex");
    let directions = [
        Point { x: 1, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: -1, y: 0 },
        Point { x: 0, y: -1 },
    ];
    let mut dir_index = 0;
    for instruction in regex.find_iter(instructions).map(|m| m.as_str()) {
        match instruction {
            "L" => {
                dir_index = if dir_index == 0 {
                    directions.len() - 1
                } else {
                    dir_index - 1
                };
            }
            "R" => {
                dir_index = (dir_index + 1) % directions.len();
            }
            _ => {
                let count = instruction.parse::<i64>().expect("Could not parse count");
                for _ in 0..count {
                    let mut next = position + directions[dir_index];
                    if !tiles.contains_key(&next) {
                        next = match dir_index {
                            // Right
                            0 => *tiles
                                .keys()
                                .filter(|p| p.y == position.y)
                                .sorted_by_key(|p| p.x)
                                .next()
                                .expect("Wrap around right not found"),
                            // Down
                            1 => *tiles
                                .keys()
                                .filter(|p| p.x == position.x)
                                .sorted_by_key(|p| p.y)
                                .next()
                                .expect("Wrap around down not found"),
                            // Left
                            2 => *tiles
                                .keys()
                                .filter(|p| p.y == position.y)
                                .sorted_by_key(|p| p.x)
                                .rev()
                                .next()
                                .expect("Wrap around left not found"),
                            // Up
                            _ => *tiles
                                .keys()
                                .filter(|p| p.x == position.x)
                                .sorted_by_key(|p| p.y)
                                .rev()
                                .next()
                                .expect("Wrap around up not found"),
                        };
                    }
                    let tile = tiles.get(&next).expect("Should have a tile by now");
                    if tile == &'#' {
                        break;
                    }
                    position = next;
                }
            }
        }
    }
    println!("{}", 1000 * position.y + 4 * position.x + dir_index as i64);
}

fn wrap(position: &Point<i64>, dir_index: usize) -> (Point<i64>, usize) {
    match dir_index {
        // Right
        0 => {
            if (151..=200).contains(&position.y) {
                // Right C = Up C
                return (
                    Point {
                        x: position.y - 150 + 50,
                        y: 150,
                    },
                    3,
                );
            } else if (51..=100).contains(&position.y) {
                // Right D = Up D
                return (
                    Point {
                        x: position.y - 50 + 100,
                        y: 50,
                    },
                    3,
                );
            } else if (1..=50).contains(&position.y) {
                // Right E = Left E
                return (
                    Point {
                        x: 100,
                        y: (51 - position.y) + 100,
                    },
                    2,
                );
            } else if (101..=150).contains(&position.y) {
                // Right E = Left E
                return (
                    Point {
                        x: 150,
                        y: 151 - position.y,
                    },
                    2,
                );
            }
        }
        // Down
        1 => {
            if (51..=100).contains(&position.x) {
                // Down C = Left C
                return (
                    Point {
                        x: 50,
                        y: position.x - 50 + 150,
                    },
                    2,
                );
            } else if (101..=150).contains(&position.x) {
                // Down D = Left D
                return (
                    Point {
                        x: 100,
                        y: position.x - 100 + 50,
                    },
                    2,
                );
            } else if (1..=50).contains(&position.x) {
                // Down F = Down F
                return (
                    Point {
                        x: position.x + 100,
                        y: 1,
                    },
                    1,
                );
            }
        }
        // Left
        2 => {
            if (51..=100).contains(&position.y) {
                // Left A = Down A
                return (
                    Point {
                        x: position.y - 50,
                        y: 101,
                    },
                    1,
                );
            } else if (1..=50).contains(&position.y) {
                // Left B = Right B
                return (
                    Point {
                        x: 1,
                        y: (51 - position.y) + 100,
                    },
                    0,
                );
            } else if (101..=150).contains(&position.y) {
                // Left B = Right B
                return (
                    Point {
                        x: 51,
                        y: 151 - position.y,
                    },
                    0,
                );
            } else if (151..=200).contains(&position.y) {
                // Left G = Down G
                return (
                    Point {
                        x: position.y - 150 + 50,
                        y: 1,
                    },
                    1,
                );
            }
        }
        // Up
        _ => {
            if (1..=50).contains(&position.x) {
                // Up A = Right A
                return (
                    Point {
                        x: 51,
                        y: position.x + 50,
                    },
                    0,
                );
            } else if (101..=150).contains(&position.x) {
                // Up F = Up F
                return (
                    Point {
                        x: position.x - 100,
                        y: 200,
                    },
                    3,
                );
            } else if (51..=100).contains(&position.x) {
                // Up G = Right G
                return (
                    Point {
                        x: 1,
                        y: position.x - 50 + 150,
                    },
                    0,
                );
            }
        }
    }
    panic!("We should never get here... I did something wrong")
}

pub fn part2(input: String) {
    let (map, instructions) = input.split_once("\n\n").expect("Could not split input");

    let mut position = Point {
        x: i64::MIN,
        y: i64::MIN,
    };
    let tiles = map
        .lines()
        .enumerate()
        .fold(HashMap::new(), |mut map, (y, line)| {
            for (x, c) in line.chars().enumerate().filter(|(_, c)| c != &' ') {
                if position.x == i64::MIN {
                    position = Point {
                        x: x as i64 + 1,
                        y: y as i64 + 1,
                    };
                }
                map.insert(
                    Point {
                        x: x as i64 + 1,
                        y: y as i64 + 1,
                    },
                    c,
                );
            }
            map
        });
    let regex = Regex::new(r#"(\d+)|(L|R)"#).expect("Invalid regex");
    let directions = [
        Point { x: 1, y: 0 },
        Point { x: 0, y: 1 },
        Point { x: -1, y: 0 },
        Point { x: 0, y: -1 },
    ];
    let mut dir_index = 0;
    for instruction in regex.find_iter(instructions).map(|m| m.as_str()) {
        match instruction {
            "L" => {
                dir_index = if dir_index == 0 {
                    directions.len() - 1
                } else {
                    dir_index - 1
                };
            }
            "R" => {
                dir_index = (dir_index + 1) % directions.len();
            }
            _ => {
                let count = instruction.parse::<i64>().expect("Could not parse count");
                for _ in 0..count {
                    let mut next = position + directions[dir_index];
                    let mut next_dir_index = dir_index;
                    if !tiles.contains_key(&next) {
                        (next, next_dir_index) = wrap(&position, dir_index);
                    }
                    let tile = tiles.get(&next).unwrap_or_else(|| {
                        panic!("Should have a tile by now {:?} {:?}", position, next)
                    });
                    if tile == &'#' {
                        break;
                    }
                    position = next;
                    dir_index = next_dir_index;
                }
            }
        }
    }
    println!("{}", 1000 * position.y + 4 * position.x + dir_index as i64);
}
