use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let rlen = input.lines().next().unwrap().len();

    let mut seats = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let neighbours: [(i32, i32); 8] = [
        (-1, 0),
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
        (0, -1),
    ];

    let mut num_changes = 1;

    while num_changes != 0 {
        num_changes = 0;
        let mut new_seats = vec![vec!['.'; rlen]; seats.len()];

        for i in 0..seats.len() {
            for j in 0..seats[0].len() {
                let env: Vec<char> = neighbours
                    .iter()
                    .filter_map(|&(dr, dc)| {
                        let new_row = i as i32 + dr;
                        let new_col = j as i32 + dc;
                        if new_row >= 0
                            && new_row < seats.len() as i32
                            && new_col >= 0
                            && new_col < rlen as i32
                        {
                            Some(seats[new_row as usize][new_col as usize])
                        } else {
                            None
                        }
                    })
                    .collect();

                if seats[i][j] == 'L' && !env.contains(&'#') {
                    new_seats[i][j] = '#';
                    num_changes += 1;
                } else if seats[i][j] == '#' && env.iter().filter(|&&s| s == '#').count() >= 4 {
                    new_seats[i][j] = 'L';
                    num_changes += 1;
                } else {
                    new_seats[i][j] = seats[i][j];
                }
            }
        }

        seats = new_seats;
    }

    seats
        .iter()
        .flat_map(|s| s.iter().filter(|&&s| s == '#').collect_vec())
        .count()
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
