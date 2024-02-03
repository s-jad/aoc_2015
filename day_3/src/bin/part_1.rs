use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn process(input: &str) -> usize {
    input
        .trim()
        .chars()
        .fold(
            (HashSet::from([(0, 0)]), (0i32, 0i32)),
            |(mut acc, mut idx), c| {
                match c {
                    '<' => idx = (idx.0 - 1, idx.1),
                    '>' => idx = (idx.0 + 1, idx.1),
                    '^' => idx = (idx.0, idx.1 + 1),
                    'v' => idx = (idx.0, idx.1 - 1),
                    _ => unreachable!(),
                }

                acc.insert(idx);
                (acc, idx)
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
