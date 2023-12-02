use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// wrong answers:
//
// 54609
// 53519
// 53516

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);

    let long_digit_to_digit = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut line = String::new();
    let mut sum = 0;
    while let Ok(n) = buf_reader.read_line(&mut line) {
        if n == 0 {
            break;
        }

        eprint!("o: {line}");

        let mut forward_line = line.clone();
        loop {
            let searches = [
                forward_line
                    .match_indices("one")
                    .take(1)
                    .collect::<Vec<_>>(),
                forward_line.match_indices("two").take(1).collect(),
                forward_line.match_indices("three").take(1).collect(),
                forward_line.match_indices("four").take(1).collect(),
                forward_line.match_indices("five").take(1).collect(),
                forward_line.match_indices("six").take(1).collect(),
                forward_line.match_indices("seven").take(1).collect(),
                forward_line.match_indices("eight").take(1).collect(),
                forward_line.match_indices("nine").take(1).collect(),
            ];

            let min_match = searches.iter().flatten().min_by_key(|&x| x.0);

            if let Some(&(pos, text)) = min_match {
                forward_line.replace_range(
                    pos..(pos + text.len()),
                    &long_digit_to_digit.get(&text).unwrap().to_string(),
                );
            } else {
                break;
            }
        }

        let mut backward_line = line.clone();
        loop {
            let searches = [
                backward_line
                    .rmatch_indices("one")
                    .take(1)
                    .collect::<Vec<_>>(),
                backward_line.rmatch_indices("two").take(1).collect(),
                backward_line.rmatch_indices("three").take(1).collect(),
                backward_line.rmatch_indices("four").take(1).collect(),
                backward_line.rmatch_indices("five").take(1).collect(),
                backward_line.rmatch_indices("six").take(1).collect(),
                backward_line.rmatch_indices("seven").take(1).collect(),
                backward_line.rmatch_indices("eight").take(1).collect(),
                backward_line.rmatch_indices("nine").take(1).collect(),
            ];

            let min_match = searches.iter().flatten().max_by_key(|&x| x.0);

            if let Some(&(pos, text)) = min_match {
                backward_line.replace_range(
                    pos..(pos + text.len()),
                    &long_digit_to_digit.get(&text).unwrap().to_string(),
                );
            } else {
                break;
            }
        }

        let number1 = if let Some(digit_index) = forward_line.find(|c: char| c.is_digit(10)) {
            char::from(forward_line.as_bytes()[digit_index]).to_digit(10)
        } else {
            None
        };

        let number2 = if let Some(digit_index) = backward_line.rfind(|c: char| c.is_digit(10)) {
            char::from(backward_line.as_bytes()[digit_index]).to_digit(10)
        } else {
            None
        };

        let num = number1.unwrap() * 10 + number2.unwrap();

        eprintln!("m: {forward_line}{backward_line}{num}");

        sum += num;

        line.clear();
    }

    println!("{sum}");

    Ok(())
}
