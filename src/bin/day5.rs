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

fn part2(s: &str) -> u64 {
    let parts: Vec<&str> = s.trim().split_whitespace().collect();

    let mut fresh_ids: Vec<(u64, u64)> = parts
        .into_iter()
        .filter(|&c| c.contains("-"))
        .map(|l| {
            let mut ranges = l.split('-');
            let first: u64 = ranges.next().unwrap().parse().unwrap();
            let last: u64 = ranges.next().unwrap().parse().unwrap();
            (first, last)
        })
        .collect();

    fresh_ids.sort_by_key(|&(a, _)| a);

    let mut combined: Vec<(u64, u64)> = Vec::new();
    for (a, b) in fresh_ids {
        if let Some(last) = combined.last_mut() {
            if a <= last.1 + 1 {
                last.1 = last.1.max(b);
            } else {
                combined.push((a, b));
            }
        } else {
            combined.push((a, b));
        }
    }

    combined.iter().fold(0, |acc, &(a, b)| acc + (b - a) + 1)
}

fn main() {
    let input = read_input("day5.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
