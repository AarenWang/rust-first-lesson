use std::path::PathBuf;
use std::fs::File;
use super::Error;
use std::io::prelude::*;

pub fn load_csv(csv_file: PathBuf) -> Result<String,Error>{
    let file = read(csv_file);
    Ok(file)
}



