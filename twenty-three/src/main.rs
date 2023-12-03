use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use error::Error;

mod error;

fn main() -> Result<(), Error> {
    let mut total: u32 = 0;
    if let Ok(lines) = read_lines("./res/day-one/part-one/input") {
        for line in lines {
            if let Ok(string) = line {
                println!("{}", string);
                let chars = string.chars();
                let reversed = chars.clone().rev();
                let left = linear_search_for_int(chars);
                let right = linear_search_for_int(reversed);
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

fn linear_search_for_int(chars: impl IntoIterator<Item = char>) -> Option<u32> {
    let mut maybe_int = None;
    for char in chars {
        if let Some(int) = char.to_digit(10) {
            println!("{}: {}", char, int);
            maybe_int = Some(int);
            break;
        }
        println!("{}", char);
    }
    maybe_int
}

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
