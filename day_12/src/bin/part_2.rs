use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let dir = input
        .lines()
        .map(|l| {
            let (d, n) = l.split_at(1);
            (d, n.parse::<i32>().unwrap())
        })
        .collect_vec();

    let mut waypoint = (10, 1);

    let (x, y) = dir
        .into_iter()
        .fold((0, 0), move |(x, y), (op, amount)| match op {
            "N" => {
                waypoint = (waypoint.0, waypoint.1 + amount);
                (x, y)
            }
            "S" => {
                waypoint = (waypoint.0, waypoint.1 - amount);
                (x, y)
            }
            "E" => {
                waypoint = (waypoint.0 + amount, waypoint.1);
                (x, y)
            }
            "W" => {
                waypoint = (waypoint.0 - amount, waypoint.1);
                (x, y)
            }
            "L" => {
                let rotate = amount / 90;

                for _ in 0..rotate {
                    waypoint = (-waypoint.1, waypoint.0);
                }

                (x, y)
            }
            "R" => {
                let rotate = amount / 90;

                for _ in 0..rotate {
                    waypoint = (waypoint.1, -waypoint.0);
                }

                (x, y)
            }
            "F" => (x + (waypoint.0 * amount), y + (waypoint.1 * amount)),
            _ => unreachable!(),
        });

    x.abs() as usize + y.abs() as usize
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
