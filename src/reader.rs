use std::fs;
use std::io;

pub fn read_file(path: &String) -> Result<String, io::Error> {
    fs::read_to_string(path)
}