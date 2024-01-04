use itertools::Itertools;
use regex::Regex;
use std::time::Instant;

fn check_count(p: &str) -> bool {
    let count = p
        .split(":")
        .flat_map(|s| s.split_whitespace())
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .count();

    if count == 8 {
        return true;
    } else if count == 7 && !p.contains("cid") {
        return true;
    } else {
        return false;
    }
}

fn check_field_validity(p: &str) -> bool {
    let fields = p
        .split_whitespace()
        .map(|s| s.split(":").collect_tuple::<(_, _)>().unwrap())
        .collect_vec();

    for (field, val) in fields {
        match field {
            "byr" => {
                if val.parse::<u32>().unwrap() < 1920 || val.parse::<u32>().unwrap() > 2002 {
                    return false;
                }
            }
            "iyr" => {
                if val.parse::<u32>().unwrap() < 2010 || val.parse::<u32>().unwrap() > 2020 {
                    return false;
                }
            }
            "eyr" => {
                if val.parse::<u32>().unwrap() < 2020 || val.parse::<u32>().unwrap() > 2030 {
                    return false;
                }
            }
            "hgt" => {
                if val.ends_with("cm") {
                    let num = val.trim_end_matches("cm").parse::<u32>().unwrap_or(0);
                    if num < 150 || num > 193 {
                        return false;
                    }
                } else if val.ends_with("in") {
                    let num = val.trim_end_matches("in").parse::<u32>().unwrap_or(0);
                    if num < 59 || num > 76 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            "hcl" => {
                let color_re = Regex::new(r"^#[A-Fa-f0-9]{6}$").unwrap();

                if !color_re.is_match(val) {
                    return false;
                }
            }
            "ecl" => {
                let color_re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();

                if !color_re.is_match(val) {
                    return false;
                }
            }
            "pid" => {
                let num_re = Regex::new(r"^[0-9]{9}$").unwrap();

                if !num_re.is_match(val) {
                    return false;
                }
            }
            _ => {}
        }
    }

    return true;
}

fn process(input: &str) -> usize {
    let paras = input.split_terminator("\n\n").collect_vec();

    paras
        .into_iter()
        .filter(|p| check_count(p))
        .filter(|p| check_field_validity(p))
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
