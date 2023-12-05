use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let sections = read_lines("./data/day5-input.txt");

    let seeds = sections
        .get(0)
        .and_then(|l| {
            l.split_once(":").and_then(|(_, s)| {
                Some(
                    s.trim()
                        .split(" ")
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>(),
                )
            })
        })
        .unwrap();

    let mut all_mappings: HashMap<(&str, &str), Vec<(u32, u32, u32)>> = HashMap::new();

    sections[1..].iter().for_each(|l| {
        let lines = l.trim().split("\n").collect::<Vec<&str>>();

        let mapping_key = lines
            .get(0)
            .unwrap()
            .split_once(" ")
            .and_then(|(m, _)| m.split_once("-to-"))
            .unwrap();

        let mut mapping: Vec<(u32, u32, u32)> = Vec::new();

        for &l in lines[1..].iter() {
            let [dest, source, n] = l
                .splitn(3, " ")
                .map(|n| n.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()[..]
            else {
                todo!("Unreachable");
            };

            mapping.push((source, dest, n));
        }

        all_mappings.insert(mapping_key, mapping);
    });

    let mut key_mappings: HashMap<&str, &str> = HashMap::new();

    for (k, v) in all_mappings.keys() {
        key_mappings.insert(k, v);
    }

    let min = seeds
        .iter()
        .map(|s| decode_mapping(s, "seed", "location", &key_mappings, &all_mappings).unwrap())
        .min()
        .unwrap();

    println!("Part 1: {}", min);
}

fn decode_mapping(
    num: &u32,
    start: &str,
    end: &str,
    key_mappings: &HashMap<&str, &str>,
    all_mappings: &HashMap<(&str, &str), Vec<(u32, u32, u32)>>,
) -> Result<u32, String> {
    let &mapping = key_mappings.get(start).unwrap();
    let values = all_mappings.get(&(start, mapping)).unwrap();

    match &values.iter().find_map(|(s, d, n)| {
        if num >= s && num <= &(s + n) {
            return Some(d + &(num - s));
        }

        return None;
    }) {
        Some(value) => {
            if mapping.eq(end) {
                return Ok(value.clone());
            } else {
                return decode_mapping(value, mapping, end, key_mappings, all_mappings);
            }
        }
        None => {
            if mapping.eq(end) {
                return Ok(num.clone());
            } else {
                return decode_mapping(num, mapping, end, key_mappings, all_mappings);
            }
        }
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    return read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
}
