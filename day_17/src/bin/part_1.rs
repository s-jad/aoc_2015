use itertools::Itertools;
use std::time::Instant;

fn total_combinations(containers: &[u32]) -> u32 {
    let total = 150;
    let mut dp = vec![0; total + 1];
    dp[0] = 1;

    for c in containers {
        for capacity in ((*c as usize)..=total).rev() {
            dp[capacity] += dp[capacity - (*c as usize)];
            println!("dp: {dp:?}");
        }
    }
    dp[total]
}

fn process(input: &str) -> u32 {
    let containers = input
        .lines()
        .filter_map(|l| l.trim().parse::<u32>().ok())
        .collect_vec();

    println!("containers: {containers:?}");

    total_combinations(&containers)
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
