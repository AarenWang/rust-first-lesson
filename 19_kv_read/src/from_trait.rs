use std::fs;
use std::io;
use std::num;

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

fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
    let mut contents = fs::read_to_string(&file_name)?;
    let num: i32 = contents.trim().parse()?;
    Ok(num)
}

fn main() {
    let file1 = "1.txt";
    let file2 = "2.txt";
    let n1 = open_and_parse_file(file1).unwrap_or_default();
    let n2 = open_and_parse_file(file2).unwrap_or_default();
    println!("file {}  result is {}", file1, n1);
    println!("file {} result is {}", file1, n2);
}
