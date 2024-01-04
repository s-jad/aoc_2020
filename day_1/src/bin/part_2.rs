use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let nums = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect_vec();

    let triplets = nums
        .iter()
        .combinations(3)
        .filter(|c| c[0] + c[1] + c[2] == 2020usize)
        .next()
        .map(|c| c[0] * c[1] * c[2])
        .unwrap();

    triplets
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
