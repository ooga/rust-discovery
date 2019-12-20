use structopt::StructOpt;
use std::io::{BufReader, BufRead};
use std::fs::File;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let file_result = File::open(args.path);

    let message = match file_result {
        Ok(file) => print_lines_containing(&args.pattern, file),
        Err(error) => format!("File error: {}", error)
    };

    println!("{}", message)
}

fn print_lines_containing(pattern: &String, file: File) -> String {
    let reader = BufReader::new(file);
    let strings: Vec<String> = reader.lines()
        .map(|line_result| line_result.unwrap())
        .filter(|line| line.contains(pattern))
        .collect();

    strings.join("\n")
}
