use aoc2026::read_input;

const DI: [i32; 8] = [-1, -1, -1, 0, 1, 1, 1, 0];
const DJ: [i32; 8] = [-1, 0, 1, 1, 1, 0, -1, -1];

fn count_adj(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut counter = 0;
    for k in 0..8 {
        let di = i as i32 + DI[k];
        let dj = j as i32 + DJ[k];
        if di < 0 || dj < 0 {
            continue;
        }
        match matrix.get(di as usize) {
            Some(v) => match v.get(dj as usize) {
                Some(&t) => {
                    if t == '@' {
                        counter += 1
                    }
                }
                None => continue,
            },
            None => continue,
        }
    }
    counter
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for r in matrix {
        println!("{:?}", r);
    }
}

fn part1(s: &str) -> u32 {
    let matrix: Vec<Vec<char>> = s
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut count = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == '@' && count_adj(&matrix, i, j) < 4 {
                count += 1;
            }
        }
    }
    count
}

fn part2(s: &str) -> u32 {
    let mut matrix: Vec<Vec<char>> = s
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut count = 0;

    loop {
        let mut removed = 0;
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == '@' && count_adj(&matrix, i, j) < 4 {
                    removed += 1;
                    matrix[i][j] = 'x';
                }
            }
        }
        if removed == 0 {
            break;
        }
        count += removed;
    }
    count
}

fn main() {
    let input = read_input("day4.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
