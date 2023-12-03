use std::fs::File;
use std::io::{BufRead, BufReader};

// wrong answers:
//
// 3974

#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct Draw {
    blue: u32,
    green: u32,
    red: u32,
}

impl Draw {
    fn any_element_greater_than(&self, other: &Self) -> bool {
        self.blue > other.blue || self.green > other.green || self.red > other.red
    }

    fn max(&self, other: &Self) -> Self {
        Draw {
            blue: self.blue.max(other.blue),
            green: self.green.max(other.green),
            red: self.red.max(other.red),
        }
    }

    fn power(&self) -> u32 {
        self.blue * self.green * self.red
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);

    let max_draw = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut line = String::new();
    let mut sum = 0;
    let mut sum_of_powers = 0;
    while let Ok(n) = buf_reader.read_line(&mut line) {
        if n == 0 {
            break;
        }

        line.pop();
        if let Some((game, rest)) = line.split_once(": ") {
            let mut game_nr: u32 = 0;
            if let Some((_, game_nr_str)) = game.split_once(' ') {
                game_nr = u32::from_str_radix(game_nr_str, 10).unwrap();
            }

            let mut draws: Vec<Draw> = Vec::new();
            for draw in rest.split("; ") {
                let mut d = Draw {
                    blue: 0,
                    green: 0,
                    red: 0,
                };
                for color in draw.split(", ") {
                    if let Some((count, color_name)) = color.split_once(' ') {
                        let count = u32::from_str_radix(count, 10).unwrap();
                        match color_name {
                            "blue" => d.blue += count,
                            "green" => d.green += count,
                            "red" => d.red += count,
                            _ => unreachable!(),
                        }
                    }
                }

                draws.push(d);
            }

            let d = Draw {
                blue: 0,
                green: 0,
                red: 0,
            };
            let result = draws.iter().fold(d, |acc, e| acc.max(e));

            eprintln!("{result:?}");
            sum_of_powers += result.power();

            if draws
                .iter()
                .fold(true, |acc, e| acc && !e.any_element_greater_than(&max_draw))
            {
                sum += game_nr;
            }
        }

        line.clear();
    }

    println!("sum of valid game IDs: {sum}");
    println!("sum of power of fewest number of cubes: {sum_of_powers}");

    assert_eq!(sum_of_powers, 54911);

    Ok(())
}
