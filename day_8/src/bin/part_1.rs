use itertools::Itertools;
use std::time::Instant;

fn parse_str(line: &str) -> usize {
    let l_len = line.len();
    let mut s_vec = line
        .chars()
        .enumerate()
        .filter(|(idx, _)| *idx != 0 && *idx != l_len - 1)
        .map(|(_, c)| c)
        .collect_vec();

    let s_vec_len = s_vec.len();
    if s_vec_len == 0 {
        return 0;
    } else if s_vec_len == 1 {
        return 1;
    }

    let mut parsed_s = Vec::new();
    let mut i = 0;
    while i <= s_vec_len - 1 {
        if s_vec[i] == '\\' && s_vec[i + 1] == 'x' {
            let (v1, v2) = (s_vec[i + 2], s_vec[i + 3]);
            let hex_str = format!("{}{}", v1, v2);
            let hex_num = u8::from_str_radix(&hex_str, 16).expect("should be valid hex_num");
            parsed_s.push(hex_num as char);
            i += 4;
        } else if s_vec[i] == '\\' && (s_vec[i + 1] == '"' || s_vec[i + 1] == '\\') {
            parsed_s.push(s_vec[i + 1]);
            i += 2;
        } else {
            parsed_s.push(s_vec[i]);

            if i + 1 < s_vec_len {
                match s_vec[i + 1] {
                    '\\' => i += 1,
                    _ => {
                        parsed_s.push(s_vec[i + 1]);
                        i += 2;
                    }
                }
            } else {
                i += 1;
            }
        }
    }

    parsed_s.len()
}

fn process(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            let trimmed = l.trim();
            let cl = trimmed.len();
            let sl = parse_str(trimmed);

            cl - sl
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
