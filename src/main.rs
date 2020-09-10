use anyhow::Result;
use clap::{crate_authors, crate_version};
use clap::{App, Arg, ArgMatches};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<()> {
    // Get arguments
    let matches = match_arguments();
    let mut output_path = unwrap_matcher(&matches, "Output")?;
    let input_path = unwrap_matcher(&matches, "Input")?;

    // Check arguments
    anyhow::ensure!(output_path.is_dir(), "Output path must be a directory!");
    anyhow::ensure!(input_path.is_file(), "Input path must be a file!");

    // Create binary
    let mut binary = input_path.clone();
    binary.set_extension("bin");
    create_binary(&input_path, &binary)?;

    print_metadata(&binary)?;

    // Copy binary to the output location
    output_path.push(
        binary
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Binary does not have a name"))?,
    );
    fs::copy(&binary, output_path)?;

    Ok(())
}

fn match_arguments() -> ArgMatches<'static> {
    App::new("cargo-cpy-run")
        .version(crate_version!())
        .author(crate_authors!(",\n"))
        .about("Creates bin object and copies to specified location.")
        .arg(Arg::with_name("bin-name").hidden(true))
        .arg(
            Arg::with_name("Output")
                .short("l")
                .long("location")
                .help("Path for the final binary location")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("Input")
                .short("i")
                .long("input")
                .help("Compiled rust elf")
                .takes_value(true)
                .required(true),
        )
        .get_matches()
}

fn unwrap_matcher(matches: &ArgMatches<'static>, name: &'static str) -> Result<PathBuf> {
    Ok(matches
        .value_of(name)
        .ok_or_else(|| anyhow::anyhow!("{} is empty", name))?
        .into())
}

fn create_binary(input: &PathBuf, binary: &PathBuf) -> Result<()> {
    let result = Command::new("rust-objcopy")
        .arg(input)
        .arg("-O")
        .arg("binary")
        .arg(&binary)
        .output()?;
    let stderr = String::from_utf8(result.stderr)?;
    anyhow::ensure!(result.status.success(), "Obj copy failed: {}", stderr);
    Ok(())
}

fn print_metadata(path: &PathBuf) -> Result<()> {
    let metadata = path.metadata()?;
    println!(
        "Final binary size: {}",
        bytesize::to_string(metadata.len(), true)
    );
    Ok(())
}
