use itertools::Itertools;

use crate::selfprint::SelfPrint;

type Operation = fn(u64) -> u64;
type Check = fn(u64) -> usize;

pub fn part1(_: String) {
    /*
    Didn't feel like parsing and there's only 8 monkeys, so just
    copied the logic into closures.
     */
    let mut stacks = [
        vec![96, 60, 68, 91, 83, 57, 85],
        vec![75, 78, 68, 81, 73, 99],
        vec![69, 86, 67, 55, 96, 69, 94, 85],
        vec![88, 75, 74, 98, 80],
        vec![82],
        vec![72, 92, 92],
        vec![74, 61],
        vec![76, 86, 83, 55],
    ];
    let monkeys: [(Operation, Check); 8] = [
        (|v| v * 2, |v| if v % 17 == 0 { 2 } else { 5 }),
        (|v| v + 3, |v| if v % 13 == 0 { 7 } else { 4 }),
        (|v| v + 6, |v| if v % 19 == 0 { 6 } else { 5 }),
        (|v| v + 5, |v| if v % 7 == 0 { 7 } else { 1 }),
        (|v| v + 8, |v| if v % 11 == 0 { 0 } else { 2 }),
        (|v| v * 5, |v| if v % 3 == 0 { 6 } else { 3 }),
        (|v| v * v, |v| if v % 2 == 0 { 3 } else { 1 }),
        (|v| v + 4, |v| if v % 5 == 0 { 4 } else { 0 }),
    ];
    let mut inspections = [0; 8];
    for _ in 0..20 {
        for i in 0..8 {
            for item in stacks[i].clone() {
                inspections[i] += 1;
                let worry = monkeys[i].0(item) / 3;
                let dest = monkeys[i].1(worry);
                stacks[dest].push(worry);
            }
            stacks[i] = vec![];
        }
    }
    inspections
        .iter()
        .sorted()
        .rev()
        .take(2)
        .product::<u64>()
        .print();
}

pub fn part2(_: String) {
    let mut stacks = [
        vec![96, 60, 68, 91, 83, 57, 85],
        vec![75, 78, 68, 81, 73, 99],
        vec![69, 86, 67, 55, 96, 69, 94, 85],
        vec![88, 75, 74, 98, 80],
        vec![82],
        vec![72, 92, 92],
        vec![74, 61],
        vec![76, 86, 83, 55],
    ];
    let monkeys: [(Operation, Check); 8] = [
        (|v| v * 2, |v| if v % 17 == 0 { 2 } else { 5 }),
        (|v| v + 3, |v| if v % 13 == 0 { 7 } else { 4 }),
        (|v| v + 6, |v| if v % 19 == 0 { 6 } else { 5 }),
        (|v| v + 5, |v| if v % 7 == 0 { 7 } else { 1 }),
        (|v| v + 8, |v| if v % 11 == 0 { 0 } else { 2 }),
        (|v| v * 5, |v| if v % 3 == 0 { 6 } else { 3 }),
        (|v| v * v, |v| if v % 2 == 0 { 3 } else { 1 }),
        (|v| v + 4, |v| if v % 5 == 0 { 4 } else { 0 }),
    ];
    let mut inspections = [0; 8];
    /*
    I spent a lot of time trying to make it work using the prime
    factorizations. Works fine collecting the prime factors when
    the worry factor is multiplied, but couldn't figure out a
    correlation after the worry factor is added to. I ended up
    reading the beginning of a spoiler thread and got the hint
    about the lcm.
     */
    let lcm = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;
    for _ in 0..10000 {
        for i in 0..8 {
            for item in stacks[i].clone() {
                inspections[i] += 1;
                let worry = monkeys[i].0(item) % lcm;
                let dest = monkeys[i].1(worry);
                stacks[dest].push(worry);
            }
            stacks[i] = vec![];
        }
    }
    inspections
        .iter()
        .sorted()
        .rev()
        .take(2)
        .product::<u64>()
        .print();
}
