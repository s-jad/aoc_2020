use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let b_passes = input.lines().collect_vec();

    let mut highest_score = 0;

    for pass in b_passes {
        let row_score = pass[0..6]
            .chars()
            .fold(0..127, |range, c| {
                let midpoint = (range.start + range.end) / 2;

                match c {
                    'F' => range.start..midpoint,
                    'B' => midpoint..range.end,
                    _ => unreachable!(),
                }
            })
            .start
            + 1;

        let col_score = pass[7..]
            .chars()
            .fold(0..7, |range, c| {
                let midpoint = (range.start + range.end) / 2;

                match c {
                    'L' => range.start..midpoint,
                    'R' => midpoint..range.end,
                    _ => unreachable!(),
                }
            })
            .start
            + 1;

        if (row_score * 8) + col_score > highest_score {
            highest_score = (row_score * 8) + col_score;
        }
    }

    highest_score
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
