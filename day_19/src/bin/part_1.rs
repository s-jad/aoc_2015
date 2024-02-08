use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn process(input: &str) -> usize {
    let (ops, start) = input.lines().fold(
        (HashMap::new(), String::new()),
        |(mut ops, mut start), line| {
            let len = line.len();
            match (len > 100, len < 5) {
                (true, _) => {
                    start = line.to_owned();
                }
                (false, false) => {
                    let (s, e) = line
                        .split_whitespace()
                        .into_iter()
                        .filter(|s| !s.contains(">"))
                        .collect_tuple::<(_, _)>()
                        .unwrap();

                    ops.entry(s).or_insert(Vec::new()).push(e);
                }
                (false, true) => {}
            }

            (ops, start)
        },
    );

    let len = start.len();

    let mut variations = HashSet::new();
    for i in 0..len - 1 {
        // Check for substrings of length  1

        match (
            ops.get(&start[i..i + 2]),
            ops.get(&start[i..i + 1]),
            ops.get(&start[i + 1..i + 2]),
        ) {
            (Some(v0), _, _) => {
                for j in 0..v0.len() {
                    variations.insert(format!("{}{}{}", &start[..i], v0[j], &start[i + 2..]));
                }
            }
            (None, Some(v1), None) => {
                for j in 0..v1.len() {
                    variations.insert(format!("{}{}{}", &start[..i], v1[j], &start[i + 1..]));
                }
            }
            (None, None, Some(v2)) => {
                for j in 0..v2.len() {
                    variations.insert(format!("{}{}{}", &start[..i + 1], v2[j], &start[i + 2..]));
                }
            }
            (None, Some(v1), Some(v2)) => {
                for j in 0..v1.len() {
                    variations.insert(format!("{}{}{}", &start[..i], v1[j], &start[i + 1..]));
                }
                for j in 0..v2.len() {
                    variations.insert(format!("{}{}{}", &start[..i + 1], v2[j], &start[i + 2..]));
                }
            }
            (None, None, None) => {}
        }
    }

    variations.len()
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
