use clap::Parser;
use lumberjack::{chop_lumber, manage_destination, paragraph_forest, read_to_vec, size_forest, split_lumber};
use std::process::ExitCode;
use std::time::Instant;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Options {
    /// The input file
    #[arg(short, long)]
    input: String,

    /// The number of lines per chunk [default = 1]
    #[arg(short, long, default_value_t = 1)]
    lines: usize,

    /// Use paragraph mode to divide by paragraph breaks rather than line count
    #[arg(short, long)]
    paragraph: bool,
}

fn main() -> ExitCode {
    // Start Benchmarking Timer
    let shift_start = Instant::now();

    // Parse the command line arguments
    let args = Options::parse();

    // Create output directory if it does not exist
    let mill = manage_destination(&args.input);

    // Read input, collect lines to vector
    let mut forest = read_to_vec(&args.input);

    // Chunk vector into desired size
    if args.paragraph {
        let timber = paragraph_forest(&mut forest);
        split_lumber(&args.input, timber.clone(), mill);
        // Finish Benchmarking Timer
        let shift_duration = shift_start.elapsed();
        println!(
            "Chopped {} into {} Logs in {:.2?}",
            &args.input,
            timber.len(),
            shift_duration
        );
    } else {
        let timber = size_forest(&args.lines, &forest);
        chop_lumber(&args.input, timber.clone(), mill);
        // Finish Benchmarking Timer
        let shift_duration = shift_start.elapsed();
        println!(
            "Chopped {} into {} Logs in {:.2?}",
            &args.input,
            timber.len(),
            shift_duration
        );
    }

    ExitCode::from(0)
}
