use std::fs::File;
use std::io::{BufRead, BufReader};

use structopt::StructOpt;

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
        Ok(file) => get_lines_containing(&args.pattern, &mut BufReader::new(file)),
        Err(error) => format!("File error: {}", error)
    };

    println!("{}", message)
}

fn get_lines_containing(pattern: &String, reader: &mut dyn BufRead) -> String {
    let matching_strings: Vec<String> = reader.lines()
        .map(Result::unwrap)
        .filter(|line| line.contains(pattern))
        .collect();

    matching_strings.join("\n")
}

#[cfg(test)]
mod tests {
    use std::io;

    use super::*;

    #[test]
    fn empty_file() {
        let mut reader = io::Cursor::new(b"");
        let lines_containing = get_lines_containing(&String::from(""), &mut reader);
        assert_eq!(lines_containing, "");
    }

    #[test]
    fn std() {
        let mut reader = io::Cursor::new(b"line1 coucou\nline2 bibu");
        let lines_containing = get_lines_containing(&String::from("line1"), &mut reader);
        assert_eq!(lines_containing, "line1 coucou");
    }
}
