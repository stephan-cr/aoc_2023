use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);

    let mut line = String::new();
    let mut sum = 0;
    while let Ok(n) = buf_reader.read_line(&mut line) {
        if n == 0 {
            break;
        }

        eprint!("{line}");

        line.clear();
    }

    Ok(())
}
