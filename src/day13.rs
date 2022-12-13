use std::cmp::Ordering;

use itertools::Itertools;

use crate::selfprint::SelfPrint;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Node {
    Number(u64),
    List(Vec<Node>),
}

impl Node {
    fn from(string: &str) -> Self {
        let mut value = u64::MAX;
        let mut lists = vec![];
        for c in string.chars() {
            if c.is_numeric() {
                if value == u64::MAX {
                    value = c as u64 - '0' as u64;
                } else {
                    value = value * 10 + (c as u64 - '0' as u64);
                }
            } else if c == '[' {
                lists.push(vec![]);
            } else if c == ']' {
                let mut list = lists.pop().expect("No list found");
                if value != u64::MAX {
                    list.push(Node::Number(value));
                    value = u64::MAX;
                }
                if let Some(parent) = lists.last_mut() {
                    parent.push(Self::List(list));
                } else {
                    return Self::List(list);
                }
            } else if c == ',' && value != u64::MAX {
                lists
                    .last_mut()
                    .expect("Comma without a list?!")
                    .push(Node::Number(value));
                value = u64::MAX;
            }
        }
        Self::Number(value)
    }

    fn is_right_order(&self, other: &Node) -> Option<bool> {
        match (self, other) {
            (Node::Number(a), Node::Number(b)) => match a.cmp(b) {
                std::cmp::Ordering::Less => Some(true),
                std::cmp::Ordering::Greater => Some(false),
                std::cmp::Ordering::Equal => None,
            },
            (Node::List(a), Node::List(b)) => {
                for i in 0..a.len() {
                    if i >= b.len() {
                        return Some(false);
                    }
                    if let Some(b) = a[i].is_right_order(&b[i]) {
                        return Some(b);
                    }
                }
                if b.len() > a.len() {
                    Some(true)
                } else {
                    None
                }
            }
            (Node::List(_), Node::Number(b)) => {
                self.is_right_order(&Node::List(vec![Node::Number(*b)]))
            }
            (Node::Number(a), Node::List(_)) => {
                Node::List(vec![Node::Number(*a)]).is_right_order(other)
            }
        }
    }
}

pub fn part1(input: String) {
    input
        .split("\n\n")
        .enumerate()
        .map(|(i, group)| {
            let (a, b) = group.split_once('\n').expect("Could not parse pairs");
            if Node::from(a)
                .is_right_order(&Node::from(b))
                .expect("Could not determine order")
            {
                i + 1
            } else {
                0
            }
        })
        .sum::<usize>()
        .print();
}

pub fn part2(input: String) {
    let dividers = [
        Node::List(vec![Node::List(vec![Node::Number(6)])]),
        Node::List(vec![Node::List(vec![Node::Number(2)])]),
    ];
    input
        .split("\n\n")
        .flat_map(|group| {
            let (a, b) = group.split_once('\n').expect("Could not parse pairs");
            vec![Node::from(a), Node::from(b)]
        })
        .chain(dividers.iter().cloned())
        .sorted_by(|a, b| {
            if a.is_right_order(b).unwrap_or(false) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
        .enumerate()
        .filter(|(_, n)| dividers.contains(n))
        .map(|(i, _)| i + 1)
        .product::<usize>()
        .print();
}
