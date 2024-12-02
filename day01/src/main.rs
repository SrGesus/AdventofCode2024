use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::vec::Vec;

fn main() {
    dbg!(env::args().collect::<String>());

    let part: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| panic!("Usage: day01 partNumber inputPath"));

    let path = std::env::args()
        .nth(2)
        .unwrap_or_else(|| panic!("Usage: day01 partNumber inputPath"));

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file).lines();

    let mut list: Vec<Vec<i32>> = vec![Vec::new(), Vec::new()];

    for line in reader {
        let line: String = line.unwrap();

        line.split_whitespace()
            .enumerate()
            .for_each(|(i, v)| list[i].push(v.parse().unwrap()));
    }
    for ele in &mut list {
        ele.sort();
    }
    let mut sum = 0;

    match part {
        1 => {
            for (v1, v2) in zip(&list[0], &list[1]) {
                sum += (v1 - v2).abs()
            }
        }
        2 => {
            for v1 in &list[0] {
                sum += v1 * list[1].iter().filter(|x| *x == v1).count() as i32;
            }
        }
        _ => panic!("Usage: day01 partNumber inputPath"),
    }
    dbg!(sum);
}
