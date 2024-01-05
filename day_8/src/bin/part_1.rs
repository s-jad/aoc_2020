use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> i32 {
    let mut ops = input
        .lines()
        .map(|l| {
            let parts: (&str, &str) = l.split_whitespace().collect_tuple().unwrap();

            let amount: i32 = parts.1.parse::<i32>().unwrap();
            (parts.0, amount, false)
        })
        .collect_vec();

    let mut op_idx = 0;
    let mut current_op = ops[op_idx];
    let mut total = 0;

    while !current_op.2 {
        println!("current_op => {:?}", current_op);
        match current_op.0 {
            "nop" => {
                ops[op_idx].2 = true;
                op_idx += 1;
                current_op = ops[op_idx];
            }
            "acc" => {
                ops[op_idx].2 = true;
                op_idx += 1;
                total += current_op.1;
                current_op = ops[op_idx];
            }
            "jmp" => {
                ops[op_idx].2 = true;
                op_idx = (op_idx as i32 + current_op.1) as usize;
                current_op = ops[op_idx];
            }
            _ => unreachable!(),
        }
    }

    total
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
