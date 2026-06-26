use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

pub fn read_to_vec(input_file: &str) -> Vec<String> {
    // Read in the input file
    let file = File::open(input_file);
    let file_reader = match file {
        Ok(file) => BufReader::new(file),
        Err(err) => panic!("[ ERROR ]: {}", err),
    };

    // Create a vector to contain the original file contents
    let mut forest: Vec<String> = Vec::new();

    // Push each line to the vector
    for line in file_reader.lines() {
        match line {
            Ok(line) => forest.push(line),
            Err(err) => println!("[ ERROR ]: {}", err),
        }
    }
    forest
}

pub fn size_forest<'a>(input_size: &'a usize, forest: &'a [String]) -> Vec<&'a [String]> {
    // Determine the number of lines to include in each file
    let mut log_size = input_size;

    // If the number is larger than the number of lines in the original file,
    // set it to the number of lines in the original file
    let maximum_size = forest.len();
    if *log_size > maximum_size {
        log_size = &maximum_size;
    }

    // Break the vector into chunks of the desired size
    let timber: Vec<&[String]> = forest.chunks(*log_size).collect();

    timber
}

pub fn manage_destination(input_file: &str) -> String {
    // Create a directory to hold the output
    // Find the position of the period in the input file name, if there is one
    let period_index = match input_file.find(".") {
        Some(index) => index,
        None => input_file.len(),
    };
    // The output name is everything up to the period index
    let input_name = &input_file[..period_index];

    // Destination directory includes the input file name for ease of location
    let mill = String::from("./") + input_name + &String::from("_chopped/");

    // If the directory already exists, do nothing
    if Path::new(&mill).exists() {
        // If the directory does not exist, create it and
        // wait for creation to finish before moving on
    } else {
        let mut create_directory = Command::new("mkdir")
            .arg(&mill)
            .spawn()
            .expect("Could not create output directory");
        let _child = create_directory
            .wait()
            .expect("Could not wait for directory creation");
    }
    mill
}

pub fn chop_lumber(input_file: &str, timber: Vec<&[String]>, mill: String) {
    // Initialize line counter
    let mut i = 0;

    for log in timber {
        // Shift 0-index to a more human-sensible index
        i += 1;
        // Create a second value, j, equal to i
        let mut j = i;
        // Add the lengh of the log to j, subtract 1 so it equals the line value
        j += log.len() - 1;
        // Find the position of the period in the input file name, if there is one
        let period_index = match input_file.find(".") {
            Some(index) => index,
            None => input_file.len(),
        };
        // The output name is everything up to the period index
        let input_name = &input_file[..period_index];
        // The output extension is everything after and including the period index
        let input_extension = &input_file[period_index..];
        // Format the output name to include the original file name, the included line range, and
        // original extension if there was one
        let output_file = format!("{}_{}-{}{}", input_name, i, j, input_extension);
        // Create the output file
        let mut final_output =
            File::create(mill.clone() + &output_file).expect("Failed to Create Output File");
        // Write each line in the chunk to the proper output file
        for ring in log {
            let _ = writeln!(final_output, "{}", ring);
        }
        // Set i to j so the next loop starts where this one ended
        i = j
    }
}
