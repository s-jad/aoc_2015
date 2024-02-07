use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn parse_input(input: &str) -> Vec<(u32, HashMap<&str, u32>)> {
    input
        .lines()
        .map(|l| {
            let (cats, vals) = l
                .split_terminator(&[',', ' ', '\n'])
                .filter(|s| !s.is_empty())
                .map(|s| s.trim_end_matches(":"))
                .fold((Vec::new(), Vec::new()), |(mut cats, mut vals), s| {
                    match s.parse::<u32>() {
                        Ok(val) => vals.push(val),
                        Err(_) => cats.push(s),
                    }
                    (cats, vals)
                });

            let id = vals[0];
            let mut map = HashMap::new();

            for (i, c) in cats.iter().skip(1).enumerate() {
                map.insert(*c, vals[i + 1]);
            }

            (id, map)
        })
        .collect_vec()
}

fn process(input: &str) -> u32 {
    let target: HashMap<&str, u32> = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .iter()
    .cloned()
    .collect();

    let sues = parse_input(input);
    let mut sue_score = 0;

    for (id, sue) in sues.iter() {
        for (obj, target_num) in target.iter() {
            match sue.get(obj) {
                Some(val) => {
                    if val == target_num {
                        sue_score += 1;
                    }
                }
                None => {}
            }
        }
        if sue_score == 3 {
            return *id;
        } else {
            sue_score = 0;
        }
    }

    return 0;
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
