use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let mut adapters = input
        .lines()
        .map(|l| l.parse::<usize>().unwrap())
        .collect_vec();

    adapters.sort();

    let mut one_total = 0;
    let mut three_total = 0;
    let mut current_jolts = 0;

    for i in 0..adapters.len() {
        let diff = adapters[i] - current_jolts;

        match diff {
            1 => one_total += 1,
            2 => {}
            3 => three_total += 1,
            _ => unreachable!(),
        }

        current_jolts = adapters[i];
    }

    one_total * (three_total + 1)
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
