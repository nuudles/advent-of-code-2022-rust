use crate::selfprint::SelfPrint;

fn snafu_to_u64(snafu: &str) -> u64 {
    let mut digit = 1;
    let mut total = 0i64;
    for c in snafu.chars().rev() {
        total += digit
            * match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                _ => -2,
            };
        digit *= 5;
    }
    total as u64
}

fn u64_to_snafu(mut number: u64) -> String {
    let mut snafu = String::new();
    while number > 0 {
        let digit = number % 5;
        snafu.push(match digit {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            _ => '-',
        });
        number /= 5;
        if digit > 2 {
            number += 1;
        }
    }
    snafu.chars().rev().collect()
}

pub fn part1(input: String) {
    u64_to_snafu(input.lines().map(snafu_to_u64).sum::<u64>()).print();
}
