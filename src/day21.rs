use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone)]
enum Monkey<'a> {
    Number(u64),
    Operation(&'a str, &'a str, &'a str),
}

impl Monkey<'_> {
    fn from(str: &str) -> Option<(&str, Monkey)> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([a-z]+): ((\d+)|([a-z]+) ([+\-*/]) ([a-z]+))")
                .expect("Invalid Regex");
        }
        let captures = RE.captures(str)?;
        if let Some(number) = captures.get(3) {
            Some((
                captures.get(1)?.as_str(),
                Monkey::Number(number.as_str().parse().ok()?),
            ))
        } else {
            Some((
                captures.get(1)?.as_str(),
                Monkey::Operation(
                    captures.get(4)?.as_str(),
                    captures.get(5)?.as_str(),
                    captures.get(6)?.as_str(),
                ),
            ))
        }
    }
}

fn monkey_value(name: &str, monkeys: &HashMap<&str, Monkey>) -> Option<u64> {
    match monkeys.get(name)? {
        Monkey::Number(value) => Some(*value),
        Monkey::Operation(a, operation, b) => {
            let a_value = monkey_value(a, monkeys)?;
            let b_value = monkey_value(b, monkeys)?;
            match *operation {
                "-" => Some(a_value - b_value),
                "/" => Some(a_value / b_value),
                "*" => Some(a_value * b_value),
                _ => Some(a_value + b_value),
            }
        }
    }
}

pub fn part1(input: String) {
    let monkeys: HashMap<_, _> = input.lines().filter_map(Monkey::from).collect();
    println!(
        "{}",
        monkey_value("root", &monkeys).expect("Root value not found")
    );
}

fn monkey_equation(name: &str, monkeys: &HashMap<&str, Monkey>) -> String {
    if let Some(value) = monkey_value(name, monkeys) {
        value.to_string()
    } else if name != "humn" {
        match monkeys.get(name).expect("Monkey not found") {
            Monkey::Number(value) => value.to_string(),
            Monkey::Operation(a, operation, b) => format!(
                "({} {} {})",
                monkey_equation(a, monkeys),
                operation,
                monkey_equation(b, monkeys)
            ),
        }
    } else {
        "humn".to_string()
    }
}

pub fn part2(input: String) {
    let monkeys: HashMap<_, _> = input
        .lines()
        .filter_map(Monkey::from)
        .filter(|t| t.0 != "humn")
        .collect();
    let root = monkeys.get("root").expect("Could not found root");
    let (a, b) = match root {
        Monkey::Number(_) => ("", ""),
        Monkey::Operation(a, _, b) => (*a, *b),
    };
    println!("{}", monkey_equation(a, &monkeys));
    println!("{}", monkey_equation(b, &monkeys));
    // Printed out the equations and then used an online equation solver to solve for "humn"
}
