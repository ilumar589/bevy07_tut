use std::fs;
use std::error::Error;


pub fn read_file_contents(file_path: &str) -> String {
    fs::read_to_string(file_path).unwrap()
}