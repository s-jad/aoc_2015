use md5::compute;
use std::time::Instant;

fn process() -> usize {
    let input = "bgvyzdsv";

    for i in 1_000_000..10_000_000 {
        let s = format!("{}{}", input, i.to_string());

        if format!("{:x}", compute(s.as_bytes())).starts_with("000000") {
            return i;
        }
    }

    return 0;
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
