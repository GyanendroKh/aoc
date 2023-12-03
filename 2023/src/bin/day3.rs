use std::{collections::HashSet, fs::read_to_string, ops::Range};

#[derive(Debug)]
struct Num {
    num: u32,
    loc: Range<usize>,
}

fn main() {
    let lines = read_lines("./data/day3-input.txt");

    let mut all_nums: Vec<Vec<Num>> = Vec::new();
    let mut symbols: HashSet<(usize, usize)> = HashSet::new();

    for (l_idx, line) in lines.iter().enumerate() {
        let mut nums: Vec<Num> = Vec::new();
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
                        loc: (digit_start as usize..idx),
                    });
                    digit_start = -1;
                }

                if c == '*' {
                    symbols.insert((l_idx, idx));
                }
            }

            if digit_start != -1 && idx == line.len() - 1 {
                let range = digit_start as usize..idx + 1;
                let num = &line[range].parse::<u32>().unwrap();

                nums.push(Num {
                    num: num.clone(),
                    loc: (digit_start as usize..idx + 1),
                });
            }
        }

        all_nums.push(nums);
    }

    let sum = symbols
        .iter()
        .filter_map(|(i, j)| {
            let start = if i > &0 { i.clone() - 1 } else { i.clone() };

            let matches = (start..i.clone() + 2).fold(Vec::<&Num>::new(), |mut acc, x| {
                let nums = all_nums.get(x).unwrap();

                nums.iter()
                    .filter(|&n| {
                        if j > &0 {
                            if n.loc.contains(&(j.clone() - 1)) {
                                return true;
                            }
                        }
                        if n.loc.contains(j) {
                            return true;
                        }

                        if n.loc.contains(&(j.clone() + 1)) {
                            return true;
                        }

                        return false;
                    })
                    .for_each(|m| acc.push(m));

                return acc;
            });

            if matches.len() == 2 {
                return Some(matches.get(0).unwrap().num * matches.get(1).unwrap().num);
            }

            return None;
        })
        .sum::<u32>();

    println!("{:?}", sum);
}

fn read_lines(filename: &str) -> Vec<String> {
    return read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}
