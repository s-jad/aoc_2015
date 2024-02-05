use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> i32 {
    input
        .split_terminator(&[' ', '\n', '[', ']', ':', ',', '}'][..])
        .filter_map(|s| s.parse::<i32>().ok())
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
