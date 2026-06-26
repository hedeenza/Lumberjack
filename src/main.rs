use clap::Parser;
use std::process::ExitCode;
use lumberjack::{read_to_vec, size_forest, manage_destination, chop_lumber};

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

    // Read input, collect lines to vector
    let forest = read_to_vec(&args.input);

    // Chunk vector into desired size
    let timber = size_forest(&args.lines, &forest);

    // Create output directory if it does not exist
    let mill = manage_destination(&args.input);

    chop_lumber(&args.input, timber, mill);

    ExitCode::from(0)
}
