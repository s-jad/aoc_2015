
use itertools::Itertools;
use std::time::Instant;

fn process() -> usize {
    let mut input: Vec<u8> = vec![1, 1, 1, 3, 1, 2, 2, 1, 1, 3];

    for _ in 0..50 {
        let mut next: Vec<u8> = Vec::new();
        let mut idx = 0;
        while idx < input.len() {
            let num = input[idx];
            let mut count = 1;
            idx += 1;

            if idx < input.len() {
                while input[idx] == num {
                    count += 1;
                    idx += 1;
                }
            }
            next.push(count);
            next.push(num);
        }

        input = next;
    }

    input.len()
}

fn main() {
    let start = Instant::now();
    let output = process();
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
