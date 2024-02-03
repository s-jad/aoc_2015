use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> isize {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => unreachable!(),
        })
        .sum()
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
