use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let lines = input.lines().collect_vec();
    let line_length = lines[0].len();

    let right_steps = [1, 3, 5, 7, 1];
    let down_steps = [1, 1, 1, 1, 2];

    let mut product = 1;

    for s in 0..right_steps.len() {
        let right = right_steps[s];
        let down = down_steps[s];
        let mut tree_count = 0;

        for i in 1..lines.len() {
            let j = (i * right) % line_length;
            let k = i * down;
            if k < lines.len() {
                if lines[k].chars().nth(j).unwrap() == '#' {
                    tree_count += 1;
                }
            }
        }

        product *= tree_count;
    }

    product
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
