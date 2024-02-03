use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let forbidden = ["ab", "cd", "pq", "xy"];

    input
        .lines()
        .filter_map(|l| {
            for i in 0..forbidden.len() {
                if l.contains(forbidden[i]) {
                    return None;
                }
            }

            let (vc, dl) = l.chars().tuple_windows().enumerate().fold(
                (0usize, false),
                |(mut vc, mut dl), (idx, (c1, c2))| {
                    if c1 == c2 {
                        dl = true;
                    }

                    if vowels.contains(&c1) {
                        vc += 1;
                    }

                    if idx == l.len() - 2 && vowels.contains(&c2) {
                        vc += 1;
                    }
                    (vc, dl)
                },
            );
            match (vc >= 3, dl) {
                (true, true) => Some(1),
                _ => None,
            }
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
