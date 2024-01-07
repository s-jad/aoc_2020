use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let mut data = input.lines();

    let dep = data.next().unwrap().parse::<usize>().unwrap();
    let bus = data
        .next()
        .unwrap()
        .split(",")
        .into_iter()
        .filter(|s| s != &"x")
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();

    let mut earliest_possible = 0;
    let mut idx = dep;
    let mut check = vec![];

    while earliest_possible == 0 {
        idx += 1;

        check = bus.iter().filter(|&&b| idx % b == 0).collect_vec();

        if check.len() > 0 {
            earliest_possible = *check[0];
        }
    }

    earliest_possible * (idx - dep)
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
