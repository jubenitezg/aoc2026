use aoc2026::read_input;

fn pretty_print(v: &Vec<Vec<&str>>) {
    for row in v {
        for c in row {
            print!("{} ", c);
        }
        println!("");
    }
}

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

fn main() {
    let input = read_input("day6.txt");
    println!("Part 1: {}", part1(&input));
}
