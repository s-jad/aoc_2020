use itertools::Itertools;
use std::time::Instant;

fn check_validity(pol: (usize, usize, char), pass: &str) -> usize {
    let key = pol.2;
    let key_pos = pass.chars().positions(|c| c == key).collect_vec();

    if key_pos.contains(&pol.0) && !(key_pos.contains(&pol.0) && key_pos.contains(&pol.1))
        || key_pos.contains(&pol.1) && !(key_pos.contains(&pol.0) && key_pos.contains(&pol.1))
    {
        return 1;
    }

    return 0;
}

fn process(input: &str) -> usize {
    let passwords = input
        .lines()
        .map(|l| {
            let (a, pass) = l.split(':').collect_tuple().unwrap();

            let p = a.split_terminator(&[' ', '-'][..]).collect_vec();

            let pol = (
                p[0].parse::<usize>().unwrap(),
                p[1].parse::<usize>().unwrap(),
                p[2].chars().next().unwrap(),
            );

            (pol, pass)
        })
        .collect_vec();

    passwords
        .into_iter()
        .map(|(pol, pass)| check_validity(pol, pass))
        .sum()
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
