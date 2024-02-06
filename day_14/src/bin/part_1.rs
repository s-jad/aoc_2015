use itertools::Itertools;
use std::time::Instant;

fn process(input: &str) -> u32 {
    const SECONDS: u32 = 2503;

    let reindeer_stats = input
        .lines()
        .map(|l| {
            let (speed, dur, rest) = l
                .split_whitespace()
                .filter_map(|s| s.parse::<u32>().ok())
                .collect_tuple::<(_, _, _)>()
                .unwrap();

            (speed, dur, rest)
        })
        .collect_vec();

    let mut most_distance_travelled = 0;

    for (s, d, r) in reindeer_stats {
        let cycle = d + r;
        let reps = SECONDS / cycle;
        let end_pos_in_cycle = SECONDS % cycle;
        let mut dist = 0;

        match d < end_pos_in_cycle {
            true => dist = s * d * (reps + 1),
            false => dist = s * d * reps + (end_pos_in_cycle * s),
        }

        if dist > most_distance_travelled {
            most_distance_travelled = dist;
        }
    }

    most_distance_travelled
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
