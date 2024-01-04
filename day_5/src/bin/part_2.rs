use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn seat_id(p: &str) -> (i32, i32) {
    let mut r_low = 0;
    let mut r_high = 127;
    let mut c_low = 0;
    let mut c_high = 7;

    for c in p.chars() {
        match c {
            'F' => r_high = (r_high + r_low) / 2,
            'B' => r_low = (r_high + r_low) / 2,
            'L' => c_high = (c_high + c_low) / 2,
            'R' => c_low = (c_high + c_low) / 2,
            _ => unreachable!(),
        }
    }

    (r_high, c_high)
}

fn process(input: &str) -> i32 {
    let filled_seats = input.lines().map(|p| seat_id(p)).collect::<HashSet<_>>();

    (0..128)
        .cartesian_product(0..7)
        .filter(|pos| !filled_seats.contains(pos))
        .find(|&(r, c)| filled_seats.contains(&(r, c - 1)) && filled_seats.contains(&(r, c + 1)))
        .map(|(r, c)| r * 8 + c)
        .unwrap()
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
