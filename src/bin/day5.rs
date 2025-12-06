use std::collections::HashSet;

use aoc2026::read_input;

fn part1(s: &str) -> u64 {
    let parts: Vec<&str> = s.trim().split_whitespace().collect();

    let (fresh, available): (Vec<&str>, Vec<&str>) =
        parts.into_iter().partition(|&c| c.contains("-"));

    let fresh_ids: Vec<(u64, u64)> = fresh
        .iter()
        .map(|&l| {
            let mut ranges = l.split('-');
            let first: u64 = ranges.next().unwrap().parse().unwrap();
            let last: u64 = ranges.next().unwrap().parse().unwrap();
            (first, last)
        })
        .collect();
    let available_ids: Vec<u64> = available.iter().map(|&c| c.parse().unwrap()).collect();

    let mut fresh_ingredient = 0;
    let mut seen: HashSet<u64> = HashSet::new();
    for (i, j) in fresh_ids {
        for id in &available_ids {
            if i <= *id && *id <= j && !seen.contains(id) {
                fresh_ingredient += 1;
                seen.insert(*id);
            }
        }
    }
    fresh_ingredient
}

fn main() {
    let input = read_input("day5.txt");
    println!("Part 1: {}", part1(&input));
}
