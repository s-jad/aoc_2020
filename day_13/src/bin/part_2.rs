use itertools::Itertools;
use std::time::Instant;

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = extended_gcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inverse(a: i64, m: i64) -> i64 {
    let (g, x, _) = extended_gcd(a, m);
    if g != 1 {
        panic!("Inverse does not exist");
    } else {
        (x % m + m) % m
    }
}

fn crt(ids_offsets: &[(i64, i64)]) -> i64 {
    let prod: i64 = ids_offsets.iter().map(|(id, _)| id).product();

    let mut sum: i64 = 0;
    for &(id, offset) in ids_offsets {
        let p = prod / id;
        let inv = mod_inverse(p, id);
        sum = (sum + offset * inv * p) % prod;
    }

    sum
}

fn process(input: &str) -> i64 {
    let mut data = input.lines();

    let buses = data.nth(1).unwrap().split(',').collect_vec();

    let b = buses
        .into_iter()
        .enumerate()
        .filter_map(|(i, b)| {
            b.parse::<i64>()
                .ok()
                .map(|id| (id, (-(i as i64)).rem_euclid(id)))
        })
        .collect_vec();

    let timestamp = crt(&b);

    timestamp
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
