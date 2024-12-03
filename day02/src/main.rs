use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::vec::Vec;

fn main() {
    let part: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| panic!("Usage: day01 partNumber inputPath"));

    let path = std::env::args()
        .nth(2)
        .unwrap_or_else(|| panic!("Usage: day01 partNumber inputPath"));

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file).lines();

    let mut sum = 0;
    for line in reader {
        let line: String = line.unwrap();

        let list: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        fn is_safe<'a, I>(rep: I) -> bool
        where
            I: Iterator<Item = &'a i32> + Clone,
        {
            let mut last_sign = 0;
            for (v1, v2) in zip(rep.clone(), rep.skip(1)) {
                if (v1 - v2).signum() != last_sign && last_sign != 0
                    || (v1 - v2).abs() < 1
                    || (v1 - v2).abs() > 3
                {
                    return false;
                }
                last_sign = (v1 - v2).signum();
            }
            return true;
        }
        if is_safe(list.iter()) {
            sum += 1;
            continue;
        }

        if part != 2 {
            continue;
        }
        
        // There should be a more efficient way to do this but let's just try
        // for every level
        for i in 0..list.len() {
            if is_safe(
                list.iter()
                    .take(i)
                    .chain(list.iter().skip(i + 1))
                    .into_iter(),
            ) {
                println!("For {line} Ignore {}", i + 1);
                sum += 1;
                break;
            }
        }
    }
    dbg!(sum);
}
