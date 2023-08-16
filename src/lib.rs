use clap::Parser;
use std::error::Error;

mod args;
mod input;
mod output;
mod output_util;

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = args::Args::parse();
    let input = input::read_input(&args.input_file)?;
    output::write(input, &args.output_file)?;
    Ok(())
}
