use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use error::Error;

mod error;
mod parser;

fn main() -> Result<(), Error> {
    let mut total: u32 = 0;
    if let Ok(lines) = read_lines("./res/day-one/part-one/input") {
        for line in lines {
            if let Ok(string) = line {
                println!("{}", string);
                let chars = &mut string.chars();
                let reversed = &mut chars.clone().rev().peekable();
                let left = linear_search(&mut chars.peekable());
                let right = linear_search(reversed);
                let sum = concat(left, right)?;
                dbg!("left: {}", left);
                dbg!("right: {}", right);
                println!("{}", sum);
                total += sum;
            }
        }
    }
    println!("total: {}", total);
    Ok(())
}

fn concat(left: Option<u32>, right: Option<u32>) -> Result<u32, Error> {
    match (left, right) {
        (Some(l), Some(r)) => format!("{}{}", l, r).parse::<u32>().map_err(Into::into),
        _ => Err(Error::NoNumberFoundOnLine),
    }
}

fn linear_search<Iter: Iterator<Item = char>>(
    chars: &mut std::iter::Peekable<Iter>,
) -> Option<u32> {
    let next = chars.peek();
    match next {
        Some(char) => match char.to_digit(10) {
            Some(int) => {
                chars.next();
                Some(int)
            }
            None => match parser::wti::parse(chars) {
                parser::wti::Int::Hit(int) => Some(int),
                parser::wti::Int::Miss => linear_search(chars),
            },
        },
        None => None,
    }
}

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
