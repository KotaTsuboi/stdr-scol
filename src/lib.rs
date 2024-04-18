use anyhow::Context;
use anyhow::Result;
use clap::Parser;
use std::path::Path;

mod args;
mod input;
mod output;
mod output_util;

pub fn run() -> Result<()> {
    let args = args::Args::parse();
    let input = input::read_input(&args.input_file)?;
    let output_file = remove_extension(&args.input_file)?;
    let output_file = format!("{output_file}.dxf");
    output::write(input, &output_file)?;
    Ok(())
}

fn remove_extension(file_name_with_extension: &str) -> Result<&str> {
    let stem = Path::new(file_name_with_extension)
        .file_stem()
        .context("Cannot get file stem")?
        .to_str()
        .context("Cannot convert to str")?;

    Ok(stem)
}
