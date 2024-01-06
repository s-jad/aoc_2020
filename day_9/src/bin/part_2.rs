use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let nums = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect_vec();

    let target = 70639851;

    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            let possible = &nums[i..j];

            if possible.iter().sum::<usize>() == target {
                let max = possible.iter().max().unwrap();
                let min = possible.iter().min().unwrap();

                return max + min;
            }
        }
    }

    0
}

fn main() {
    let input = include_str!("../../input.txt");

    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();

    dbg!(output, time);
}
