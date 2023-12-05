use std::{collections::HashSet, fs::read_to_string, str::FromStr};

#[derive(Debug)]
struct Card {
    no: usize,
    winning_nos: HashSet<u32>,
    nos_u_have: Vec<u32>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((card, nos)) = s.split_once(":") {
            let card_no = card.trim().split_once(" ").and_then(|(_, n)| {
                if let Ok(card_no) = n.trim().parse::<usize>() {
                    return Some(card_no);
                };

                return None;
            });

            let parsed_nos = nos.trim().split_once("|").and_then(|(w_no, h_no)| {
                let winning_nos = w_no
                    .trim()
                    .split(" ")
                    .filter(|n| !n.trim().is_empty())
                    .try_fold(HashSet::<u32>::new(), |mut acc, n| {
                        return match n.parse::<u32>() {
                            Ok(no) => {
                                acc.insert(no);

                                Some(acc)
                            }
                            Err(_) => None,
                        };
                    });

                let having_nos = h_no
                    .trim()
                    .split(" ")
                    .filter(|n| !n.trim().is_empty())
                    .try_fold(Vec::<u32>::new(), |mut acc, n| {
                        return match n.trim().parse::<u32>() {
                            Ok(no) => {
                                acc.push(no);

                                Some(acc)
                            }
                            Err(_) => None,
                        };
                    });

                if winning_nos.is_some() && having_nos.is_some() {
                    Some((winning_nos.unwrap(), having_nos.unwrap()))
                } else {
                    None
                }
            });

            match (&card_no, &parsed_nos) {
                (Some(c), Some((w, h))) => {
                    return Ok(Card {
                        no: c.clone(),
                        winning_nos: w.clone(),
                        nos_u_have: h.clone(),
                    });
                }
                _ => {}
            }
        }

        return Err(ParseCardError);
    }
}

fn main() {
    let cards = read_lines("./data/day4-input.txt");

    let sum = cards
        .iter()
        .map(|c| {
            let winning = c.nos_u_have.iter().fold(0u32, |acc, n| {
                if c.winning_nos.contains(n) {
                    if acc == 0 {
                        1
                    } else {
                        acc * 2
                    }
                } else {
                    acc
                }
            });

            winning
        })
        .sum::<u32>();

    println!("{:?}", sum);
}

fn read_lines(filename: &str) -> Vec<Card> {
    return read_to_string(filename)
        .unwrap()
        .lines()
        .map(|l| l.parse::<Card>().unwrap())
        .collect();
}
