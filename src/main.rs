use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {

    let file = File::open("test.txt");

    let file_reader = match file {
        Ok(file) => BufReader::new(file),
        Err(err) => panic!("[ ERROR ]: {}", err),
    };

    let mut forest: Vec<String> = Vec::new();

    for line in file_reader.lines() {
        match line {
            Ok(line) => {
                forest.push(line);
            }
            Err(err) => {
                println!("[ ERROR ]: {}", err);
            }
        }
    }
}
