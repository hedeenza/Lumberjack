use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Command, ExitCode};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Options {
    
    /// The input file
    #[arg(short, long)]
    input: String,

    /// The number of lines per chunk
    #[arg(short, long)]
    lines: usize,

}

fn main() -> ExitCode {
    // Parse the command line arguments
    let args = Options::parse();

    // Read in the input file
    let file = File::open(&args.input);

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

    // Determine the number of lines to include in each file
    let mut log_size = args.lines;

    // If the number is larger than the number of lines in the original file,
    // set it to the number of lines in the original file
    if log_size > forest.len() {
        log_size = forest.len();
    }

    // Break the vector into chunks of the desired size
    let timber: Vec<&[String]> = forest
        .chunks(log_size)
        .collect();

    // Create a directory to hold the output
    // Find the position of the period in the input file name, if there is one
    let period_index = match args.input.find(".") {
        Some(index) => index,
        None => args.input.len(),
    };
    // The output name is everything up to the period index
    let input_name = &args.input[..period_index];

    // Destination directory includes the input file name for ease of location
    let mill = String::from("./") + input_name + &String::from("_divided/");
    
    // If the directory already exists, do nothing
    if Path::new(&mill).exists() {
        ()
    // If the directory does not exist, create it and 
    // wait for creation to finish before moving on
    } else {
        let mut create_directory = Command::new("mkdir")
            .arg(&mill)
            .spawn().
            expect("Could not create output directory");
        let _child = create_directory
            .wait()
            .expect("Could not wait for directory creation");
    }

    // Initialize line counter
    let mut i = 0;

    for log in timber {
        // Shift 0-index to a more human-sensible index
        i += 1;
        // Create a second value, j, equal to i
        let mut j = i;
        // Add the lengh of the log to j, subtract 1 so it equals the line value
        j += log.len() - 1;
        // The output extension is everything after and including the period index
        let input_extension = &args.input[period_index..];
        // Format the output name to include the original file name, the included line range, and
        // original extension if there was one
        let output_file = format!("{}_{}-{}{}", input_name, i, j, input_extension);
        // Create the output file
        let mut final_output = File::create(mill.clone() + &output_file).expect("Failed to Create Output File");
        // Write each line in the chunk to the proper output file
        for ring in log {
            let _ = writeln!(final_output, "{}", ring);
        }
        // Set i to j so the next loop starts where this one ended
        i = j
    }

    ExitCode::from(0)
}
