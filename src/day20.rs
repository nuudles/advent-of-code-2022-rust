use std::collections::VecDeque;

use itertools::Itertools;

use crate::parse_nums::parse_nums;

fn mix(list: &mut Vec<(usize, i64)>) {
    let len = list.len();
    let mut indices: VecDeque<_> = (0..len)
        .filter_map(|i| list.iter().position(|t| t.0 == i))
        .collect::<VecDeque<_>>();
    while let Some(index) = indices.pop_front() {
        let new_index = (list[index].1 + index as i64).rem_euclid(len as i64 - 1) as usize;
        let dropped = list.remove(index);
        list.insert(new_index, dropped);
        for i in indices.iter_mut() {
            if new_index < index && *i >= new_index && *i <= index {
                *i += 1;
            } else if new_index > index && *i >= index && *i <= new_index {
                *i -= 1;
            }
        }
    }
}

pub fn part1(input: String) {
    let mut list = input
        .lines()
        .flat_map(|l| parse_nums::<i64>(l).next())
        .enumerate()
        .collect_vec();
    mix(&mut list);
    let zero_pos = list
        .iter()
        .position(|t| t.1 == 0)
        .expect("Could not find 0");
    println!(
        "{}",
        list[(zero_pos + 1000) % list.len()].1
            + list[(zero_pos + 2000) % list.len()].1
            + list[(zero_pos + 3000) % list.len()].1
    );
}

pub fn part2(input: String) {
    let mut list = input
        .lines()
        .flat_map(|l| parse_nums::<i64>(l).next())
        .map(|x| x * 811589153)
        .enumerate()
        .collect_vec();
    for _ in 0..10 {
        mix(&mut list);
    }

    let zero_pos = list
        .iter()
        .position(|t| t.1 == 0)
        .expect("Could not find 0");
    println!(
        "{}",
        list[(zero_pos + 1000) % list.len()].1
            + list[(zero_pos + 2000) % list.len()].1
            + list[(zero_pos + 3000) % list.len()].1
    );
}
