use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let lines = input.lines().collect_vec();
    let line_length = lines[0].len();

    let mut tree_count = 0;

    for i in 1..lines.len() {
        let j = (i * 3) % line_length;
        if lines[i].chars().nth(j).unwrap() == '#' {
            tree_count += 1;
        }
    }

    tree_count
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
