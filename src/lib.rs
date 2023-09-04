use clap::Parser;
use std::error::Error;
use std::path::Path;

mod args;
mod input;
mod output;
mod output_util;

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = args::Args::parse();

    let input_file = Path::new(&args.input_file);
    let output_file = input_file.with_extension("dxf");

    let input = input::read_input(input_file.to_str().unwrap())?;

    output::write(input, output_file.to_str().unwrap())?;

    Ok(())
}
