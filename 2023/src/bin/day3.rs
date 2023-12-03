use std::{collections::HashSet, fs::read_to_string, ops::Range};

#[derive(Debug)]
struct Num {
    num: u32,
    line: usize,
    loc: Range<usize>,
}

fn main() {
    let lines = read_lines("./data/day3-input.txt");

    let mut nums: Vec<Num> = Vec::new();
    let mut symbols: HashSet<(usize, usize)> = HashSet::new();

    for (l_idx, line) in lines.iter().enumerate() {
        let mut digit_start: i32 = -1;

        for (idx, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if digit_start == -1 {
                    digit_start = idx as i32;
                }
            } else {
                if digit_start != -1 {
                    let range = digit_start as usize..idx;
                    let num = &line[range].parse::<u32>().unwrap();

                    nums.push(Num {
                        num: num.clone(),
                        line: l_idx,
                        loc: (digit_start as usize..idx),
                    });
                    digit_start = -1;
                }

                if c != '.' {
                    let neighbours = generate_neighbours(l_idx, idx);

                    for n in neighbours {
                        symbols.insert(n);
                    }
                }
            }

            if digit_start != -1 && idx == line.len() - 1 {
                let range = digit_start as usize..idx + 1;
                let num = &line[range].parse::<u32>().unwrap();

                nums.push(Num {
                    num: num.clone(),
                    line: l_idx,
                    loc: (digit_start as usize..idx),
                });
            }
        }
    }

    let sum = nums
        .iter()
        .filter_map(|n| {
            match n.loc.clone().find(|x| {
                return symbols.contains(&(n.line, x.clone()));
            }) {
                Some(_) => {
                    return Some(n.num);
                }
                None => {
                    return None;
                }
            }
        })
        .sum::<u32>();

    println!("{:?}", sum);
}

fn generate_neighbours(i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut idxs: Vec<(usize, usize)> = Vec::new();

    if i > 0 {
        if j > 0 {
            idxs.push((i - 1, j - 1));
        }
        idxs.push((i - 1, j));
        idxs.push((i - 1, j + 1));
    }

    if j > 0 {
        idxs.push((i, j - 1));
    }
    idxs.push((i, j));
    idxs.push((i, j + 1));

    if j > 0 {
        idxs.push((i + 1, j - 1));
    }
    idxs.push((i + 1, j));
    idxs.push((i + 1, j + 1));

    return idxs;
}

fn read_lines(filename: &str) -> Vec<String> {
    return read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}
