use itertools::Itertools;
use std::time::Instant;

fn check_neighbours(y: usize, x: usize, neighbours: &[(i32, i32)], lights: &Vec<Vec<u32>>) -> u32 {
    let mut total_on = 0;

    for (nx, ny) in neighbours {
        let dx = (x as i32) + nx;
        let dy = (y as i32) + ny;

        if dy >= 0 && dx >= 0 && dy < 100 && dx < 100 {
            total_on += lights[dy as usize][dx as usize];
        }
    }

    match (lights[y][x], total_on) {
        (1, 2) | (1, 3) | (0, 3) => return 1,
        _ => return 0,
    }
}

fn process(input: &str) -> u32 {
    let mut lights = input
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| match c {
                    '#' => 1,
                    _ => 0,
                })
                .collect_vec()
        })
        .collect_vec();

    let neighbours = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    lights[0][0] = 1;
    lights[99][0] = 1;
    lights[0][99] = 1;
    lights[99][99] = 1;

    for _ in 0..100 {
        let mut next = vec![vec![0; 100]; 100];
        for i in 0..100 {
            for j in 0..100 {
                next[i][j] = check_neighbours(i, j, &neighbours, &lights);
            }
        }
        lights = next;
        lights[0][0] = 1;
        lights[99][0] = 1;
        lights[0][99] = 1;
        lights[99][99] = 1;
    }

    lights.iter().map(|r| r.iter().sum::<u32>()).sum()
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
