use itertools::Itertools;
use std::time::Instant;

fn parse_str(line: &str) -> usize {
    let mut parsed_s = Vec::new();
    parsed_s.push('"');
    println!("line: {line}");
    for c in line.chars() {
        println!("c: {c}");
        match c {
            '"' => {
                parsed_s.push('\\');
                parsed_s.push('"');
            }
            '\\' => {
                parsed_s.push('\\');
                parsed_s.push('\\');
            }
            _ => parsed_s.push(c),
        }
    }
    parsed_s.push('"');
    println!("parsed_s: {parsed_s:?}");
    parsed_s.len()
}

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let trimmed = l.trim();
            let cl = trimmed.len();
            let sl = parse_str(trimmed);

            sl - cl
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
