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

fn main() {
    let input = read_input("day3.txt");
    println!("Part 1: {}", part1(&input));
}
