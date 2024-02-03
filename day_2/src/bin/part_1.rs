use itertools::Itertools;
use std::time::Instant;

fn surface_area(l: usize, w: usize, h: usize) -> usize {
    let slack = ((l * w).min(w * h)).min(h * l);
    (2 * l * w) + (2 * w * h) + (2 * h * l) + slack
}

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let (l, w, h) = l
                .split('x')
                .into_iter()
                .filter_map(|s| s.parse::<usize>().ok())
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
