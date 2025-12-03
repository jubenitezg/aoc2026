use std::error::Error;
use std::ops::Index;
use std::ptr::eq;

use aoc2026::read_input;

fn count_digits(n: u64) -> u32 {
    let mut tmp = n;
    let mut count = 0;
    while tmp != 0 {
        tmp /= 10;
        count += 1;
    }
    count
}

fn invalid(n: u64) -> bool {
    if count_digits(n) % 2 != 0 {
        return false;
    }
    let s = n.to_string();
    let sn = s.as_bytes();
    let mid = sn.len() / 2;
    let mut i = 0;
    let mut j = mid;
    while i < mid {
        if sn[i] != sn[j] {
            return false;
        }
        i += 1;
        j += 1;
    }
    true
}

fn invalid_v2(n: u64) -> bool {
    let s = n.to_string();
    let b = s.as_bytes();
    //println!("\n\nNumber: {}", n);
    let mut at_least_one = false;
    for i in 1..=s.len() {
        let mut chunks_iter = b.chunks(i);
        let mut curr_chunk_opt = chunks_iter.next();
        let mut equal_chunks = false;
        while let Some(curr_chunk) = curr_chunk_opt {
            let next_chunk_opt = chunks_iter.next();
            if let Some(next_chunk) = next_chunk_opt {
                //println!("Curr chunk {:?} | Next chunk {:?}", curr_chunk, next_chunk);
                let mut i = 0;
                let mut j = 0;
                if curr_chunk.len() != next_chunk.len() {
                    equal_chunks = false;
                    //println!("NOT EQUAL CHUNKS");
                } else {
                    while i < curr_chunk.len() && j < next_chunk.len() {
                        if curr_chunk[i] != next_chunk[j] {
                            //println!("NOT EQUAL CHUNKS");
                            equal_chunks = false;
                            break;
                        }
                        i += 1;
                        j += 1;
                    }
                    if i == curr_chunk.len() {
                        equal_chunks = true;
                    }
                    if !equal_chunks {
                        break;
                    }
                }
            } else {
                //println!("No more chunks, curr chunk {:?}", curr_chunk);
            }
            curr_chunk_opt = next_chunk_opt;
        }
        // println!(
        //     "at_least_one{} || equal_chunks{} = {}",
        //     at_least_one,
        //     equal_chunks,
        //     at_least_one || equal_chunks
        // );
        at_least_one = at_least_one || equal_chunks;
        if at_least_one {
            return true;
        }
    }
    //println!("++++++++++++++++++++++++++++++++++++++++++++++++++++");
    //println!("at_least_one {}", at_least_one);
    at_least_one
}

fn part1(s: &str) -> u64 {
    let mut result = 0u64;
    for range in s.trim().split(",") {
        let x: Vec<u64> = range.split("-").map(|c| c.parse().unwrap()).collect();
        let low = x[0];
        let high = x[1];
        for id in low..=high {
            if invalid(id) {
                result += id;
            }
        }
    }
    result
}

fn part2(s: &str) -> u64 {
    let mut result = 0;
    for range in s.trim().split(",") {
        let x: Vec<u64> = range.split("-").map(|c| c.parse().unwrap()).collect();
        let low = x[0];
        let high = x[1];
        for id in low..=high {
            if invalid_v2(id) {
                //println!("INVALID: {}", id);
                result += id;
            }
        }
    }
    result
}

fn main() {
    let input = read_input("day2.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
