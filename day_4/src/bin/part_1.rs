use itertools::Itertools;
use std::time::Instant;

fn check_validity(p: &str) -> usize {
    let count = p
        .split(":")
        .flat_map(|s| s.split_whitespace())
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .count();

    if count == 8 {
        return 1;
    } else if count == 7 && !p.contains("cid") {
        return 1;
    } else {
        return 0;
    }
}

fn process(input: &str) -> usize {
    let paras = input.split_terminator("\n\n").collect_vec();

    let fields = paras.into_iter().map(|p| check_validity(p)).sum();

    fields
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
