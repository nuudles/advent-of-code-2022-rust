use crate::{parse_nums::parse_nums, selfprint::SelfPrint};

pub fn part1(input: String) {
    let cycles = [20, 60, 100, 140, 180, 220];
    let mut pixels = [[false; 40]; 6];
    let mut y = 0;
    input
        .lines()
        .flat_map(|l| {
            if l == "noop" {
                vec![0]
            } else {
                vec![0, parse_nums::<i64>(l).next().unwrap_or_default()]
            }
        })
        .enumerate()
        .fold((0, 1), |(mut sum, mut x), (i, v)| {
            if cycles.contains(&(i + 1)) {
                sum += (i as i64 + 1) * x;
            }
            if (x - 1..=x + 1).contains(&((i as i64) % 40)) {
                pixels[y][i % 40] = true;
            } else {
                pixels[y][i % 40] = false;
            }
            if i % 40 == 39 {
                y += 1;
            }
            x += v;
            (sum, x)
        })
        .0
        .print();
    for row in pixels {
        for pixel in row {
            if pixel {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
