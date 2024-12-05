use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::Regex;

fn main() {
    let part: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| panic!("Usage: day03 partNumber inputPath"));

    let path = std::env::args()
        .nth(2)
        .unwrap_or_else(|| panic!("Usage: day03 partNumber inputPath"));

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file).lines();

    let mut sum = 0;
    let mut enabled = true;
    for line in reader {
        let line: String = line.unwrap();
        let re = Regex::new(if part == 2 {
            r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)"
        } else {
            r"mul\((\d+),(\d+)\)"
        })
        .unwrap();

        for mat in re.find_iter(&line) {
            match mat.as_str() {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                mul => {
                    if enabled {
                        if let Some(cap) = re.captures(mul) {
                            let v1 = &cap[1].parse().unwrap_or(0);
                            let v2 = &cap[2].parse().unwrap_or(0);
                            sum += v1 * v2;
                        }
                    }
                }
            }
        }
    }
    dbg!(sum);
}
