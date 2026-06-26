use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

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

fn main() {

    let args = Options::parse();

    let file = File::open(&args.input);

    let file_reader = match file {
        Ok(file) => BufReader::new(file),
        Err(err) => panic!("[ ERROR ]: {}", err),
    };

    let mut forest: Vec<String> = Vec::new();

    for line in file_reader.lines() {
        match line {
            Ok(line) => forest.push(line),
            Err(err) => println!("[ ERROR ]: {}", err),
        }
    }

    let mut log_size = args.lines;

    if log_size > forest.len() {
        log_size = forest.len();
    }

    let timber: Vec<&[String]> = forest
        .chunks(log_size)
        .collect();

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
        let period_index = match args.input.find(".") {
            Some(index) => index,
            None => args.input.len(),
        };
        // The output name is everything up to the period index
        let output_name = &args.input[..period_index];
        // The output extension is everything after and including the period index
        let output_extension = &args.input[period_index..];
        // Format the output name to include the original file name, the included line range, and
        // original extension if there was one
        let output_file = format!("{}_{}-{}{}", output_name, i, j, output_extension);
        // Create the output file
        let mut final_output = File::create(&output_file).expect("Failed to Create Output File");
        // Write each line in the chunk to the proper output file
        for ring in log {
            let _ = writeln!(final_output, "{}", ring);
        }
        // Set i to j so the next loop starts where this one ended
        i = j
    }
}
