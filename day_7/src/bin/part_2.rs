use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn dfs(
    graph: &HashMap<String, Vec<(usize, String)>>,
    bag: &str,
    visited: &mut HashMap<String, bool>,
    multiplier: usize,
) -> usize {
    let mut count = 0;

    if bag != "shiny gold" {
        count += multiplier;
    }

    if let Some(bags) = graph.get(bag) {
        for (n, b) in bags {
            count += dfs(graph, b, visited, *n * multiplier);
        }
    }

    count
}

fn process(input: &str) -> usize {
    let graph = input
        .lines()
        .map(|l| {
            let (bag, c) = l
                .split_terminator(" bags contain ")
                .collect_tuple::<(&str, &str)>()
                .unwrap();

            if c == "no other bags." {
                (bag.to_owned(), vec![])
            } else {
                let contents = c.split_terminator(", ").collect_vec();
                let mut content_vec = vec![];
                for content in contents {
                    let parts = content.split_once(" ").unwrap();
                    let num = parts.0.parse::<usize>().unwrap();
                    let description = parts
                        .1
                        .trim_end_matches(" bag")
                        .trim_end_matches(" bags")
                        .trim_end_matches(" bag.")
                        .trim_end_matches(" bags.")
                        .to_owned();

                    content_vec.push((num, description));
                }

                (bag.to_owned(), content_vec)
            }
        })
        .collect::<HashMap<String, Vec<(usize, String)>>>();

    let mut visited: HashMap<String, bool> = HashMap::new();
    let multiplier = 1;

    dfs(&graph, "shiny gold", &mut visited, multiplier)
}

fn main() {
    let input = include_str!("../../input.txt");

    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();

    dbg!(output, time);
}
