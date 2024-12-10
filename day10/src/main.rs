use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Grid {
    v: Vec<Vec<u8>>,
}

impl Grid {
    fn neighbours(&self, cor: &(usize, usize)) -> [Option<(usize, usize)>; 4] {
        let &(i, j) = cor;
        [
            if i == 0 { None } else { Some((i - 1, j)) },
            if i >= self.v.len() - 1 {
                None
            } else {
                Some((i + 1, j))
            },
            if j == 0 { None } else { Some((i, j - 1)) },
            if j >= self.v[i].len() - 1 {
                None
            } else {
                Some((i, j + 1))
            },
        ]
    }

    pub fn uphill_neighbours(
        &self,
        cor: &(usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> + '_ {
        let k = self.get(cor);
        self.neighbours(cor)
            .into_iter()
            .filter_map(move |ncor| ncor.filter(|ncor| self.get(ncor) == k + 1))
    }

    pub fn get(&self, cor: &(usize, usize)) -> u8 {
        let &(i, j) = cor;
        self.v[i][j]
    }

    pub fn get_heads(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.v.iter().enumerate().flat_map(|(i, v)| {
            v.iter()
                .enumerate()
                .filter(|(_, &k)| k == 0)
                .map(move |(j, _)| (i, j))
        })
    }
}

fn main() {
    let part: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or_else(|| panic!("Usage: day10 partNumber inputPath"));

    let path = std::env::args()
        .nth(2)
        .unwrap_or_else(|| panic!("Usage: day10 partNumber inputPath"));

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file).lines();

    let grid = Grid {
        v: reader
            .map(|x| x.unwrap().bytes().map(|b| b - b'0').collect())
            .collect(),
    };

    let mut sum = 0;

    for (row, col) in grid.get_heads() {
        let mut visited = HashSet::from([(row, col)]);

        let mut stack = vec![(row, col)];
        while let Some(head) = stack.pop() {
            for cor in grid.uphill_neighbours(&head) {
                stack.push(cor);
                if visited.contains(&cor) {
                    if part == 2 && grid.get(&cor) == 9 {
                        sum += 1;
                    }
                    continue;
                }
                visited.insert(cor);
                if grid.get(&cor) == 9 {
                    sum += 1;
                }
            }
        }
    }

    dbg!(sum);
}
