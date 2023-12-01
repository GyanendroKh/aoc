use std::fs::read_to_string;

fn main() {
    let lines = read_lines("./data/day1-input.txt");

    let numbers: u32 = lines
        .iter()
        .map(|line| {
            let digits: Vec<char> = line
                .replace("one", "on1e")
                .replace("two", "tw2o")
                .replace("three", "thre3e")
                .replace("four", "fou4r")
                .replace("five", "fiv5e")
                .replace("six", "si6x")
                .replace("seven", "seve7n")
                .replace("eight", "eigh8t")
                .replace("nine", "nin9e")
                .chars()
                .filter_map(|c| {
                    return if c.is_numeric() { Some(c) } else { None };
                })
                .collect();

            format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
                .parse::<u32>()
                .unwrap()
        })
        .sum();

    println!("{numbers}");
}

fn read_lines(filename: &str) -> Vec<String> {
    return read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}
