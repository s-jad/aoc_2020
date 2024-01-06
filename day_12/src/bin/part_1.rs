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

    let mut current_dir = "E";
    let compass = ["N", "E", "S", "W"];

    let (x, y) = dir
        .into_iter()
        .fold((0, 0), |(x, y), (dir, amount)| match dir {
            "N" => (x, y + amount),
            "S" => (x, y - amount),
            "E" => (x + amount, y),
            "W" => (x - amount, y),
            "L" => {
                let rotate = amount / 90;
                let c_idx = compass.iter().position(|&c| c == current_dir).unwrap();
                current_dir = compass[((c_idx as i32 + 4 - rotate) % 4) as usize];
                (x, y)
            }
            "R" => {
                let rotate = amount / 90;
                let c_idx = compass.iter().position(|&c| c == current_dir).unwrap();
                current_dir = compass[((c_idx as i32 + rotate) % 4) as usize];
                (x, y)
            }
            "F" => match current_dir {
                "N" => (x, y + amount),
                "S" => (x, y - amount),
                "E" => (x + amount, y),
                "W" => (x - amount, y),
                _ => unreachable!(),
            },
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
