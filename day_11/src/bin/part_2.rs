use itertools::Itertools;
use std::time::Instant;

fn valid(input: &[char], pairs: &mut [char]) -> bool {
    let mut three_ascending = false;

    let mut p_idx = 0;
    for i in 0..input.len() - 2 {
        if input[i] == 'i' || input[i] == 'l' || input[i] == 'o' {
            return false;
        }

        let ascii = input[i] as u8;

        if ascii + 1 == input[i + 1] as u8 && ascii + 2 == input[i + 2] as u8 {
            three_ascending = true;
        }

        if ((input[i] == input[i + 1] && input[i] != input[i + 2])
            || (input[i + 1] == input[i + 2] && input[i + 1] != input[i]))
            && !pairs.contains(&input[i + 1])
        {
            pairs[p_idx] = input[i + 1];
            p_idx += 1;
        }
    }

    match (p_idx >= 2, three_ascending) {
        (true, true) => return true,
        _ => return false,
    }
}

fn process() -> String {
    let mut input = ['c', 'q', 'j', 'x', 'x', 'z', 'a', 'a'];
    let mut idx = 7;
    let mut pairs = ['!', '!', '!'];

    while !valid(&input, &mut pairs) {
        pairs = ['!', '!', '!'];
        let new = (input[idx] as u8 - 96) % 26;
        if new == 8 || new == 11 || new == 14 {
            input[idx] = (new + 98) as char;
            for j in (idx + 1)..8 {
                input[j] = 'a';
            }
            idx = 7;
        }
        if new == 0 {
            input[idx] = (new + 97) as char;
            idx = (idx - 1) % 8;
        } else {
            input[idx] = (new + 97) as char;
            idx = 7;
        }
    }

    input.iter().collect::<String>()
}

fn main() {
    let start = Instant::now();
    let output = process();
    let time = start.elapsed();

    dbg!(output, time);
}
