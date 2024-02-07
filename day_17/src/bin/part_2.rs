use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> usize {
    let mut containers = input
        .lines()
        .filter_map(|l| l.trim().parse::<u32>().ok())
        .collect_vec();

    containers.sort_by(|a, b| b.cmp(a));

    let mut min_containers = vec![];

    for i in 1..containers.len() - 1 {
        containers.iter().map(|v| *v).combinations(i).for_each(|c| {
            let sum: u32 = c.into_iter().sum();
            if sum == 150 {
                min_containers.push(1);
            }
        });
        if min_containers.len() > 0 {
            return min_containers.len();
        }
    }

    0
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
