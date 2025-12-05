use std::cmp::Reverse;

use aoc2026::read_input;

fn part1(s: &str) -> u32 {
    let mut result = 0;
    for line in s.trim().lines() {
        let max1 = line
            .get(..line.len() - 1)
            .and_then(|n| {
                n.char_indices()
                    .max_by_key(|&(i, c)| (c, Reverse(i)))
                    .map(|(i, c)| (i, c.to_digit(10).unwrap()))
            })
            .unwrap();
        let max2 = line
            .get(max1.0 + 1..line.len())
            .and_then(|n| {
                n.char_indices()
                    .max_by(|&(_, c1), &(_, c2)| c1.cmp(&c2))
                    .map(|(i, c)| (i, c.to_digit(10).unwrap()))
            })
            .unwrap();
        result += max1.1 * 10 + max2.1;
    }
    result
}

fn part2(s: &str) -> u64 {
    let mut result = 0;
    for line in s.trim().lines() {
        let mut maxes: Vec<u64> = Vec::new();
        let mut start_at = 0;
        for i in (0..=11).rev() {
            let max = line
                .get(start_at..line.len() - i)
                .and_then(|n| {
                    n.char_indices()
                        .max_by_key(|&(i, c)| (c, Reverse(i)))
                        .map(|(i, c)| {
                            let abs_idx = start_at + i;
                            (abs_idx, c.to_digit(10).unwrap() as u64)
                        })
                })
                .unwrap();

            start_at = max.0 + 1;
            maxes.push(max.1);
        }
        result += maxes.iter().fold(0, |acc, &v| acc * 10 + v);
    }
    result
}

fn main() {
    let input = read_input("day3.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
