use std::io;
use std::io::prelude::*;
use std::num::ParseIntError;


fn parse_input() -> Result<(i32, i32), ParseIntError> {
    let stdin = io::stdin();
    let mut lines: Vec<String> = stdin
        .lock()
        .lines()
        .take(2)
        .filter_map(|line| line.ok())
        .collect();

    let x: i32 = lines.remove(0).parse()?;
    let y: i32 = lines.remove(0).parse()?;

    Ok((x, y))
}


fn main() {
    match parse_input() {
        Ok((x, y)) => println!("{}", x + y),
        Err(_) => println!("Unable to parse inputs"),
    }
}
