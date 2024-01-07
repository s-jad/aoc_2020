use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn process(input: &str) -> usize {
    let starting_nums = input
        .split_terminator(&[',', '\n'][..])
        .filter_map(|s| s.parse::<usize>().ok())
        .collect_vec();

    let last_round = 2020usize;
    let mut already_spoken: HashSet<usize> = HashSet::new();
    let mut turn_spoken = vec![];

    for i in 0..starting_nums.len() - 1 {
        already_spoken.insert(starting_nums[i]);
        turn_spoken.push((i + 1, starting_nums[i]));
    }

    let mut ln = *starting_nums.last().unwrap();

    for i in starting_nums.len()..last_round {
        if already_spoken.get(&ln).is_some() {
            let idx = turn_spoken.iter().position(|(_, n)| n == &ln).unwrap();
            ln = i - turn_spoken[idx].0;
            turn_spoken[idx].0 = i;
        } else {
            already_spoken.insert(ln);
            turn_spoken.push((i, ln));
            ln = 0;
        }
    }

    ln
}

fn main() {
    let input = include_str!("../../input.txt");

    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();

    dbg!(output, time);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("../../test.txt");
        let output = process(input);
        assert_eq!(result,);
    }
}
