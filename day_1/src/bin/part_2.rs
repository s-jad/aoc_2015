use itertools::{FoldWhile, Itertools};
use std::time::Instant;

fn process(input: &str) -> i32 {
    input
        .trim()
        .chars()
        .enumerate()
        .fold_while(0i32, |mut acc, (idx, c)| {
            match c {
                '(' => acc += 1,
                ')' => acc -= 1,
                _ => unreachable!(),
            }

            if acc == -1 {
                FoldWhile::Done(idx as i32)
            } else {
                FoldWhile::Continue(acc)
            }
        })
        .into_inner()
        + 1
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
