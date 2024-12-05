use std::collections::HashMap;
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
    let mut rules = true;
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    for line in reader {
        let line: String = line.unwrap();
        if line.is_empty() {
            rules = false;
            continue;
        }
        if rules {
            let re = Regex::new(r"(\d+)\|(\d+)").unwrap();
            if let Some(cap) = re.captures(&line) {
                let v1 = &cap[1].parse().unwrap_or(0);
                let v2 = &cap[2].parse().unwrap_or(0);

                if map.contains_key(v2) {
                    map.get_mut(v2).unwrap().push(*v1);
                } else {
                    map.insert(*v2, vec![*v1]);
                }
            }
        } else {
            let mut upd: Vec<i32> = line
                .split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect();
            if part == 1 {
                if upd.iter().enumerate().all(|(i, v)| {
                    upd.iter().skip(i + 1).all(|x| {
                        !map.get(v)
                            .map(|before_v| before_v.contains(x))
                            .unwrap_or(false)
                    })
                }) {
                    sum += upd.get(upd.len() / 2).unwrap();
                }
            } else if part == 2 {
                let mut was_incorrect = false;
                let mut cor = false;
                // Just a bubble sort 
                while !cor {
                    cor = true;
                    let (mut i_swap, mut j_swap) = (0, 0);
                    for (i, v2) in upd.iter().enumerate() {
                        if let Some(before_v) = map.get(v2) {
                            for (j, v1) in upd.iter().skip(i + 1).enumerate().rev() {
                                if before_v.contains(v1) {
                                    was_incorrect = true;
                                    cor = false;
                                    (i_swap, j_swap) = (i, j + i + 1);
                                }
                            }
                            if !cor {
                                break;
                            }
                        }
                    }
                    if !cor {
                        upd.swap(i_swap, j_swap);
                    }
                }
                if was_incorrect {
                    println!("Fixed {upd:?}");
                    sum += upd.get(upd.len() / 2).unwrap();
                } else {
                    println!("Correct {upd:?}");
                }
            }
        }
    }
    dbg!(sum);
}
