use itertools::Itertools;
use std::time::Instant;

#[inline(always)]
fn surface_area(s1: u32, s2: u32, b: u32) -> u32 {
    (s1 * s2 * b) + (s1 * 2 + s2 * 2)
}

fn process(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let (l, w, h) = l
                .split('x')
                .into_iter()
                .filter_map(|s| s.parse::<u32>().ok())
                .sorted()
                .collect_tuple::<(_, _, _)>()
                .unwrap();

            surface_area(l, w, h)
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
