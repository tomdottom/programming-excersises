use std::io;
use std::io::prelude::*;
use std::process;

type Matrix = Vec<Vec<i32>>;

fn get_lines(n: usize) -> Vec<String> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .take(n)
        .filter_map(|line| line.ok())
        .collect();

    lines
}

fn parse_input() -> Result<Matrix, String> {
    let mut size = get_lines(1);
    // let size: usize = size.remove(0).parse().unwrap();
    let size: usize = match size.remove(0).parse() {
        Ok(size) => size,
        Err(err) => {
            let msg = format!("Foo {:?}", err);
            return Err(msg)
        }
    };

    let lines = get_lines(size);
    if lines.len() != size {
        println!("Recieve unexpected number of rows: {:?}", lines);
        return Err(format!("Error in input"))
    }

    let mut matrix: Matrix = vec![vec![0; size]; size];
    for (i, line) in lines.iter().enumerate() {
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|line| line.parse().unwrap())
            .collect();

        if row.len() != size {
            println!("Recieve unexpected number of columns in line {:?}", row);
            return Err(format!("Error in input"))
        }
        matrix[i] = row;
    }

    return Ok(matrix)
}

fn diagonal_difference(matrix: Matrix) -> i32 {
    let size: usize = matrix.len();
    let diagonal_1: Vec<i32> = (0..size)
        .map(|i| matrix[i][i])
        .collect();
    let sum_1: i32 =  diagonal_1.iter().sum();
    let diagonal_2: Vec<i32> = (0..size)
        .map(|i| matrix[size - 1 - i][i])
        .collect();
    let sum_2: i32 =  diagonal_2.iter().sum();

    (sum_1 - sum_2).abs()
}

fn main() {
    match parse_input() {
        Ok(matrix) => println!("{:?}", diagonal_difference(matrix)),
        Err(err) => {
            println!("{:?}", err);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {

    }
}