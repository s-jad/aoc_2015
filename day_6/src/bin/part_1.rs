use itertools::Itertools;
use std::time::Instant;

fn control_lights<'a>(grid: &mut [[u8; 1000]; 1000], op: &'a str) {
    let parts = op.split_terminator(&[',', ' ', '\n']).collect_vec();
    match parts[0] {
        "turn" => {
            let (sx, sy, ex, ey) = parts[2..]
                .iter()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_tuple::<(_, _, _, _)>()
                .unwrap();

            match parts[1] {
                "on" => {
                    for y in sy..=ey {
                        for x in sx..=ex {
                            grid[y][x] = 1;
                        }
                    }
                }
                "off" => {
                    for y in sy..=ey {
                        for x in sx..=ex {
                            grid[y][x] = 0;
                        }
                    }
                }
                _ => unreachable!(),
            }
        }
        "toggle" => {
            let (sx, sy, ex, ey) = parts[1..]
                .iter()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect_tuple::<(_, _, _, _)>()
                .unwrap();

            for y in sy..=ey {
                for x in sx..=ex {
                    grid[y][x] ^= 1;
                }
            }
        }
        _ => unreachable!(),
    }
}

fn process(input: &str) -> usize {
    let ops = input.lines().collect_vec();
    let mut grid = [[0u8; 1000]; 1000];

    for op in ops {
        control_lights(&mut grid, op);
    }

    grid.iter()
        .map(|row| row.iter().filter(|b| b == &&1).count())
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
