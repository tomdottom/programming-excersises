use std::io::{self, BufRead, BufReader, Read};
use std::num;

#[derive(Debug)]
enum CliError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CliError {
    fn from(error: io::Error) -> Self {
        CliError::IoError(error)
    }
}

impl From<num::ParseIntError> for CliError {
    fn from(error: num::ParseIntError) -> Self {
        CliError::ParseError(error)
    }
}

fn parse_u64(line: Result<String, io::Error>) -> Result<u64, CliError> {
    match line {
        Ok(a) => {
            a.parse::<u64>()
             .map_err(|e| e.into())
        },
        Err(e) => Err(e.into())
    }
}

#[derive(Debug, PartialEq)]
struct Inputs {
    t: u64,
    n: Vec<u64>,
}

fn parse_input<I>(buf: I) -> Result<Inputs, CliError>
where
    I: Read
{
    let mut reader = BufReader::new(buf);

    let t: u64 = reader
        .by_ref().lines()
        .take(1)
        .map(parse_u64)
        .collect::<Result<Vec<u64>, CliError>>()?
        .remove(0);

    let n: Result<Vec<u64>, CliError> = reader
        .by_ref().lines()
        .take(t as usize)
        .map(parse_u64)
        .collect();

    match n {
        Ok(a) => Ok(Inputs{t: t, n: a}),
        Err(e) => Err(e)
    }
}

fn factorials(n: u64) -> Vec<u64> {
    let mut tmp;
    let mut a = 1;
    let mut b = 1;
    let mut res: Vec<u64> = vec![];
    while b < n {
        if b % 2 == 0 {
            res.push(b);
        }
        tmp = b;
        b = a + b;
        a = tmp;
    }
    res
}

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();

    let input = parse_input(stdin_lock);
    match input {
        Ok(i) => {
            for n in i.n {
                let sum: u64 = factorials(n).iter().sum();
                println!("{:?}", sum);
            }
        },
        Err(_) => println!("Encountered an error"),
    }
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn parses_sample_input() {
        println!("");
        let input = concat!(
            "2\n",
            "10\n",
            "100\n"
        );
        let expected = Inputs {
            t: 2,
            n: vec![10, 100],
        };

        let buf = io::Cursor::new(input);

        let res = parse_input(buf);
        assert_eq!(
            res.unwrap(),
            expected,
        )
    }

    #[test]
    fn test_factorials() {
        assert_eq!(
            factorials(10),
            vec![2, 8]
        );
        assert_eq!(
            factorials(100),
            vec![2, 8, 34]
        );
    }
}
