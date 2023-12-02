use std::fs::read_to_string;

fn main() {
    let sum: u32 = read_lines("./data/day2-input.txt")
        .iter()
        .filter_map(|s| {
            let (day, input) = s.split_once(":").unwrap();

            let (_, day_no) = day.split_once(" ").unwrap();
            let sets = input.split(";");

            for s in sets {
                match s.trim().split(",").find(|x| {
                    let (count, color) = x.trim().split_once(" ").unwrap();
                    let count_num = count.parse::<usize>().unwrap();

                    let exceeded = match color {
                        "red" => count_num > 12,
                        "green" => count_num > 13,
                        "blue" => count_num > 14,
                        _ => true,
                    };

                    return exceeded;
                }) {
                    Some(_) => {
                        return None;
                    }
                    None => {}
                }
            }

            return Some(day_no.trim().parse::<u32>().unwrap());
        })
        .sum();

    println!("Sum: {:?}", sum);
}

fn read_lines(filename: &str) -> Vec<String> {
    return read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
}
