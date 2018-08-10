use std::io;
use std::io::prelude::*;
use std::num::ParseIntError;


fn parse_input() -> Result<(i32, Vec<i32>), ParseIntError> {
    let stdin = io::stdin();
    let mut lines: Vec<String> = stdin
        .lock()
        .lines()
        .take(2)
        .filter_map(|line| line.ok())
        .collect();

    let count: i32 = lines.remove(0).parse()?;
    let items: Vec<i32> = lines.remove(0).split_whitespace().map(|i| {
        i.parse().unwrap()
    }).collect();

    Ok((count, items))
}


fn main() {
    match parse_input() {
        Ok((x, y)) => {
            let res: i32 = y.iter().sum();
            println!("{}", res);
        },
        Err(_) => println!("Unable to parse inputs"),
    }
}
