use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn process(input: &str) -> i32 {
    let preferences = input.lines().fold(HashMap::new(), |mut acc, l| {
        let parts = l.split_whitespace().collect_vec();
        let p1 = parts[0].trim_end_matches(".");
        let p2 = parts[10].trim_end_matches(".");
        let sign = match parts[2] {
            "gain" => 1,
            "lose" => -1,
            _ => unreachable!(),
        };
        let num = parts[3].parse::<i32>().unwrap();

        acc.entry(p1).or_insert(Vec::new()).push((p2, num * sign));
        acc
    });

    let people = preferences.keys().collect_vec();
    let p_len = people.len();
    let mut highest_overall_happiness = 0;

    for order in people.iter().permutations(p_len) {
        let mut happiness = 0;

        for i in 0..p_len {
            let j = (i + 1) % p_len;
            let h1 = preferences
                .get(*order[i])
                .unwrap()
                .iter()
                .find(|p2| p2.0 == **order[j])
                .unwrap()
                .1;

            let h2 = preferences
                .get(*order[j])
                .unwrap()
                .iter()
                .find(|p1| p1.0 == **order[i])
                .unwrap()
                .1;

            happiness += h1 + h2;
        }

        if happiness > highest_overall_happiness {
            highest_overall_happiness = happiness;
        }
    }

    highest_overall_happiness
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
