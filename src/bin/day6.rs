use aoc2026::read_input;

fn part1(s: &str) -> u64 {
    let matrix: Vec<Vec<&str>> = s
        .trim()
        .split("\n")
        .map(|l| l.split_whitespace().collect())
        .collect();
    let row = matrix.len();
    let col = matrix[0].len();

    let mut result = 0;
    for c in 0..col {
        let mut acc_mult = 1;
        let mut acc_sum = 0;
        let op = matrix[matrix.len() - 1][c];
        for r in 0..row - 1 {
            let v = matrix[r][c].parse::<u64>().unwrap();
            match op {
                "*" => acc_mult *= v,
                "+" => acc_sum += v,
                _ => panic!("help"),
            };
        }
        if acc_mult > 1 {
            result += acc_mult;
        }
        result += acc_sum;
    }

    result
}

fn chars_to_number(v: &[char]) -> u64 {
    v.iter()
        .filter_map(|c| c.to_digit(10))
        .fold(0, |acc, v| acc * 10 + v as u64)
}

fn apply(acc: &[Vec<char>], op: &str) -> u64 {
    let mut acc_mult = 1;
    let mut acc_sum = 0;
    for v in acc {
        let n = chars_to_number(v);
        match op {
            "*" => acc_mult *= n,
            "+" => acc_sum += n,
            _ => panic!("help"),
        };
    }
    if acc_mult > 1 {
        return acc_mult + acc_sum;
    }
    acc_sum
}

fn part2(s: &str) -> u64 {
    let mut lines: Vec<&str> = s.trim().lines().collect();
    let ops: Vec<&str> = lines.pop().unwrap().split_whitespace().collect();

    let rows = lines.len();
    let cols = lines[0].len();
    let mut result = 0;
    let mut ops_idx = 0;

    let mut acc: Vec<Vec<char>> = Vec::new();

    for c in 0..cols {
        let vertical: Vec<char> = (0..rows).map(|r| lines[r].as_bytes()[c] as char).collect();
        let all_spaces = vertical.iter().all(|&c| c == ' ');
        if all_spaces {
            let op = ops[ops_idx];
            result += apply(&acc, op);
            acc.clear();
            ops_idx += 1;
        } else {
            acc.push(vertical);
        }
    }
    if !acc.is_empty() {
        let op = ops[ops_idx];
        result += apply(&acc, op);
    }

    result
}

fn main() {
    let input = read_input("day6.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
