use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn is_xmas(v: &Vec<Vec<char>>, i: i32, j: i32, di: i32, dj: i32) -> bool {
    v[i as usize][j as usize] == 'X'
        && v[(i + 1 * di) as usize][(j + 1 * dj) as usize] == 'M'
        && v[(i + 2 * di) as usize][(j + 2 * dj) as usize] == 'A'
        && v[(i + 3 * di) as usize][(j + 3 * dj) as usize] == 'S'
}

fn main() {
    let part: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| panic!("Usage: day04 partNumber inputPath"));

    let path = std::env::args()
        .nth(2)
        .unwrap_or_else(|| panic!("Usage: day04 partNumber inputPath"));

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file).lines();

    let grid: Vec<Vec<char>> = reader.map(|x| x.unwrap().chars().collect()).collect();
    let line_len: usize = grid[0].len();

    let mut sum = 0;

    for i in 0..grid.len() {
        for j in 0..line_len {
            let vertical = [(true, 0), (i <= grid.len() - 4, 1), (i >= 3, -1)];
            let horizontal = [(true, 0), (j <= line_len - 4, 1), (j >= 3, -1)];

            sum += vertical
                .iter()
                .map(|(n, di)| {
                    if *n {
                        horizontal
                            .iter()
                            .filter(|(n, dj)| *n && is_xmas(&grid, i as i32, j as i32, *di, *dj))
                            .count()
                    } else {
                        0
                    }
                })
                .sum::<usize>();
        }
    }

    dbg!(sum);
}
