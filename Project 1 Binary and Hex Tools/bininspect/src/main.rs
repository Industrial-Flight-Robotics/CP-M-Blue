use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print_usage(&args[0]);
        return ExitCode::FAILURE;
    }

    let filename = &args[1];

    match read_binary_file(filename) {
        Ok(data) => {
            println!("File: {}", filename);
            println!("Size: {} bytes", data.len());

            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Error reading '{}': {}", filename, error);
            ExitCode::FAILURE
        }
    }
}

fn read_binary_file<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<u8>> {
    fs::read(path)
}

fn print_usage(program_name: &str) {
    eprintln!("Usage:");
    eprintln!("  {} <binary-file>", program_name);
}