use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn process(input: &str) -> usize {
    let nums = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect_vec();

    let (mut checked, rest) = (nums[0..25].to_vec(), nums[25..].to_vec());

    for i in 0..rest.len() {
        let possible = &checked
            .iter()
            .cartesian_product(&checked)
            .filter(|(n1, n2)| n1 != n2)
            .map(|(n1, n2)| n1 + n2)
            .collect::<HashSet<usize>>();

        if possible.contains(&rest[i]) {
            checked = nums[0 + i + 1..25 + i + 1].to_vec();
        } else {
            return rest[i];
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
