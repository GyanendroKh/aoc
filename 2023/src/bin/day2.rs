use std::fs::read_to_string;

#[derive(Debug)]
struct PowerCount {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let sum: u32 = read_lines("./data/day2-input.txt")
        .iter()
        .map(|s| {
            let (_, input) = s.split_once(":").unwrap();

            let power = input.split(";").fold(
                PowerCount {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |mut acc, s| {
                    s.trim().split(",").for_each(|x| {
                        let (count, color) = x.trim().split_once(" ").unwrap();
                        let count_num = count.parse::<u32>().unwrap();

                        match color {
                            "red" => {
                                if acc.red < count_num {
                                    acc.red = count_num;
                                }
                            }
                            "green" => {
                                if acc.green < count_num {
                                    acc.green = count_num;
                                }
                            }
                            "blue" => {
                                if acc.blue < count_num {
                                    acc.blue = count_num;
                                }
                            }
                            _ => {}
                        };
                    });

                    return acc;
                },
            );

            return power.blue * power.green * power.red;
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
