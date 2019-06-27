use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

/// This takes a filename and reads the entire file to a string
pub fn read_file_as_string(file_name: &str) -> std::io::Result<String>{
    let file = File::open(file_name)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}
