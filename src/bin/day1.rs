use std::ops::Rem;

use aoc2026::read_input;

// (current - rot) % 100

// (current +- rot) % 100 / 100

fn part1(s: &str) -> u32 {
    let mut c = 0;
    let mut comb = 50i32;
    s.lines().for_each(|l| {
        let n: i32 = l.get(1..).unwrap().parse().unwrap();
        if l.starts_with("R") {
            comb = (comb + n) % 100;
        }
        if l.starts_with("L") {
            comb = (comb - n) % 100;
            if comb < 0 {
                comb += 100;
            }
        }
        if comb == 0 {
            c += 1;
        }
    });
    c
}

fn part2(s: &str) -> u32 {
    let mut c = 0;
    let mut comb = 50i32;
    for line in s.lines() {
        let rotation: i32 = line[1..].parse().unwrap();
        let is_left = line.starts_with("L");

        let d = if !is_left {
            // clockwise
            if comb == 0 { 0 } else { 100 - comb }
        } else {
            // counterclockwise
            comb
        };

        let passes = if d == 0 {
            // simple, starts at 0, how many '0s' are there
            (rotation / 100) as u32
        } else {
            // no 0 passes
            if rotation < d {
                0
            } else {
                // adjust rotation to '0'
                ((rotation - d) / 100 + 1) as u32
            }
        };

        c += passes;

        if !is_left {
            comb = (comb + rotation) % 100;
        } else {
            comb = (comb - rotation) % 100;
            if comb < 0 {
                comb += 100;
            }
        }
    }

    c
}

fn main() {
    let input = read_input("day1.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
