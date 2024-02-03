use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn process(input: &str) -> usize {
    input
        .trim()
        .chars()
        .enumerate()
        .fold(
            (HashSet::from([(0, 0)]), (0i32, 0i32), (0i32, 0i32)),
            |(mut acc, mut s_idx, mut r_idx), (c_idx, c)| {
                match (c_idx % 2, c) {
                    (0, '<') => s_idx = (s_idx.0 - 1, s_idx.1),
                    (0, '>') => s_idx = (s_idx.0 + 1, s_idx.1),
                    (0, '^') => s_idx = (s_idx.0, s_idx.1 + 1),
                    (0, 'v') => s_idx = (s_idx.0, s_idx.1 - 1),
                    (1, '<') => r_idx = (r_idx.0 - 1, r_idx.1),
                    (1, '>') => r_idx = (r_idx.0 + 1, r_idx.1),
                    (1, '^') => r_idx = (r_idx.0, r_idx.1 + 1),
                    (1, 'v') => r_idx = (r_idx.0, r_idx.1 - 1),
                    _ => unreachable!(),
                }

                acc.insert(s_idx);
                acc.insert(r_idx);
                (acc, s_idx, r_idx)
            },
        )
        .0
        .len()
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
